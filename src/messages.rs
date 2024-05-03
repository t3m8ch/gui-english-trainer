use crate::domain::Answer;

#[derive(Debug, Clone)]
pub enum Message {
    ChooseFile,
    ResetFile,
    SubmitAnswer(Answer),
    ChangeAnswerTextInput(String),
    NextWord,
}
