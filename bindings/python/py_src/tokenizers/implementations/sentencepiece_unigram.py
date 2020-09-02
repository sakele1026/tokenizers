from tokenizers import Tokenizer, AddedToken, pre_tokenizers, decoders, trainers
from tokenizers.models import Unigram
from tokenizers.normalizers import NFKC
from .base_tokenizer import BaseTokenizer

from typing import Optional, List, Union


class SentencePieceUnigramTokenizer(BaseTokenizer):
    """SentencePiece Unigram Tokenizer

    Represents the Unigram algorithm, with the pretokenization used by SentencePiece
    """

    def __init__(
        self, vocab: Optional[str] = None, replacement: str = "▁", add_prefix_space: bool = True,
    ):
        if vocab is not None:
            tokenizer = Tokenizer(Unigram(vocab))
        else:
            tokenizer = Tokenizer(Unigram())

        tokenizer.normalizer = NFKC()
        tokenizer.pre_tokenizer = pre_tokenizers.Sequence(
            [
                pre_tokenizers.WhitespaceSplit(),
                pre_tokenizers.Metaspace(
                    replacement=replacement, add_prefix_space=add_prefix_space
                ),
            ]
        )
        tokenizer.decoder = decoders.Metaspace(
            replacement=replacement, add_prefix_space=add_prefix_space
        )

        parameters = {
            "model": "SentencePieceUnigram",
            "replacement": replacement,
            "add_prefix_space": add_prefix_space,
        }

        super().__init__(tokenizer, parameters)

    def train(
        self,
        files: Union[str, List[str]],
        vocab_size: int = 8000,
        show_progress: bool = True,
        special_tokens: List[Union[str, AddedToken]] = [],
    ):
        """ Train the model using the given files """

        trainer = trainers.UnigramTrainer(
            vocab_size=vocab_size, special_tokens=special_tokens, show_progress=show_progress,
        )

        if isinstance(files, str):
            files = [files]
        self._tokenizer.train(trainer, files)
