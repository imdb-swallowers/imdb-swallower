use scraper::ElementRef;

use super::helper_tags::ATag;

pub(crate) trait ElementParserHelper {
    fn parse_a_tag(&self) -> Option<ATag>;
}

impl<'a> ElementParserHelper for ElementRef<'a> {
    fn parse_a_tag(&self) -> Option<ATag> {
        let element = self.value();
        let link = element.attr("href")?.to_string();
        let text = self.inner_html();

        Some(ATag { text, link })
    }
}
