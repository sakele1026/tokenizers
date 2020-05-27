use super::{AddedToken, Tokenizer};
use serde::{self, ser::SerializeStruct, Deserialize, Serialize, Serializer};

static SERIALIZATION_VERSION: &str = "1.0";

#[derive(Serialize, Deserialize)]
struct AddedTokenWithId {
    id: u32,
    #[serde(flatten)]
    token: AddedToken,
}

impl Serialize for Tokenizer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tokenizer = serializer.serialize_struct("Tokenizer", 10)?;

        // Start by adding the current version
        tokenizer.serialize_field("version", SERIALIZATION_VERSION)?;

        // Then add our parts
        tokenizer.serialize_field("model", &self.model)?;
        tokenizer.serialize_field("normalizer", &self.normalizer)?;
        tokenizer.serialize_field("pre_tokenizer", &self.pre_tokenizer)?;
        tokenizer.serialize_field("post_processor", &self.post_processor)?;
        tokenizer.serialize_field("decoder", &self.decoder)?;

        // Added tokens
        let added_tokens = self
            .added_tokens
            .iter()
            .map(|token| AddedTokenWithId {
                id: self.added_tokens_map[&token.content],
                token: token.clone(),
            })
            .collect::<Vec<_>>();
        tokenizer.serialize_field("added_tokens", &added_tokens)?;
        let special_tokens = self
            .special_tokens
            .iter()
            .map(|token| AddedTokenWithId {
                id: self.added_tokens_map[&token.content],
                token: token.clone(),
            })
            .collect::<Vec<_>>();
        tokenizer.serialize_field("special_tokens", &special_tokens)?;

        // Params
        tokenizer.serialize_field("truncation", &self.truncation)?;
        tokenizer.serialize_field("padding", &self.padding)?;

        tokenizer.end()
    }
}
