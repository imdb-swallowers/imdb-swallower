use scraper::{ElementRef, Html};

use super::get_selector;

pub(crate) trait HtmlParserHelper {
    fn select_first(&self, selector: &str) -> ElementRef;
    fn select_all(&self, selector: &str) -> Vec<ElementRef>;
}

impl HtmlParserHelper for Html {
    fn select_first(&self, selector: &str) -> ElementRef {
        self.select(&get_selector(selector)).next().unwrap()
    }

    fn select_all(&self, selector: &str) -> Vec<ElementRef> {
        self.select(&get_selector(selector))
            .collect::<Vec<ElementRef>>()
    }
}

impl<'a> HtmlParserHelper for ElementRef<'a> {
    fn select_first(&self, selector: &str) -> ElementRef {
        self.select(&get_selector(selector)).next().unwrap()
    }

    fn select_all(&self, selector: &str) -> Vec<ElementRef> {
        self.select(&get_selector(selector))
            .collect::<Vec<ElementRef>>()
    }
}
