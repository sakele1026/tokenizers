/**
 * This class is not supposed to be instantiated directly. Instead, any implementation of a
 * Normalizer will return an instance of this class when instantiated.
 */
declare class Normalizer {}

export namespace normalizers {
  interface BertNormalizerOptions {
    /**
     * Whether to clean the text, by removing any control characters
     * and replacing all whitespaces by the classic one.
     * @default true
     */
    cleanText?:          boolean;
    /**
     * Whether to handle chinese chars by putting spaces around them.
     * @default true
     */
    handleChineseChars?: boolean;
    /**
     * Whether to lowercase.
     * @default true
     */
    lowercase?:          boolean;
    /**
     * Whether to strip all accents.
     * @default true
     */
    stripAccents?:       boolean;
  }

  /**
   * Instantiate a Bert Normalizer with the given options
   *
   * @param [options] Normalizer options
   * @returns {Normalizer} Bert Normalizer. Takes care of normalizing raw text before giving it to a Bert model.
   * This includes cleaning the text, handling accents, chinese chars and lowercasing
   */
  export function bertNormalizer(options?: BertNormalizerOptions): Normalizer;

  /**
   * Returns a new NFD Unicode Normalizer
   */
  export function nfd(): Normalizer;

  /**
   * Returns a new NFKD Unicode Normalizer
   */
  export function nfkd(): Normalizer;

  /**
   * Returns a new NFC Unicode Normalizer
   */
  export function nfc(): Normalizer;

  /**
   * Returns a new NFKC Unicode Normalizer
   */
  export function nfkc(): Normalizer;

  /**
   * Instantiate a new Normalization Sequence using the given normalizers
   * @param normalizers A list of Normalizer to be run as a sequence
   */
  export function sequence(normalizers: Normalizer[]): Normalizer;

  /**
   * Returns a new Lowercase Normalizer
   */
  export function lowercase(): Normalizer;
}
