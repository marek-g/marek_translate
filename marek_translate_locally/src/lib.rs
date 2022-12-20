mod models;

use crate::models::{TranslateRequest, TranslateRequestData, TranslateResponse};
use async_process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use async_trait::async_trait;
use futures_lite::io::AsyncWriteExt;
use futures_lite::AsyncReadExt;
use marek_translate_api::TextTranslator;
use std::error::Error;

pub struct TranslateLocally {
    _command: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl TranslateLocally {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        #[cfg(target_os = "windows")]
        let mut command = Command::new("./translateLocally.exe")
            .arg("-p")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        #[cfg(not(target_os = "windows"))]
        let mut command = Command::new("./translateLocally")
            .arg("-p")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = command.stdin.take().unwrap();
        let stdout = command.stdout.take().unwrap();

        Ok(TranslateLocally {
            _command: command,
            stdin,
            stdout,
        })
    }
}

#[async_trait]
impl TextTranslator for TranslateLocally {
    async fn translate(
        &mut self,
        source_text: &str,
        source_lang: &str,
        dest_lang: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let request = TranslateRequest {
            id: 1,
            command: "Translate",
            data: TranslateRequestData {
                src: source_lang,
                trg: dest_lang,
                text: source_text,
            },
        };

        let request = serde_json::to_vec(&request)?;
        let length = (request.len() as u32).to_ne_bytes();

        self.stdin.write(&length).await?;
        self.stdin.write(&request).await?;

        let mut response_len = [0u8; 4];
        self.stdout.read(&mut response_len).await?;
        let response_len = u32::from_ne_bytes(response_len);

        let mut res = vec![0u8; response_len as usize];
        self.stdout.read(&mut res).await?;

        let response = serde_json::from_slice::<TranslateResponse>(&res)?;

        if !response.success {
            return Err("translation response doesn't contain success".into());
        }

        if let Some(data) = response.data {
            Ok(data.target.text)
        } else {
            Err("translation response doesn't contain data".into())
        }
    }
}
