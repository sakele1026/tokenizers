# Tokenizers

Provides an implementation of today's most used tokenizers, with a focus on performance and
versatility.

## What is a Tokenizer

A Tokenizer works as a pipeline, it processes some raw text as input and outputs an `Encoding`.
The various steps of the pipeline are:

1. The `Normalizer`: in charge of normalizing the text. Common examples of normalization are
   the [unicode normalization standards](https://unicode.org/reports/tr15/#Norm_Forms), such as `NFD` or `NFKC`.
2. The `PreTokenizer`: in charge of creating initial words splits in the text. The most common way of
   splitting text is simply on whitespace.
3. The `Model`: in charge of doing the actual tokenization. An example of a `Model` would be
   `BPE` or `WordPiece`.
4. The `PostProcessor`: in charge of post-processing the `Encoding` to add anything relevant
   that, for example, a language model would need, such as special tokens.

## Main features:

 - Train new vocabularies and tokenize, using todays most used tokenizers.
 - Extremely fast (both training and tokenization), thanks to the Rust implementation. Takes
   less than 20 seconds to tokenize a GB of text on a server's CPU.
 - Easy to use, but also extremely versatile.
 - Designed for research and production.
 - Normalization comes with alignments tracking. It's always possible to get the part of the
   original sentence that corresponds to a given token.
 - Does all the pre-processing: Truncate, Pad, add the special tokens your model needs.

## Bindings

We provide bindings to the following languages (more to come!):
  - [Python](https://github.com/huggingface/tokenizers/tree/master/bindings/python)
