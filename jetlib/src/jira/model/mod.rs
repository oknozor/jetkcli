pub mod issue;
pub mod project;
pub mod response;
pub mod status;
pub mod transition;

pub trait ToPage {
    fn to_page(&self) -> String;
}
