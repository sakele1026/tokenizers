const native = require("./native");

module.exports = {
  byteLevelDecoder: native.decoders_ByteLevel,
  wordPieceDecoder: native.decoders_WordPiece,
  byteFallbackDecoder: native.decoders_ByteFallback,
  metaspaceDecoder: native.decoders_Metaspace,
  bpeDecoder: native.decoders_BPEDecoder,
  ctcDecoder: native.decoders_CTC,
  sequenceDecoder: native.decoders_Sequence,
};
