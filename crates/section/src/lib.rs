use md_writer::LF;
use std::fmt::{self, Formatter};

pub struct Section {
    pub title: String,
    pub body: String,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{LF}{}", self.title, self.body)
    }
}
