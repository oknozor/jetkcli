pub mod issue;
pub mod response;
pub mod status;
pub mod project;

pub trait ToPage {
    fn to_page(&self) -> String;
}