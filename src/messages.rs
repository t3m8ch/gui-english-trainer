use crate::{
    domain::{Answer, Trainer},
    files::LoadTrainerError,
};

#[derive(Debug, Clone)]
pub enum Message {
    FileChosen(Result<Trainer, LoadTrainerError>),
    ChooseFile,
    ResetFile,
    SubmitAnswer(Answer),
    ChangeAnswerTextInput(String),
    NextWord,
}
