use std::collections::{HashMap, HashSet};
use rand::{distributions::Alphanumeric, Rng};

use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Room {
    code: String,
    users: HashMap<String, User>,
    started: bool,
    shuffle_questions: bool,
    shuffle_answers: bool,
    questions: Vec<Question>,
}

pub struct User {
    id: u64,
    name: String,
}

pub struct Question {
    prompt: String,
    answers: HashSet<String>,
    correct_answer: String,
}

impl Room {
    // Creates new room code with new code and default parameters
    pub fn new() -> Self {
        // Randomly generated 5 character alphanumeric code
        let room_code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

        return Room {
            code: room_code,
            users: HashMap::new(),
            started: false,
            shuffle_questions: false,
            shuffle_answers: false,
            questions: Vec::new(),
        }
    }

    // Starts room
    pub fn start_room(&mut self) {
        if !self.started {
            self.started = true;

            if self.shuffle_questions {
                self.questions.shuffle(&mut thread_rng());
            }
        }
    }

    // Adds given user to room
    pub fn add_user(&mut self, new_user: User) {
        self.users.insert(new_user.name.clone(), new_user);
    }

    // Removes given user if exists
    pub fn remove_user(&mut self, user: User) {
        self.users.remove(&user.name.clone());
    }

    // Toggles whether or not question order is shuffled
    pub fn toggle_shuffle_questions(&mut self) {
        self.shuffle_questions = !self.shuffle_questions;
    }

    // Toggles whether or not answer order is shuffled
    pub fn toggle_shuffle_answers(&mut self) {
        self.shuffle_answers = !self.shuffle_answers;
    }
}