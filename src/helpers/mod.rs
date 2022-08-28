pub mod element_parser;
pub mod helper_tags;
pub mod html_parser;

use scraper::Selector;

pub(crate) fn get_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}
