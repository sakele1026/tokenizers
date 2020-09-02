use crate::models::unigram::lattice::Lattice;
use crate::tokenizer::{Model, Offsets, Result, Token};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use trie_rs::{Trie, TrieBuilder};

type TokenMap = HashMap<String, u32>;
type Vocab = Vec<String>;

#[derive(Serialize, Deserialize)]
pub struct Unigram {
    token_to_ids: TokenMap,
    vocab: Vocab,
    scores: Vec<f64>,
    #[serde(skip_serializing, skip_deserializing, default = "empty_trie")]
    trie: Trie<char>,
    min_score: f64,
}
impl std::fmt::Debug for Unigram {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BPE")
            .field("vocab", &self.vocab.len())
            .finish()
    }
}

fn empty_trie() -> Trie<char> {
    TrieBuilder::new().build()
}

static K_UNK_PENALTY: f64 = 10.0;
static UNK_ID: usize = 2;

impl Default for Unigram {
    fn default() -> Self {
        Self {
            token_to_ids: HashMap::new(),
            vocab: vec![],
            trie: empty_trie(),
            scores: vec![],
            min_score: 0.0,
        }
    }
}

impl Unigram {
    pub fn from(table: &[(String, f64)]) -> Self {
        let n = table.len();
        let mut vocab: Vec<String> = Vec::with_capacity(n);
        let mut scores: Vec<f64> = Vec::with_capacity(n);
        let mut token_to_ids: TokenMap = HashMap::new();
        let mut builder = TrieBuilder::new();
        for (id, (token, score)) in table.iter().enumerate() {
            vocab.push(token.to_string());
            scores.push(*score);
            token_to_ids.insert(token.to_string(), id as u32);
            let chars: Vec<char> = token.chars().collect();
            builder.push(chars);
        }
        let min_score = scores.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let trie = builder.build();

        Unigram {
            vocab,
            scores,
            token_to_ids,
            trie,
            min_score,
        }
    }

    pub fn len(&self) -> usize {
        self.vocab.len()
    }
    pub fn is_empty(&self) -> bool {
        self.vocab.is_empty()
    }

    pub fn populate_nodes(&self, lattice: &mut Lattice) {
        //TODO
        //  auto get_chars_length = [&lattice](int begin_pos, const char *end) {
        //   int pos = begin_pos;
        //   while (lattice->surface(pos) < end) ++pos;
        //   return pos - begin_pos;
        // };
        let unk_score = self.min_score - K_UNK_PENALTY;

        // const float unk_score = min_score() - kUnkPenalty;

        // const int len = lattice->size();
        let len = lattice.len();
        // const char *end = lattice->sentence() + lattice->utf8_size();

        // // +1 just in case.
        // std::vector<Darts::DoubleArray::result_pair_type> trie_results(
        //     trie_results_size_ + 1);

        for begin_pos in 0..len {
            let rest: Vec<char> = lattice.chars[begin_pos..].to_vec();
            let trie_results: Vec<Vec<char>> = self.trie.common_prefix_search(&rest);

            let mut has_single_node = false;

            for result in trie_results {
                // TODO score comes from proto id.
                let n = result.len();
                let tok: String = result.into_iter().collect();
                let id = *self.token_to_ids.get(&tok).unwrap();
                let score: f64 = self.scores[id as usize];
                lattice.insert(begin_pos, n, score);
                if !has_single_node && n == 1 {
                    has_single_node = true;
                }
            }

            if !has_single_node {
                lattice.insert_with_id(begin_pos, 1, unk_score, UNK_ID);
            }
        }
        // for (int begin_pos = 0; begin_pos < len; ++begin_pos) {
        //   const char *begin = lattice->surface(begin_pos);

        //   // Finds all pieces which are prefix of surface(begin_pos).
        //   const size_t num_nodes = trie_->commonPrefixSearch(
        //       begin, trie_results.data(), trie_results.size(),
        //       static_cast<int>(end - begin));
        //   CHECK_LT(num_nodes, trie_results.size());

        //   bool has_single_node = false;

        //   // Inserts pieces to the lattice.
        //   for (size_t k = 0; k < num_nodes; ++k) {
        //     const int length =
        //         get_chars_length(begin_pos, begin + trie_results[k].length);
        //     const int id = trie_results[k].value;
        //     if (IsUnusedInlined(id)) continue;
        //     Lattice::Node *node = lattice->Insert(begin_pos, length);
        //     node->id = id;  // the value of Trie stores vocab_id.
        //     // User defined symbol receives extra bonus to always be selected.
        //     node->score = IsUserDefinedInlined(id) ? (length * max_score_ - 0.1)
        //                                            : GetScoreInlined(id);
        //     if (!has_single_node && node->length == 1) {
        //       has_single_node = true;
        //     }
        //   }

        //   if (!has_single_node) {
        //     Lattice::Node *node = lattice->Insert(begin_pos, 1);
        //     node->id = unk_id_;  // add UNK node.
        //     node->score = unk_score;
        //   }
        // }
    }
    pub fn encode(&self, sentence: &str) -> Vec<String> {
        // let pretokenizer = Whitespace;
        // let mut input = NormalizedString::from(sentence);
        // let encoded = pretokenizer.pre_tokenize(&mut input)?;
        // self.tokenize(encoded)
        // TODO optimized version
        let mut lattice = Lattice::from(sentence);
        self.populate_nodes(&mut lattice);
        lattice.tokens()
    }

