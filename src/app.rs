use iced::{
    widget::{button, text, text_input, Column, Container},
    Color, Command,
};

use crate::{
    domain::{Answer, AnswerResult, Trainer, Word},
    files::load_trainer_from_file,
    messages::Message,
    widgets::{app_column, app_container},
};

#[derive(Debug, Default, Clone)]
pub struct Application {
    file_chosen: FileChosen,
}

#[derive(Debug, Clone)]
pub enum FileChosen {
    No {
        error: Option<String>,
    },
    Yes {
        trainer: Trainer,
        answer_text_input: String,
        answer_result: Option<AnswerResult>,
    },
}

impl Default for FileChosen {
    fn default() -> Self {
        Self::No { error: None }
    }
}

impl Application {
    pub fn view(&self) -> Container<Message> {
        app_container(match &self.file_chosen {
            FileChosen::No { error } => {
                let column = app_column()
                    .push(text("Необходимо открыть файл со словами"))
                    .push(button("Выбрать файл").on_press(Message::ChooseFile));

                let red = Color::from_rgb8(255, 40, 40);

                match error {
                    Some(error) => column.push(text(error).color(red)),
                    None => column,
                }
            }
            FileChosen::Yes {
                trainer,
                answer_result,
                answer_text_input,
            } => match trainer.get_current_word() {
                Some(current_word) => {
                    self.current_word_view(trainer, current_word, answer_text_input, answer_result)
                }
                None => self.words_are_out_view(),
            },
        })
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChooseFile => Command::perform(load_trainer_from_file(), Message::FileChosen),
            Message::FileChosen(trainer) => {
                match trainer {
                    Ok(trainer) => {
                        self.file_chosen = FileChosen::Yes {
                            trainer,
                            answer_result: None,
                            answer_text_input: String::new(),
                        }
                    }
                    Err(error) => {
                        self.file_chosen = FileChosen::No {
                            error: match error {
                                crate::files::LoadTrainerError::DialogClosed => None,
                                crate::files::LoadTrainerError::IoError(_) => {
                                    Some("Ошибка ввода/вывода".to_string())
                                }
                                crate::files::LoadTrainerError::ParseError => Some(
                                    "Некорректный файл, отредактируйте его и попробуйте ещё раз"
                                        .to_string(),
                                ),
                            },
                        }
                    }
                }

                Command::none()
            }
            Message::ResetFile => {
                self.file_chosen = FileChosen::No { error: None };
                Command::none()
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

                Command::none()
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

                Command::none()
            }
            Message::NextWord => {
                if let FileChosen::Yes {
                    trainer: _,
                    answer_text_input: _,
                    answer_result: Some(answer_result),
                } = &self.file_chosen
                {
                    self.file_chosen = FileChosen::Yes {
                        trainer: answer_result.next(),
                        answer_text_input: String::new(),
                        answer_result: None,
                    }
                }

                Command::none()
            }
        }
    }

    fn current_word_view(
        &self,
        trainer: &Trainer,
        current_word: &Word,
        text_input_value: &str,
        answer_result: &Option<AnswerResult>,
    ) -> Column<Message> {
        app_column()
            .push(text(format!("{}/{}", trainer.get_correct_answers_count(), trainer.get_words_count())))
            .push(text(current_word.get_original()))
            .push(match answer_result {
                Some(answer_result) => self.answer_correctness_view(answer_result, current_word),
                None => self.answer_form_view(text_input_value),
            })
    }

    fn words_are_out_view(&self) -> Column<Message> {
        app_column()
            .push(text("Слова закончились"))
            .push(button("Перейти к выбору файла").on_press(Message::ResetFile))
    }

    fn answer_correctness_view(
        &self,
        answer_result: &AnswerResult,
        current_word: &Word,
    ) -> Column<Message> {
        let red = Color::from_rgb8(255, 40, 40);
        let green = Color::from_rgb8(40, 255, 40);

        let correctness = if answer_result.is_correct() {
            text("Правильный ответ").color(green)
        } else {
            text(format!(
                "Неправильный ответ. Правильный: {}",
                format_traslation(current_word.get_translation())
            ))
            .color(red)
        };

        app_column()
            .push(correctness)
            .push(button("Следующий вопрос").on_press(Message::NextWord))
    }

    fn answer_form_view(&self, text_input_value: &str) -> Column<Message> {
        app_column()
            .push(
                text_input("Введите перевод", text_input_value)
                    .width(500)
                    .on_submit(Message::SubmitAnswer(Answer::from_answer_text(
                        text_input_value,
                    )))
                    .on_input(Message::ChangeAnswerTextInput),
            )
            .push(
                button("Ответить").on_press(Message::SubmitAnswer(Answer::from_answer_text(
                    text_input_value,
                ))),
            )
    }
}

fn format_traslation(translation: &[Vec<String>]) -> String {
    translation
        .iter()
        .map(|variants| variants.join("/"))
        .collect::<Vec<String>>()
        .join(" ")
}
