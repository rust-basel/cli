pub struct Meetup {
    pub id: u32,
    pub title: String,
    pub date: String,
    pub location: String,
    pub description: String,
    pub sponsors: Vec<Sponsor>,
}

pub struct Sponsor {
    pub name: String,
    pub website: Option<String>,
    pub content: Option<String>,
}
