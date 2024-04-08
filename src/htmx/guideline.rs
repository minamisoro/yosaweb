use askama::Template;

#[derive(Template)]
#[template(path = "guideline.html")]
pub struct Guideline {}

impl Guideline {
    pub fn new() -> Self {
        Self {}
    }
}
