use crate::tokenizer::{
    Decoder, Encoding, NormalizedString, Normalizer, Offsets, PostProcessor, PreTokenizer, Result,
};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use unicode_categories::UnicodeCategories;

fn bytes_char() -> HashMap<u8, char> {
    let mut bs: Vec<u8> = vec![];
    bs.extend(b'!'..=b'~');
    bs.extend(b'\xA1'..=b'\xAC');
    bs.extend(b'\xAE'..=b'\xFF');

    let mut cs: Vec<u32> = bs.iter().map(|i| *i as u32).collect();
    let mut n = 0;

    for b in 0..=255u8 {
        if !bs.contains(&b) {
            bs.push(b);
            cs.push(u32::pow(2, 8) + n);
            n += 1;
        }
    }

    bs.into_iter()
        .zip(cs)
        .map(|(f, t)| (f, unsafe { std::char::from_u32_unchecked(t) }))
        .collect()
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+").unwrap();
    static ref BYTES_CHAR: HashMap<u8, char> = bytes_char();
    static ref CHAR_BYTES: HashMap<char, u8> =
        bytes_char().into_iter().map(|(c, b)| (b, c)).collect();
}

/// Provides all the necessary steps to handle the BPE tokenization at the byte-level. Takes care
/// of all the required processing steps to transform a UTF-8 string as needed before and after the
/// BPE model does its job.
pub struct ByteLevel {
    add_prefix_space: bool,
}
impl ByteLevel {
    pub fn new(add_prefix_space: bool) -> Self {
        ByteLevel { add_prefix_space }
    }

    pub fn alphabet() -> HashSet<char> {
        BYTES_CHAR.values().copied().collect()
    }
}

/// As a `Normalizer`, `ByteLevel` is in charge of adding a prefix space if needed. This prefix
/// space let's the BPE model treat all words the same way (A word at the beginning of a sentence
/// should be treated exactly like a word in the middle of a sentence).
impl Normalizer for ByteLevel {
    fn normalize(&self, normalized: &mut NormalizedString) -> Result<()> {
        if self.add_prefix_space && !normalized.get().starts_with(' ') {
            normalized.prepend(" ");
        }
        Ok(())
    }
}

/// As a `PreTokenizer`, `ByteLevel` is in charge of transforming all the unicode characters into
/// their byte-level counterpart. It also splits the input according to the configured regex.
// TODO: Give the ability to modify this regex
impl PreTokenizer for ByteLevel {
    fn pre_tokenize(&self, s: &str) -> Result<Vec<(String, Offsets)>> {
        let mut total_len = 0;
        Ok(RE
            .captures_iter(&s)
            .map(|capture| {
                let capture = capture.get(0).unwrap();
                let start = capture.start();
                let end = capture.end();

                // if our last character is a whitespace, followed by a non whitespace,
                // we don't want to return it
                let last = s[start..end].chars().last();
                let next = s[end..].chars().next();
                if let (Some(last), Some(next)) = (last, next) {
                    if last.is_separator_space() && !next.is_separator_space() {
                        if let Some((_last, others)) =
                            s[start..end].chars().collect::<Vec<_>>().split_last()
                        {
                            let bytes = others.iter().collect::<String>().as_bytes().to_vec();
                            return (bytes, others.len());
                        }
                    }
                }
                // if our first char is not a whitespace but the previous one was, we return
                // a whitespace before our match
                let prev = s[0..start].chars().last();
                let current = s[start..end].chars().next().map(|c| c.is_whitespace());
                if let (Some(prev), Some(current)) = (prev, current) {
                    if prev.is_separator_space() && !current {
                        let bytes =
                            [format!("{}", prev).as_bytes(), s[start..end].as_bytes()].concat();
                        let len = s[start..end].chars().count() + 1;
                        return (bytes, len);
                    }
                }

                (
                    s[start..end].as_bytes().to_vec(),
                    s[start..end].chars().count(),
                )
            })
            .map(|(s, len)| {
                total_len += len;
                (
                    s.iter().map(|b| BYTES_CHAR[b]).collect(),
                    (total_len - len, total_len),
                )
            })
            .collect())
    }
}

