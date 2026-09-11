pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSearchCampaignsResponseResultsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<Campaigns>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

impl ListSearchCampaignsResponseResultsItem {
    pub fn builder() -> ListSearchCampaignsResponseResultsItemBuilder {
        <ListSearchCampaignsResponseResultsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSearchCampaignsResponseResultsItemBuilder {
    campaign: Option<Campaigns>,
    snippet: Option<String>,
}

impl ListSearchCampaignsResponseResultsItemBuilder {
    pub fn campaign(mut self, value: Campaigns) -> Self {
        self.campaign = Some(value);
        self
    }

    pub fn snippet(mut self, value: impl Into<String>) -> Self {
        self.snippet = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListSearchCampaignsResponseResultsItem`].
    pub fn build(self) -> Result<ListSearchCampaignsResponseResultsItem, BuildError> {
        Ok(ListSearchCampaignsResponseResultsItem {
            campaign: self.campaign,
            snippet: self.snippet,
        })
    }
}
