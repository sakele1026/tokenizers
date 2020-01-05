# Tokenizers

Provides an implementation of today's most used tokenizers, with a focus on performances and
versatility.

## What is a Tokenizer

A Tokenizer works as a pipeline, processing some raw text as input, to finally output an
`Encoding`.
The various steps of the pipeline are:

1. The `Normalizer` is in charge of normalizing the text. Common examples of Normalization are
   the unicode normalization standards, such as `NFD` or `NFKC`.
2. The `PreTokenizer` is in charge of splitting the text as relevant. The most common way of
   splitting text is simply on whitespaces, to manipulate words.
3. The `Model` is in charge of doing the actual tokenization. An example of `Model` would be
   `BPE` or `WordPiece`.
4. The `PostProcessor` is in charge of post processing the `Encoding`, to add anything relevant
   that a language model would need, like special tokens.

## Bindings

We provide bindings to the following languages (more to come!):
  - [Python](https://github.com/huggingface/tokenizers/tree/master/bindings/python)