/// As a `Decoder`, `ByteLevel` is in charge of converting any byte-level characters to their
/// unicode counterpart, before merging everything back into a single String.
impl Decoder for ByteLevel {
    fn decode(&self, tokens: Vec<String>) -> Result<String> {
        Ok(String::from_utf8_lossy(
            &tokens
                .join("")
                .chars()
                .map(|c| CHAR_BYTES[&c])
                .collect::<Vec<_>>(),
        )
        .into_owned())
    }
}

/// As a `PostProcessor`, `ByteLevel` is in charge of fixing any offsets produced by the BPE Model
/// that end up wrong. This happens because the BPE model is not aware that some characters get
/// split up in multiple other characters. So it treats all of them as single characters and
/// then increase the offsets when merging them.
impl PostProcessor for ByteLevel {
    fn added_tokens(&self, _is_pair: bool) -> usize {
        0
    }

    fn process(&self, mut encoding: Encoding, pair_encoding: Option<Encoding>) -> Result<Encoding> {
        let process_offsets = |encoding: &mut Encoding| {
            let len = encoding.get_offsets().len();
            let mut i = 0;
            let mut from_i = None;
            while i < len {
                if i + 1 < len {
                    let offsets = encoding.get_offsets();
                    if offsets[i + 1] < offsets[i] {
                        let mut j = i;
                        while j > 0 && offsets[j] >= offsets[i + 1] {
                            j -= 1;
                        }
                        from_i = Some(j);
                    }
                }

                if let Some(from_i) = from_i.take() {
                    let offsets = encoding.get_offsets_mut();
                    for j in from_i + 1..=i {
                        offsets[j] = offsets[from_i];
                    }
                }

                i += 1;
            }
        };

        process_offsets(&mut encoding);
        let final_encoding = match pair_encoding {
            None => encoding,
            Some(mut pair) => {
                process_offsets(&mut pair);
                encoding.merge_with(pair);
                encoding
            }
        };

        Ok(final_encoding)
    }
}

#[cfg(test)]
mod tests {
    use super::ByteLevel;
    use crate::tokenizer::{
        Decoder, Encoding, NormalizedString, Normalizer, PostProcessor, PreTokenizer,
    };

    #[test]
    fn pre_tokenization() {
        let bytelevel = ByteLevel::new(false);
        assert_eq!(
            bytelevel
                .pre_tokenize("Hello my friend, how is your day going?")
                .unwrap(),
            vec![
                ("Hello".into(), (0, 5)),
                ("Ġmy".into(), (5, 8)),
                ("Ġfriend".into(), (8, 15)),
                (",".into(), (15, 16)),
                ("Ġhow".into(), (16, 20)),
                ("Ġis".into(), (20, 23)),
                ("Ġyour".into(), (23, 28)),
                ("Ġday".into(), (28, 32)),
                ("Ġgoing".into(), (32, 38)),
                ("?".into(), (38, 39))
            ]
        );
    }

    #[test]
    fn decoding() {
        let bytelevel = ByteLevel::new(false);
        assert_eq!(
            "Hello my friend, how is your day going?",
            bytelevel
                .decode(
                    vec![
                        "Hello", "Ġmy", "Ġfriend", ",", "Ġhow", "Ġis", "Ġyour", "Ġday", "Ġgoing",
                        "?"
                    ]
                    .into_iter()
                    .map(|s| s.into())
                    .collect::<Vec<String>>()
                )
                .unwrap()
        );
    }

