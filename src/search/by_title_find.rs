use std::str::FromStr;

use hyper::Uri;

use crate::helpers::{
    element_parser::ElementParserHelper, get_selector, html_parser::HtmlParserHelper,
};

use super::By;

pub struct ByTitleFind;

impl Default for ByTitleFind {
    fn default() -> Self {
        Self {}
    }
}

pub struct ByTitleFoundItem {
    title: String,
    link: String,
    img_uri: String,
}

impl ByTitleFoundItem {
    fn get_title_id(&self) -> String {
        self.link.split("/").nth(2).unwrap().to_string()
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn link(&self) -> &str {
        self.link.as_ref()
    }

    pub fn img_uri(&self) -> &str {
        self.img_uri.as_ref()
    }
}

pub struct ByTitleFound {
    items: Vec<ByTitleFoundItem>,
}

impl ByTitleFound {
    pub fn items(&self) -> &[ByTitleFoundItem] {
        self.items.as_ref()
    }
}

impl By for ByTitleFind {
    type ParseResult = ByTitleFound;

    fn get_uri(&self, engine: &crate::prelude::ImdbSearchEngine, query: &str) -> hyper::Uri {
        Uri::from_str(format!("{}/find?s=tt&q={}", engine.base_uri(), query).as_str()).unwrap()
    }

    fn parse_result(&self, html: scraper::Html) -> Self::ParseResult {
        let items_selector = get_selector("#main > div > div.findSection > table > tbody > tr");
        let td_selector = get_selector("td");

        let mut items = vec![];
        for tr in html.select(&items_selector) {
            // We have tr here ...
            // Every tr has exactly 2 tds.
            // First > Photo
            // Second > Text

            let mut tds = tr.select(&td_selector);

            let first_ele = match tds.next() {
                Some(f) => f,
                None => {
                    // What ???
                    continue;
                }
            };

            let second_ele = match tds.next() {
                Some(s) => s,
                None => {
                    // Whaaaat ???
                    continue;
                }
            };

            let img_ele = match first_ele.select_first("a > img") {
                Some(img) => img,
                None => continue,
            };

            let image_src = if let Some(src) = img_ele.value().attr("src") {
                src
            } else {
                continue;
            };

            let text_a_ele = if let Some(ele) = second_ele.select_first("a") {
                ele
            } else {
                continue;
            };

            let a_tag = match text_a_ele.parse_a_tag() {
                Some(a) => a,
                None => continue,
            };

            items.push(ByTitleFoundItem {
                title: a_tag.text,
                link: a_tag.link,
                img_uri: image_src.to_string(),
            })
        }

        ByTitleFound { items }
    }
}

#[cfg(test)]
pub mod tests {
    use scraper::Html;

    use crate::search::By;

    use super::ByTitleFind;

