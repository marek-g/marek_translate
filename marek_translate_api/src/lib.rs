use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait TextTranslator {
    async fn translate(
        &mut self,
        source_text: &str,
        source_lang: &str,
        dest_lang: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>>;
}
