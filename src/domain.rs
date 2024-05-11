use std::iter::zip;

#[derive(Debug, Clone)]
pub struct Trainer {
    words: Vec<Word>,
}

#[derive(Debug, Clone)]
pub struct Word {
    original: String,
    translation: Vec<Vec<String>>,
}

impl Word {
    pub fn new(original: String, translation: Vec<Vec<String>>) -> Self {
        Self {
            original,
            translation,
        }
    }

    pub fn get_original(&self) -> &String {
        &self.original
    }

    pub fn get_translation(&self) -> &Vec<Vec<String>> {
        &self.translation
    }
}

#[derive(Debug, Clone)]
pub struct AnswerResult {
    trainer: Trainer,
    correct: bool,
}

impl AnswerResult {
    pub fn get_trainer(&self) -> &Trainer {
        &self.trainer
    }

    pub fn is_correct(&self) -> bool {
        self.correct
    }
}

#[derive(Debug, Clone)]
pub struct Answer(Vec<String>);

impl Answer {
    pub fn from_answer_text(answer: &str) -> Self {
        Self(answer.split(' ').map(|s| s.to_string()).collect())
    }
}

impl Trainer {
    pub fn new(words: Vec<Word>) -> Self {
        Self { words }
    }

    pub fn answer(&self, answer: Answer) -> AnswerResult {
        AnswerResult {
            trainer: self.clone(),
            correct: self.is_correct(answer),
        }
    }

    pub fn get_current_word(&self) -> Option<&Word> {
        self.words.first()
    }

    fn is_correct(&self, answer: Answer) -> bool {
        match self.get_current_word() {
            Some(current_word) => zip(answer.0, &current_word.translation)
                .all(|(actual, expected)| expected.contains(&actual)),
            None => false,
        }
    }

    fn remove_first_word(&self) -> Trainer {
        Trainer {
            words: self.words.iter().skip(1).cloned().collect(),
        }
    }

    fn move_first_word_to_end(&self) -> Trainer {
        Trainer {
            words: self
                .words
                .iter()
                .enumerate()
                .map(|(i, _)| self.words[(i + 1) % self.words.len()].clone())
                .collect(),
        }
    }
}

pub fn get_test_trainer() -> Trainer {
    Trainer::new(vec![Word::new(
        "Читать по буквам".to_string(),
        vec![
            vec!["spell".to_string()],
            vec!["spelt".to_string(), "spelled".to_string()],
            vec!["spell".to_string(), "spelled".to_string()],
        ],
    )])
}

impl AnswerResult {
    pub fn next(&self) -> Trainer {
        if self.correct {
            self.trainer.remove_first_word()
        } else {
            self.trainer.move_first_word_to_end()
        }
    }
}