    #[test]
    pub fn test_by_title_parser() {
        let html = Html::parse_fragment(
            r#"
        <div id="main">
    <div class="article">
        <h1 class="findHeader">Displaying 200 results for <span class="findSearchTerm">"Starwars"</span></h1>

        <div id="findSubHeader"><span id="findSubHeaderLabel">Search category: </span>
            All Titles

        </div>

        <div class="findSection">
            <h3 class="findSectionHeader"><a name="tt"></a>Titles</h3>
            <table class="findList">
                <tbody>
                    <tr class="findResult odd">
                        <td class="primary_photo"> <a href="/title/tt9336300/?ref_=fn_tt_tt_1"><img
                                    src="https://m.media-amazon.com/images/M/MV5BNTI5OTBhMGYtNTZlNS00MjMzLTk5NTEtZDZkODM5YjYzYmE5XkEyXkFqcGdeQXVyMzU0OTU0MzY@._V1_UY44_CR2,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt9336300/?ref_=fn_tt_tt_1">Starwars: Goretech</a>
                            (2018) </td>
                    </tr>
                    <tr class="findResult even">
                        <td class="primary_photo"> <a href="/title/tt11953086/?ref_=fn_tt_tt_2"><img
                                    src="https://m.media-amazon.com/images/M/MV5BNGFlOWMxNTktMjJiOC00ZDczLWFiNDgtYTdmZTE5Nzc4Mjk4XkEyXkFqcGdeQXVyNzExMzc0MDg@._V1_UY44_CR23,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt11953086/?ref_=fn_tt_tt_2">StarWarsOnly</a> (2016)
                            (TV Series) </td>
                    </tr>
                    <tr class="findResult odd">
                        <td class="primary_photo"> <a href="/title/tt10763556/?ref_=fn_tt_tt_3"><img
                                    src="https://m.media-amazon.com/images/M/MV5BODNlZTQ0ZTYtNzk3Ni00ODA2LWJjNDItNzQ5MDhjZTNiYTQ1XkEyXkFqcGdeQXVyNjU1OTg4OTM@._V1_UY44_CR23,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt10763556/?ref_=fn_tt_tt_3">StarWarsTFA Spoiler Review
                                pt 12/12 - Favorite Lines and Moments</a> (2015) (TV Episode) <br> <small>- Season 1
                                <span class="ghost">|</span> Episode 13 </small> <br><small>- <a
                                    href="/title/tt10763450/?ref_=fn_tt_tt_3a">Blind Wave Movie Reviews</a> (2015) (TV
                                Series) </small> </td>
                    </tr>
                    <tr class="findResult even">
                        <td class="primary_photo"> <a href="/title/tt14640026/?ref_=fn_tt_tt_4"><img
                                    src="https://m.media-amazon.com/images/M/MV5BNGFlOWMxNTktMjJiOC00ZDczLWFiNDgtYTdmZTE5Nzc4Mjk4XkEyXkFqcGdeQXVyNzExMzc0MDg@._V1_UY44_CR23,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt14640026/?ref_=fn_tt_tt_4">Bad Batch Episode 2 Cut
                                and Run Review!</a> (2021) (TV Episode) <br><small>- <a
                                    href="/title/tt11953086/?ref_=fn_tt_tt_4a">StarWarsOnly</a> (2016) (TV Series)
                            </small> </td>
                    </tr>
                    <tr class="findResult odd">
                        <td class="primary_photo"> <a href="/title/tt10763540/?ref_=fn_tt_tt_5"><img
                                    src="https://m.media-amazon.com/images/M/MV5BM2FiNjliYzMtNTc1MC00MWIyLWJiOGMtYmZkZGYzZjYwMWU4XkEyXkFqcGdeQXVyNjU1OTg4OTM@._V1_UY44_CR23,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt10763540/?ref_=fn_tt_tt_5">StarWarsTFA Spoiler Review
                                pt 10/12 - Visions, Jedi, and Snoke</a> (2015) (TV Episode) <br> <small>- Season 1 <span
                                    class="ghost">|</span> Episode 11 </small> <br><small>- <a
                                    href="/title/tt10763450/?ref_=fn_tt_tt_5a">Blind Wave Movie Reviews</a> (2015) (TV
                                Series) </small> </td>
                    </tr>
                    <tr class="findResult even">
                        <td class="primary_photo"> <a href="/title/tt0076759/?ref_=fn_tt_tt_6"><img
                                    src="https://m.media-amazon.com/images/M/MV5BNzg4MjQxNTQtZmI5My00YjMwLWJlMjUtMmJlY2U2ZWFlNzY1XkEyXkFqcGdeQXVyODk4OTc3MTY@._V1_UX32_CR0,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt0076759/?ref_=fn_tt_tt_6">Star Wars</a> (1977) </td>
                    </tr>
                    <tr class="findResult odd">
                        <td class="primary_photo"> <a href="/title/tt0458290/?ref_=fn_tt_tt_7"><img
                                    src="https://m.media-amazon.com/images/M/MV5BZWFlNzRmOTItZjY1Ni00ZjZkLTk5MDgtOGFhOTYzNWFhYzhmXkEyXkFqcGdeQXVyMDM2NDM2MQ@@._V1_UX32_CR0,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt0458290/?ref_=fn_tt_tt_7">Star Wars: The Clone
                                Wars</a> (2008) (TV Series) </td>
                    </tr>
                    <tr class="findResult even">
                        <td class="primary_photo"> <a href="/title/tt2930604/?ref_=fn_tt_tt_8"><img
                                    src="https://m.media-amazon.com/images/M/MV5BY2Q1ZTAzNzMtMzlmNy00NjdjLThhYjgtMzgxN2ZkYmFhMDIwXkEyXkFqcGdeQXVyMjg5NDMwMQ@@._V1_UX32_CR0,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt2930604/?ref_=fn_tt_tt_8">Star Wars: Rebels</a>
                            (2014) (TV Series) </td>
                    </tr>
                    <tr class="findResult odd">
                        <td class="primary_photo"> <a href="/title/tt3778644/?ref_=fn_tt_tt_9"><img
                                    src="https://m.media-amazon.com/images/M/MV5BOTM2NTI3NTc3Nl5BMl5BanBnXkFtZTgwNzM1OTQyNTM@._V1_UX32_CR0,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt3778644/?ref_=fn_tt_tt_9">Solo: A Star Wars Story</a>
                            (2018) </td>
                    </tr>
                    <tr class="findResult even">
                        <td class="primary_photo"> <a href="/title/tt3748528/?ref_=fn_tt_tt_10"><img
                                    src="https://m.media-amazon.com/images/M/MV5BMjEwMzMxODIzOV5BMl5BanBnXkFtZTgwNzg3OTAzMDI@._V1_UX32_CR0,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt3748528/?ref_=fn_tt_tt_10">Rogue One: A Star Wars
                                Story</a> (2016) </td>
                    </tr>
                    <tr class="findResult odd">
                        <td class="primary_photo"> <a href="/title/tt12708542/?ref_=fn_tt_tt_11"><img
                                    src="https://m.media-amazon.com/images/M/MV5BNDA5YzQzYmItZTE4NC00NTNkLTljZGItNDQ0YjI3MzdhZjlhXkEyXkFqcGdeQXVyMTEyMjM2NDc2._V1_UX32_CR0,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt12708542/?ref_=fn_tt_tt_11">Star Wars: The Bad
                                Batch</a> (2021) (TV Series) </td>
                    </tr>
                    <tr class="findResult even">
                        <td class="primary_photo"> <a href="/title/tt13622982/?ref_=fn_tt_tt_12"><img
                                    src="https://m.media-amazon.com/images/M/MV5BNWJiMDA4OTEtOGU0MC00ZDc5LWE5NWItNWVhMTlhNDliMjc4XkEyXkFqcGdeQXVyMTEyMjM2NDc2._V1_UY44_CR1,0,32,44_AL_.jpg"></a>
                        </td>
                        <td class="result_text"> <a href="/title/tt13622982/?ref_=fn_tt_tt_12">Star Wars: Visions</a>
                            (2021) (TV Series) </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</div>
        "#,
        );

        let by_title_find = ByTitleFind::default();
        let result = by_title_find.parse_result(html);

        let first = result.items.first().unwrap();
        let title_id = first.get_title_id();

        println!("{}", title_id);
    }
}
