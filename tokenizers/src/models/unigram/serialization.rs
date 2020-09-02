use super::model::Unigram;
use serde::{ser::SerializeSeq, Serialize, Serializer};

impl Serialize for Unigram {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for i in 0..self.len() {
            seq.serialize_element(&(&self.vocab[i], &self.scores[i]))?;
        }

        seq.end()
    }
}
