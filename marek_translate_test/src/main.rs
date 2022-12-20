use marek_translate_api::TextTranslator;
use marek_translate_locally::TranslateLocally;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut translator = TranslateLocally::new()?;

    let text = "This is a translation example.";
    println!(
        "{} -> {}",
        text,
        translator.translate(text, "en", "pl").await?
    );

    let text = "Another sentence to translate.";
    println!(
        "{} -> {}",
        text,
        translator.translate(text, "en", "pl").await?
    );

    Ok(())
}
