use std::collections::{HashMap, HashSet};
use axum::extract::ws::{WebSocket, Message};
use futures_util::stream::{SplitSink, SplitStream};
use rand::{distributions::Alphanumeric, Rng};

use rand::thread_rng;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use redis_derive::{FromRedisValue, ToRedisArgs};
use r2d2_redis::redis;

#[derive(Serialize, FromRedisValue, ToRedisArgs)]
pub struct Room {
    /// 5 digit alphanumeric room code
    code: String,
    /// List of users actively in room
    users: Vec<User>,
    /// Scores per user
    user_scores: Vec<(User, i32)>,
    /// Whether or not the room has begun
    started: bool,
    /// Whether or not to shuffle the order questions appear in
    shuffle_questions: bool,
    /// Whether or not to shuffle the order answers appear in
    shuffle_answers: bool,
    /// List of questions for the room
    questions: Vec<Question>,
}

#[derive(Serialize, FromRedisValue, ToRedisArgs)]
pub struct User {
    id: u64,
    name: String,
}

#[derive(Serialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Question {
    pub prompt: String,
    pub answers: HashSet<String>,
    correct_answer: String,
}

impl Question {
    pub fn check_correct(&self, answer: String) -> bool {
        return answer == self.correct_answer;
    }
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
            users: Vec::new(),
            user_scores: Vec::new(),
            started: false,
            shuffle_questions: false,
            shuffle_answers: false,
            questions: Vec::new(),
        }
    }

    // Returns and removes question from inner list if not empty
    pub fn new_question(&self) -> Question {
        let mut rng = rand::thread_rng();
        let question = rng.gen_range(0..self.questions.len());
        return self.questions[question].clone();
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

    // Gets room code
    pub fn get_code(&self) -> String {
        return self.code.clone();
    }

    // Adds given user to room
    pub fn add_user(&mut self, new_user: User) {
        self.users.push(new_user);
    }

    // Removes given user if exists
    pub fn remove_user(&mut self, removed_user: User) {
        let mut old_user = 0;
        for (index, user) in self.users.iter().enumerate() {
            if user.name == removed_user.name {
                old_user = index;
            }
        }
        self.users.remove(old_user);
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