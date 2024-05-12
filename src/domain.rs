use std::iter::zip;

#[derive(Debug, Clone)]
pub struct Trainer {
    words: Vec<Word>,
    words_count: usize,
}

#[derive(Debug, Clone)]
pub struct Word {
    original: String,
    translation: Vec<Vec<String>>,
}

impl Word {
    pub fn new(original: &str, translation: &[Vec<String>]) -> Self {
        Self {
            original: original.trim().to_lowercase(),
            translation: translation
                .iter()
                .map(|word| {
                    let mut words: Vec<String> = word
                        .iter()
                        .map(|variant| variant.trim().to_lowercase())
                        .collect();
                    words.sort();
                    words
                })
                .collect(),
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
pub struct Answer(Vec<Vec<String>>);

impl Answer {
    pub fn from_answer_text(answer: &str) -> Self {
        Self(
            answer
                .trim()
                .split(' ')
                .map(|word| {
                    let mut variants: Vec<String> = word
                        .trim()
                        .split('/')
                        .map(|w| w.trim().to_lowercase())
                        .collect();
                    variants.sort();
                    variants
                })
                .collect(),
        )
    }
}

impl Trainer {
    pub fn new(words: &[Word]) -> Self {
        Self { words: words.to_vec(), words_count: words.len() }
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

    pub fn get_words_count(&self) -> usize {
        self.words_count
    }

    pub fn get_correct_answers_count(&self) -> usize {
        self.words_count - self.words.len()
    }

    fn is_correct(&self, answer: Answer) -> bool {
        match self.get_current_word() {
            Some(current_word) => zip(answer.0, &current_word.translation)
                .all(|(actual, expected)| actual == *expected),
            None => false,
        }
    }

    fn remove_first_word(&self) -> Trainer {
        Trainer {
            words: self.words.iter().skip(1).cloned().collect(),
            words_count: self.words_count,
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
            words_count: self.words_count,
        }
    }
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
