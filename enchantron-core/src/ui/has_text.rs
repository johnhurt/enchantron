use std::string::String;

pub trait HasText {
    fn get_text(&self) -> String;
    fn set_text(&self, new_text: String);
}
