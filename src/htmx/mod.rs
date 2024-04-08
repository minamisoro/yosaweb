mod character;
mod contact;
mod footer;
mod guideline;
mod header;
mod navbar;
mod store;

use askama::Template;
use axum::Router;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Root {
    navbar: navbar::Navbar,
    header: header::Header,
    character: character::Character,
    guideline: guideline::Guideline,
    store: store::Store,
    contact: contact::Contact,
    footer: footer::Footer,
}

pub fn route() -> Router<()> {
    axum::Router::new()
}

impl Root {
    pub fn new() -> Self {
        Self {
            navbar: navbar::Navbar::new(),
            header: header::Header::new(),
            character: character::Character::new(),
            guideline: guideline::Guideline::new(),
            store: store::Store::new(),
            contact: contact::Contact::new(),
            footer: footer::Footer::new(),
        }
    }
}
