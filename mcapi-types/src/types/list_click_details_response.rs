pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of URLs and unique IDs included in HTML and plain-text versions of a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListClickDetailsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListClickDetailsResponseLinksItem>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// An array of objects, each representing a specific URL contained in the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls_clicked: Option<Vec<ClickDetailReport>>,
}

impl ListClickDetailsResponse {
    pub fn builder() -> ListClickDetailsResponseBuilder {
        <ListClickDetailsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClickDetailsResponseBuilder {
    links: Option<Vec<ListClickDetailsResponseLinksItem>>,
    campaign_id: Option<String>,
    total_items: Option<i64>,
    urls_clicked: Option<Vec<ClickDetailReport>>,
}

impl ListClickDetailsResponseBuilder {
    pub fn links(mut self, value: Vec<ListClickDetailsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn urls_clicked(mut self, value: Vec<ClickDetailReport>) -> Self {
        self.urls_clicked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListClickDetailsResponse`].
    pub fn build(self) -> Result<ListClickDetailsResponse, BuildError> {
        Ok(ListClickDetailsResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            total_items: self.total_items,
            urls_clicked: self.urls_clicked,
        })
    }
}
