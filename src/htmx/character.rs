use askama::Template;

#[derive(Template)]
#[template(path = "character.html")]
pub struct Character {}

impl Character {
    pub fn new() -> Self {
        Self {}
    }
}
