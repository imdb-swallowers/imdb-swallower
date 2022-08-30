use std::collections::HashMap;
use std::str::FromStr;

use hyper::Uri;
use scraper::{ElementRef, Html};

use crate::helpers::helper_tags::ATag;
use crate::helpers::{
    element_parser::ElementParserHelper, get_selector, html_parser::HtmlParserHelper,
};
use crate::ImdbSearchEngine;

use super::By;

#[derive(Debug, Clone)]
pub struct PeopleInfo {
    name: String,
    link: String,
    role: String,
}

impl PeopleInfo {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn link(&self) -> &str {
        self.link.as_ref()
    }

    pub fn role(&self) -> &str {
        self.role.as_ref()
    }
}

pub struct TitleSearchItem {
    title: ATag,
    image_url: String,
    years: String,
    info: String,
    rating: String,
    summery: String,
    peoples_info: HashMap<String, Vec<PeopleInfo>>,
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

    pub fn get_by_role(&self, role: &str) -> Vec<PeopleInfo> {
        self.peoples_info.get(role).unwrap_or(&vec![]).to_vec()
    }

    pub fn get_by_roles(&self, roles: Vec<&str>) -> Vec<PeopleInfo> {
        let mut result = vec![];
        for role in roles {
            result.extend(self.get_by_role(role))
        }

        result
    }

    pub fn directors(&self) -> Vec<PeopleInfo> {
        self.get_by_roles(vec!["Director", "Directors"])
    }

    pub fn stars(&self) -> Vec<PeopleInfo> {
        self.get_by_roles(vec!["Star", "Stars"])
    }

    pub fn peoples_info(&self) -> &HashMap<String, Vec<PeopleInfo>> {
        &self.peoples_info
    }

    pub fn join_peoples<P, F>(
        &self,
        parse_group_name: P,
        separator: &str,
        people_to_str: F,
        people_separator: &str,
    ) -> String
    where
        P: Fn(&String) -> String,
        F: Fn(&PeopleInfo) -> String,
    {
        let mut pre = vec![];
        for group in self.peoples_info.iter() {
            pre.push(format!(
                "{}{}",
                parse_group_name(group.0),
                group
                    .1
                    .iter()
                    .map(|p| people_to_str(p))
                    .collect::<Vec<String>>()
                    .join(people_separator),
            ));
        }

        pre.join(separator)
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

    fn parse_people_tag(ele: ElementRef) -> HashMap<String, Vec<PeopleInfo>> {
        let mut people_roles = HashMap::new();

        let joined = ele
            .text()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect::<Vec<String>>();

        let mut current_role = String::new();
        for item in joined {
            if item.ends_with(":") {
                current_role = item.split(":").next().unwrap().to_string();
                people_roles.insert(current_role.clone(), vec![]);
            } else {
                if item == "," || item == "|" {
                    continue;
                }

                let val = people_roles.get_mut(&current_role).unwrap();
                val.push(item)
            }
        }

        let mut peoples_link = HashMap::new();

        let a_tags = ele
            .select_all("a")
            .iter()
            .filter_map(|e| e.parse_a_tag())
            .collect::<Vec<ATag>>();

        for a_tag in a_tags {
            peoples_link.insert(a_tag.text().to_string(), a_tag.link.to_string());
        }

        let mut people_info = HashMap::new();

        for (role, names) in people_roles {
            let mut names_in_role = vec![];
            for name in names {
                match peoples_link.remove(&name) {
                    Some(link) => {
                        names_in_role.push(PeopleInfo {
                            name,
                            link,
                            role: role.clone(),
                        });
                    }
                    None => (),
                };
            }

            people_info.insert(role, names_in_role);
        }

        people_info
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
            if let Some(contents) = contents {
                let img_ele = element.select_first("div.lister-item-image>a>img").unwrap();
                let name_ele = contents.select_first("h3.lister-item-header>a").unwrap();
                let year_ele = contents
                    .select_first("h3.lister-item-header>span.lister-item-year")
                    .unwrap();

                let mut p_text_muted_eles = contents.select(&item_content_p_selector);
                let info_ele = p_text_muted_eles.next().unwrap();
                let summery_ele = p_text_muted_eles.next().unwrap();
                let rating = match contents.select(&item_content_rating_bar_selector).next() {
                    Some(ele) => ele.value().attr("data-value").unwrap(),
                    None => "No rating.",
                }
                .to_string();
                let image_url = img_ele.value().attr("src").unwrap().to_string();
                let peoples = p_text_muted_eles.next().unwrap(); // always exists ...

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
                    peoples_info: ByTitle::parse_people_tag(peoples),
                });
            }
        }

        TitleSearch { items }
    }
}

#[cfg(test)]
pub mod test {
    use scraper::Html;

    use super::ByTitle;

    #[test]
    fn test_parse_people() {
        ByTitle::parse_people_tag(
            Html::parse_fragment(
                r#"
            <p class="">
    Director:
<a href="/name/nm1918751/?ref_=adv_li_dr_0">Christopher Robin Collins</a>
                 <span class="ghost">|</span> 
    Stars:
<a href="/name/nm5086781/?ref_=adv_li_st_0">Five Star</a>, 
<a href="/name/nm1711245/?ref_=adv_li_st_1">Delroy Pearson</a>, 
<a href="/name/nm1269772/?ref_=adv_li_st_2">Denise Pearson</a>, 
<a href="/name/nm1711246/?ref_=adv_li_st_3">Doris Pearson</a>
    </p>
            "#,
            )
            .root_element(),
        );
    }
}
