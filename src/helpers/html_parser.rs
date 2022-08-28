use scraper::{ElementRef, Html};

use super::get_selector;

pub(crate) trait HtmlParserHelper {
    fn select_first(&self, selector: &str) -> ElementRef;
}

impl HtmlParserHelper for Html {
    fn select_first(&self, selector: &str) -> ElementRef {
        self.select(&get_selector(selector)).next().unwrap()
    }
}

impl<'a> HtmlParserHelper for ElementRef<'a> {
    fn select_first(&self, selector: &str) -> ElementRef {
        self.select(&get_selector(selector)).next().unwrap()
    }
}
