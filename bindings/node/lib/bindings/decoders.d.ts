/**
 * This class is not supposed to be instantiated directly. Instead, any implementation of
 * a Decoder will return an instance of this class when instantiated.
 */
declare class Decoder {}

/**
 * Instantiate a new ByteLevel Decoder
 */
export function byteLevelDecoder(): Decoder;

/**
 * Instantiate a new WordPiece Decoder
 * @param {string} [prefix='##'] The prefix to use for subwords that are not a beginning-of-word
 */
export function wordPieceDecoder(prefix?: string): Decoder;

/**
 * Instantiate a new Metaspace
 *
 * @param {string} [replacement='▁'] The replacement character. 
 * Must be exactly one character. By default we use the `▁` (U+2581) meta symbol (same as in SentencePiece).
 * @param {boolean} [addPrefixSpace=true] Whether to add a space to the first word if there isn't already one.
 * This lets us treat `hello` exactly like `say hello`.
 */
export function metaspaceDecoder(replacement?: string, addPrefixSpace?: boolean): Decoder;

/**
 * Instantiate a new BPE Decoder
 * @param {string} [suffix='</w>'] The suffix that was used to caracterize an end-of-word.
 * This suffix will be replaced by whitespaces during the decoding
 */
export function bpeDecoder(suffix?: string): Decoder;
