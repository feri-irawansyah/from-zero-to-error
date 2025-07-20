use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionResult<T, E> {
    pub result: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<E>,
}

// Implementasi Default
impl<T, E> Default for ActionResult<T, E> {
    fn default() -> Self {
        Self {
            result: false, // Default-nya false
            message: String::new(),
            data: None,
            error: None,
        }
    }
}

use crate::middleware::session::SessionData; 

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SuccessResponse<T> {
    pub data: Option<T>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ErrorResponse {
    pub error: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}

#[derive(Clone, Debug, PartialEq, Serialize, Default, Deserialize)]
pub struct EmailRequest {
    pub name: String,
    pub recipient: String,
    pub subject: String,
    pub message: String
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Skill {
    pub skill_id: i32,
    pub title: String,
    pub description: String,
    pub url_docs: String,
    pub image_src: String,
    pub progress: i32,
    pub star: i32,
    pub experience: i32,
    pub tech_category: String,
    pub last_update: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Portfolio {
    pub portfolio_id: i32,
    pub title: String,
    pub description: String,
    pub url_docs: String,
    pub image_src: String,
    pub tech: Vec<i32>,
    pub last_update: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Category {
    pub name: &'static str,
    pub image: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    pub list_tech: &'static [&'static str],
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Notes {
    pub notes_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub slug: String,
    pub last_update: String,
    pub category: String,
    pub hashtag: Option<Vec<String>>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct HashTag {
    pub tag_id: i32,
    pub tag_name: String,
    pub img_url: String,
    pub notes_id: i32
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct HashTagData {
    pub result: bool,
    pub message: String,
    pub data: Vec<HashTag>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Note {
    pub notes_id: i32,
    pub title: String,
    pub category: String,
    pub slug: String,
    pub content: String,
    pub last_update: String,
    pub hashtag: Option<Vec<String>>
}

impl Note {
    pub fn new() -> Self {
        Note {
            notes_id: 0,
            title: "".to_string(),
            category: "".to_string(),
            slug: "".to_string(),
            content: "".to_string(),
            last_update: "".to_string(),
            hashtag: None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NoteData {
    pub data: Note,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NotesData {
    pub rows: Vec<Notes>,
    #[serde(rename = "totalNotFiltered")]
    pub total_not_filtered: usize,
    pub total: usize,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SkillsData {
    pub rows: Vec<Skill>,
    #[serde(rename = "totalNotFiltered")]
    pub total_not_filtered: usize,
    pub total: usize,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PortfolioData {
    pub rows: Vec<Portfolio>,
    #[serde(rename = "totalNotFiltered")]
    pub total_not_filtered: usize,
    pub total: usize,
}

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub count: RwSignal<i32>,
    pub name: RwSignal<String>,
    pub title: RwSignal<String>,
    pub loading: RwSignal<bool>,
    pub session: RwSignal<SessionData>,
    pub note_url: RwSignal<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ModalState {
    pub note_url: RwSignal<Option<String>>,
    pub title: RwSignal<String>,
    pub data: RwSignal<serde_json::Value>
}

