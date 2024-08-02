// src/game/quiz.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: i32,
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quiz {
    pub id: i32,
    pub name: String,
    pub questions: Vec<Question>,
}

impl Quiz {
    pub fn new(id: i32, name: String, questions: Vec<Question>) -> Self {
        Self { id, name, questions }
    }

    pub fn check_answer(&self, question_index: usize, answer: usize) -> bool {
        if let Some(question) = self.questions.get(question_index) {
            question.correct_answer == answer
        } else {
            false
        }
    }
}