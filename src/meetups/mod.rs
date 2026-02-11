use serde::{Deserialize, Serialize};

pub mod init;
pub mod website;

use crate::remote;

#[derive(Debug, Deserialize, Serialize)]
pub struct Meetups {
    pub meetups: Vec<Meetup>,
}

impl Default for Meetups {
    fn default() -> Self {
        Meetups {
            meetups: vec![Meetup::default_with_id(0), Meetup::default_with_id(1)],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Meetup {
    pub id: u32,
    pub title: String,
    pub date: String,
    pub address: Address,
    pub description: String,
    pub markdown_name: String,
    pub sponsors: Vec<Sponsor>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sponsor {
    pub name: String,
    pub website: Option<String>,
    pub content: Option<String>,
}

impl Meetup {
    fn default_with_id(id: u32) -> Self {
        let title = format!("Meetup {}", id);
        Meetup {
            id,
            title,
            date: "2024-01-01".to_string(),
            address: Address {
                street: "123 Default St".to_string(),
                city: "Default City".to_string(),
                postal_code: "00000".to_string(),
                country: "Default Country".to_string(),
                description: Some("Go around the building and dance.".to_string()),
            },
            description: "This is a default meetup description.".to_string(),
            markdown_name: "http://example.com/meetup.md".to_string(),
            sponsors: vec![Sponsor {
                name: "Default Sponsor".to_string(),
                website: Some("http://sponsor.com".to_string()),
                content: Some("This is a default sponsor.".to_string()),
            }],
        }
    }
}

pub fn meetup_ui() {
    let (_css, markdown) = remote::get_files();
    println!("{markdown}");
}
