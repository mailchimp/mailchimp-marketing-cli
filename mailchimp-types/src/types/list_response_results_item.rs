pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListResponseResultsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<Campaigns>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

impl ListResponseResultsItem {
    pub fn builder() -> ListResponseResultsItemBuilder {
        <ListResponseResultsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseResultsItemBuilder {
    campaign: Option<Campaigns>,
    snippet: Option<String>,
}

impl ListResponseResultsItemBuilder {
    pub fn campaign(mut self, value: Campaigns) -> Self {
        self.campaign = Some(value);
        self
    }

    pub fn snippet(mut self, value: impl Into<String>) -> Self {
        self.snippet = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListResponseResultsItem`].
    pub fn build(self) -> Result<ListResponseResultsItem, BuildError> {
        Ok(ListResponseResultsItem {
            campaign: self.campaign,
            snippet: self.snippet,
        })
    }
}
