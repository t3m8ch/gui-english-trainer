use iced::{
    widget::{button, text, text_input, Column, Container},
    Color, Element,
};

use crate::{
    domain::{get_test_trainer, Answer, AnswerResult, Trainer, Word},
    messages::Message,
    widgets::{app_column, app_container},
};

#[derive(Debug, Default, Clone)]
pub struct Application {
    file_chosen: FileChosen,
}

#[derive(Debug, Clone)]
pub enum FileChosen {
    No,
    Yes {
        trainer: Trainer,
        answer_text_input: String,
        answer_result: Option<AnswerResult>,
    },
}

impl Default for FileChosen {
    fn default() -> Self {
        Self::No
    }
}

impl Application {
    pub fn view(&self) -> Container<Message> {
        app_container(match &self.file_chosen {
            FileChosen::No => app_column()
                .push(text("Необходимо открыть файл со словами"))
                .push(button("Выбрать файл").on_press(Message::ChooseFile)),
            FileChosen::Yes {
                trainer,
                answer_result,
                answer_text_input,
            } => match trainer.get_current_word() {
                Some(current_word) => {
                    self.current_word_view(current_word, answer_text_input, answer_result)
                }
                None => self.words_are_out_view(),
            },
        })
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChooseFile => {
                self.file_chosen = FileChosen::Yes {
                    trainer: get_test_trainer(),
                    answer_result: None,
                    answer_text_input: String::new(),
                }
            }
            Message::ResetFile => {
                self.file_chosen = FileChosen::No;
            }
            Message::SubmitAnswer(answer) => {
                if let FileChosen::Yes {
                    trainer,
                    answer_text_input,
                    answer_result: _,
                } = &self.file_chosen
                {
                    self.file_chosen = FileChosen::Yes {
                        trainer: trainer.clone(),
                        answer_text_input: answer_text_input.clone(),
                        answer_result: Some(trainer.answer(answer)),
                    }
                }
            }
            Message::ChangeAnswerTextInput(new_value) => {
                if let FileChosen::Yes {
                    trainer,
                    answer_text_input: _,
                    answer_result,
                } = &self.file_chosen
                {
                    self.file_chosen = FileChosen::Yes {
                        trainer: trainer.clone(),
                        answer_text_input: new_value,
                        answer_result: answer_result.clone(),
                    }
                }
            }
            Message::NextWord => {
                if let FileChosen::Yes {
                    trainer: _,
                    answer_text_input: _,
                    answer_result,
                } = &self.file_chosen
                {
                    if let Some(answer_result) = answer_result {
                        self.file_chosen = FileChosen::Yes {
                            trainer: answer_result.next(),
                            answer_text_input: String::new(),
                            answer_result: None,
                        }
                    }
                }
            },
        }
    }

    fn current_word_view(
        &self,
        current_word: &Word,
        text_input_value: &str,
        answer_result: &Option<AnswerResult>,
    ) -> Column<Message> {
        app_column()
            .push(text(current_word.get_original()))
            .push(match answer_result {
                Some(answer_result) => Element::from(
                    app_column()
                        .push(if answer_result.is_correct() {
                            text("Правильный ответ").color(Color::from_rgb8(40, 255, 40))
                        } else {
                            text(format!(
                                "Неправильный ответ. Правильный: {}",
                                format_traslation(current_word.get_translation())
                            ))
                            .color(Color::from_rgb8(255, 40, 40))
                        })
                        .push(button("Следующий вопрос").on_press(Message::NextWord)),
                ),
                None => Element::from(
                    app_column()
                        .push(
                            text_input("Введите перевод", text_input_value)
                                .width(500)
                                .on_submit(Message::SubmitAnswer(Answer::from_str(
                                    text_input_value,
                                )))
                                .on_input(Message::ChangeAnswerTextInput),
                        )
                        .push(button("Ответить").on_press(Message::SubmitAnswer(
                            Answer::from_str(text_input_value),
                        ))),
                ),
            })
    }

    fn words_are_out_view(&self) -> Column<Message> {
        app_column()
            .push(text("Слова закончились"))
            .push(button("Перейти к выбору файла").on_press(Message::ResetFile))
    }
}

fn format_traslation(translation: &Vec<Vec<String>>) -> String {
    translation
        .iter()
        .map(|variants| variants.join("/"))
        .collect::<Vec<String>>()
        .join(" ")
}
