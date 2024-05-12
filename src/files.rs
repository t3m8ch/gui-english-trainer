use std::io;

use crate::domain::{Trainer, Word};

#[derive(Debug, Clone)]
pub enum LoadTrainerError {
    DialogClosed,
    IoError(io::ErrorKind),
    ParseError,
}

pub async fn load_trainer_from_file() -> Result<Trainer, LoadTrainerError> {
    Ok(Trainer::new(
        &read_text()
            .await?
            .split('\n')
            .filter(|line| line.trim() != "")
            .map(parse_line)
            .collect::<Result<Vec<Word>, LoadTrainerError>>()?,
    ))
}

async fn read_text() -> Result<String, LoadTrainerError> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Открыть файл со словами")
        .pick_file()
        .await
        .ok_or(LoadTrainerError::DialogClosed)?;

    tokio::fs::read_to_string(&picked_file.path())
        .await
        .map_err(|error| LoadTrainerError::IoError(error.kind()))
}

fn parse_line(line: &str) -> Result<Word, LoadTrainerError> {
    let mut line = line.trim().split(':');
    if let Some(original) = line.next() {
        let original = original.trim();
        if let Some(translation) = line.next() {
            let translation = translation
                .trim()
                .split(' ')
                .map(|variants| {
                    variants
                        .split('/')
                        .map(|variant| variant.trim().to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>();

            return Ok(Word::new(original, &translation));
        }
    }

    Err(LoadTrainerError::ParseError)
}