    #[test]
    fn add_prefix_space() {
        let bytelevel = ByteLevel::new(true);
        for s in &[
            " Hello my friend, how is your day going?",
            "Hello my friend, how is your day going?",
        ] {
            let mut normalized = NormalizedString::from(s);
            bytelevel.normalize(&mut normalized).unwrap();
            assert_eq!(normalized.get(), " Hello my friend, how is your day going?");
            assert_eq!(
                bytelevel.pre_tokenize(normalized.get()).unwrap(),
                vec![
                    ("ĠHello".into(), (0, 6)),
                    ("Ġmy".into(), (6, 9)),
                    ("Ġfriend".into(), (9, 16)),
                    (",".into(), (16, 17)),
                    ("Ġhow".into(), (17, 21)),
                    ("Ġis".into(), (21, 24)),
                    ("Ġyour".into(), (24, 29)),
                    ("Ġday".into(), (29, 33)),
                    ("Ġgoing".into(), (33, 39)),
                    ("?".into(), (39, 40))
                ]
            );
        }
    }

    #[test]
    fn decode_works_on_separated_tokens() {
        let samples = vec![
            String::from(
                "A Nuskhuri abbreviation of იესუ ქრისტე ( iesu kriste ) \" Jesus Christ \"",
            ),
            String::from(
                "An equal number have descenders , like p or q in English \
                 : გ , დ , ე , ვ , კ , ლ , ჟ , ტ , უ , ფ , ღ , ყ , ც",
            ),
        ];

        let bytelevel = ByteLevel::new(false);
        for sample in samples {
            let pre_tokenized = bytelevel.pre_tokenize(&sample).unwrap();
            let separated_tokens = pre_tokenized
                .iter()
                .flat_map(|(token, _)| token.split("").map(|t| t.into()))
                .collect::<Vec<_>>();
            assert_eq!(sample, bytelevel.decode(separated_tokens).unwrap());
        }
    }

    #[test]
    fn handling_of_newlines() {
        let s = String::from("Hello there\nHello there");
        let bytelevel = ByteLevel::new(false);
        let p = bytelevel.pre_tokenize(&s).unwrap();

        assert_eq!(
            p,
            vec![
                ("Hello".into(), (0, 5)),
                ("Ġthere".into(), (5, 11)),
                ("Ċ".into(), (11, 12)),
                ("Hello".into(), (12, 17)),
                ("Ġthere".into(), (17, 23))
            ]
        );
    }

    #[test]
    fn handling_of_multiple_whitespaces() {
        let s = String::from("Hello there       dear");
        let bytelevel = ByteLevel::new(false);
        let p = bytelevel.pre_tokenize(&s).unwrap();

        assert_eq!(
            p,
            vec![
                ("Hello".into(), (0, 5)),
                ("Ġthere".into(), (5, 11)),
                ("ĠĠĠĠĠĠ".into(), (11, 17)),
                ("Ġdear".into(), (17, 22))
            ]
        );
    }

    #[test]
    fn offsets_when_char_split_up() {
        let s = String::from("i⭢j");
        let bytelevel = ByteLevel::new(false);
        let p = bytelevel.pre_tokenize(&s).unwrap();

        assert_eq!(
            p,
            vec![
                ("i".into(), (0, 1)),
                ("âŃ¢".into(), (1, 2)),
                ("j".into(), (2, 3)),
            ]
        );
    }

    #[test]
    fn processor_fixes_offsets() {
        let bad_encoding = Encoding::new(
            NormalizedString::from(""),
            vec![],
            vec![],
            vec![],
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (3, 4), (4, 5)],
            vec![],
            vec![],
            vec![],
        );
        let fixed_encoding = Encoding::new(
            NormalizedString::from(""),
            vec![],
            vec![],
            vec![],
            vec![(0, 1), (1, 2), (2, 3), (2, 3), (2, 3), (3, 4), (4, 5)],
            vec![],
            vec![],
            vec![],
        );

        let bytelevel = ByteLevel::new(false);

        let fixed_single = bytelevel.process(bad_encoding.clone(), None).unwrap();
        assert_eq!(fixed_single, fixed_encoding);

        let fixed_pair = bytelevel
            .process(bad_encoding.clone(), Some(bad_encoding))
            .unwrap();
        let mut fixed_encoding_pair = fixed_encoding.clone();
        fixed_encoding_pair.merge_with(fixed_encoding);
        assert_eq!(fixed_pair, fixed_encoding_pair);
    }
}
