use hyper::Uri;
use scraper::Html;

use crate::ImdbSearchEngine;

pub mod by;
pub mod by_title;
pub mod results;

pub trait By {
    type ParseResult;

    fn get_uri(&self, engine: &ImdbSearchEngine, query: &str) -> Uri;

    fn parse_result(&self, html: Html) -> Self::ParseResult;
}
