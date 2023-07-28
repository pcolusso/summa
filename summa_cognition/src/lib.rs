use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsModel, SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType
};
use rust_bert::RustBertError;

pub struct Engine {
    model: SentenceEmbeddingsModel,
}

impl Engine {
    pub fn new() -> Result<Self, RustBertError> {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()?;

        Ok(Engine { model })
    }

    pub fn encode(&self, text: &str) -> Result<Vec<f32>, RustBertError> {
        let input = vec![text];
        let embeddings = self.model.encode(&input)?;

        Ok(embeddings[0].to_vec())
    }
}
