use crate::prelude::ImdbSearchEngine;

pub struct ATag {
    pub(crate) text: String,
    pub(crate) link: String,
}

impl ATag {
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub fn link(&self) -> &str {
        self.link.as_ref()
    }

    pub fn get_abs_link(&self, engine: &ImdbSearchEngine) -> String {
        format!("{}{}", engine.base_uri(), self.link)
    }
}
