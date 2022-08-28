use std::str::FromStr;

use hyper::Uri;
use scraper::Html;

use crate::helpers::helper_tags::ATag;
use crate::helpers::{
    element_parser::ElementParserHelper, get_selector, html_parser::HtmlParserHelper,
};
use crate::ImdbSearchEngine;

use super::By;

pub struct TitleSearchItem {
    title: ATag,
    image_url: String,
    years: String,
    info: String,
    rating: String,
    summery: String,
}

impl TitleSearchItem {
    pub fn title(&self) -> &ATag {
        &self.title
    }

    pub fn years(&self) -> &str {
        self.years.as_ref()
    }

    pub fn info(&self) -> &str {
        self.info.as_ref()
    }

    pub fn rating(&self) -> &str {
        self.rating.as_ref()
    }

    pub fn summery(&self) -> &str {
        self.summery.as_ref()
    }

    pub fn image_url(&self) -> &str {
        self.image_url.as_ref()
    }
}

impl ToString for TitleSearchItem {
    fn to_string(&self) -> String {
        format!(
            "{} {}\n- {}\n- Rating: {}\n- {}",
            self.title.text(),
            self.years,
            self.info,
            self.rating,
            self.summery
        )
    }
}

pub struct TitleSearch {
    items: Vec<TitleSearchItem>,
}

impl TitleSearch {
    pub fn items(&self) -> &[TitleSearchItem] {
        self.items.as_ref()
    }
}

pub struct ByTitle {
    pub(crate) start: u16,
    pub(crate) count: u8,
}

impl ByTitle {
    pub fn new(start: u16, count: u8) -> Self {
        Self { start, count }
    }
}

impl Default for ByTitle {
    fn default() -> Self {
        Self {
            start: 1,
            count: 10,
        }
    }
}

impl By for ByTitle {
    type ParseResult = TitleSearch;

    fn get_uri(&self, engine: &ImdbSearchEngine, query: &str) -> Uri {
        Uri::from_str(
            format!(
                "{}/search/title/?title={}&start={}&count={}",
                engine.base_uri(),
                query,
                self.start,
                self.count
            )
            .as_str(),
        )
        .unwrap()
    }

    fn parse_result(&self, html: Html) -> Self::ParseResult {
        let item_list_selector = get_selector("div.lister-list>div");
        let item_content_p_selector = get_selector("p");
        let spans_selector = get_selector("span");
        let item_content_rating_bar_selector =
            get_selector("div.ratings-bar>div.ratings-imdb-rating");

        let mut items = vec![];

        for element in html.select(&item_list_selector) {
            let contents = element.select_first("div.lister-item-content");
            let img_ele = element.select_first("div.lister-item-image>a>img");
            let name_ele = contents.select_first("h3.lister-item-header>a");
            let year_ele = contents.select_first("h3.lister-item-header>span.lister-item-year");

            let mut p_text_muted_eles = contents.select(&item_content_p_selector);
            let info_ele = p_text_muted_eles.next().unwrap();
            let summery_ele = p_text_muted_eles.next().unwrap();
            let rating = match contents.select(&item_content_rating_bar_selector).next() {
                Some(ele) => ele.value().attr("data-value").unwrap(),
                None => "No rating.",
            }
            .to_string();
            let image_url = img_ele.value().attr("src").unwrap().to_string();
            // let peoples = p_text_muted_eles.next().unwrap();
            // let votes = p_text_muted_eles.next().unwrap();

            let info_spans = info_ele
                .select(&spans_selector)
                .map(|e| e.inner_html().trim().to_string())
                .collect::<Vec<String>>();

            let info_spans_str = info_spans.join(" ");

            items.push(TitleSearchItem {
                title: name_ele.parse_a_tag().unwrap(),
                image_url,
                years: year_ele.inner_html(),
                info: info_spans_str,
                rating,
                summery: summery_ele.inner_html().trim().to_string(),
            });
        }

        TitleSearch { items }
    }
}
