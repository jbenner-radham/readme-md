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

pub struct Readme {
    sections: Vec<Section>,
}

impl fmt::Display for Readme {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let readme = self
            .sections
            .iter()
            .map(|section| section.to_string())
            .collect::<Vec<String>>()
            .join(&LF.to_string().repeat(2));

        write!(f, "{readme}")
    }
}

impl Readme {
    pub fn new(sections: Vec<Section>) -> Self {
        Readme { sections }
    }
}