    fn load(path: &Path) -> Result<Unigram> {
        println!("Path {:?}", path);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let table: Vec<(String, f64)> = serde_json::from_reader(reader).unwrap();
        let u = Unigram::from(&table);
        Ok(u)
    }
}

#[typetag::serde]
impl Model for Unigram {
    fn get_vocab(&self) -> &HashMap<String, u32> {
        &self.token_to_ids
    }

    fn get_vocab_size(&self) -> usize {
        self.vocab.len()
    }

    fn tokenize(&self, sentence: Vec<(String, Offsets)>) -> Result<Vec<Token>> {
        // TODO offsets
        let mut results: Vec<Token> = Vec::with_capacity(sentence.len());
        let string = sentence
            .into_iter()
            .map(|(s, _)| s)
            .collect::<Vec<_>>()
            .concat();
        let tokens = self.encode(&string);
        let elts: Vec<Token> = tokens
            .iter()
            .enumerate()
            .map(|(word, string)| {
                let id = match self.token_to_ids.get(string) {
                    Some(id) => id,
                    None => {
                        println!("Vocab {:?}", self.vocab);
                        println!("String {:?} has no id", string);
                        &0
                    }
                };
                let offsets = (0, 0);
                Token::new(*id, string.to_string(), offsets, word as u32)
            })
            .collect();
        Ok(elts)
        // for (element, _) in sentence {
        //     let tokens = self.encode(&element);
        //     let elts: Vec<Token> = tokens
        //         .iter()
        //         .enumerate()
        //         .map(|(word, string)| {
        //             let id = match self.token_to_ids.get(string) {
        //                 Some(id) => id,
        //                 None => {
        //                     println!("Vocab {:?}", self.vocab);
        //                     println!("String {:?} has no id", string);
        //                     &0
        //                 }
        //             };
        //             let offsets = (0, 0);
        //             Token::new(*id, string.to_string(), offsets, word as u32)
        //         })
        //         .collect();
        //     results.extend(elts);
        // }
        // Ok(results)
    }

    fn token_to_id(&self, token: &str) -> Option<u32> {
        self.token_to_ids.get(token).copied()
    }

    fn id_to_token(&self, id: u32) -> Option<&str> {
        match self.vocab.get(id as usize) {
            Some(string) => Some(string),
            None => None,
        }
    }

    fn save(&self, _folder: &Path, _name: Option<&str>) -> Result<Vec<PathBuf>> {
        // TODO
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pre_tokenizers::whitespace::Whitespace;
    use crate::tokenizer::{NormalizedString, PreTokenizer};

    #[test]
    fn test_encode() {
        let sentencepieces = vec![
            ("a".to_string(), 0.0),
            ("b".to_string(), 0.0),
            ("c".to_string(), 0.0),
            ("d".to_string(), 0.0),
            ("cd".to_string(), 1.0),
            ("ab".to_string(), 2.0),
            ("abc".to_string(), 5.0),
            ("abcd".to_string(), 10.0),
        ];

        //TODO
        let model = Unigram::from(&sentencepieces);
        let result = model.encode("abcd");
        assert_eq!(result, vec!["abcd"]);
    }

    #[test]
    fn test_encode2() {
        let sentencepieces = vec![
            ("ab".to_string(), 0.0),
            ("cd".to_string(), -0.1),
            ("abc".to_string(), -0.2),
            ("a".to_string(), -0.3),
            ("b".to_string(), -0.4),
            ("c".to_string(), -0.5),
            ("ABC".to_string(), -0.5),
            ("abcdabcd".to_string(), 20.0), // User defined just max the scores.
            ("q".to_string(), 20.5),
            ("r".to_string(), 20.5),
            ("qr".to_string(), -0.5),
            ("ab".to_string(), 2.0),
            ("abc".to_string(), 5.0),
            ("abcd".to_string(), 10.0),
        ];

        let model = Unigram::from(&sentencepieces);
        assert_eq!(model.encode("abc"), vec!["abc"]);
        assert_eq!(model.encode("AB"), vec!["A", "B"]);
        assert_eq!(model.encode("abcd"), vec!["ab", "cd"]);
        assert_eq!(model.encode("abcc"), vec!["abc", "c"]);
        assert_eq!(
            model.encode("xabcabaabcdd"),
            vec!["x", "abc", "ab", "a", "ab", "cd", "d"]
        );
        assert_eq!(model.encode("xyz東京"), vec!["x", "y", "z", "東", "京"]);

        // User encoded in original version
        assert_eq!(model.encode("ABC"), vec!["ABC"]);
        assert_eq!(model.encode("abABCcd"), vec!["ab", "ABC", "cd"]);
        assert_eq!(model.encode("ababcdabcdcd"), vec!["ab", "abcdabcd", "cd"]);
        assert_eq!(model.encode("abqrcd"), vec!["ab", "q", "r", "cd"]);
    }

    #[test]
    fn test_unigram_from_file() {
        let model = Unigram::load(Path::new("data/model.json")).unwrap();
        let pretok = Whitespace;
        let input = pretok
            .pre_tokenize(&mut NormalizedString::from(
                "吾輩《わがはい》は猫である。名前はまだ無い。",
            ))
            .unwrap();
        println!("input {:?}", input);
        assert_eq!(
            model
                .tokenize(input)
                .unwrap()
                .iter()
                .map(|tok| tok.value.clone())
                .collect::<Vec<_>>(),
            vec![
                "吾輩",
                "《",
                "わが",
                "はい",
                "》",
                "は",
                "猫",
                "である",
                "。",
                "名前",
                "はまだ",
                "無い",
                "。"
            ]
        );
    }
}
