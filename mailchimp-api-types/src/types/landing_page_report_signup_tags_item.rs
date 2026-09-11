pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPageReportSignupTagsItem {
    /// The unique id for the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<i64>,
    /// The name of the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
}

impl LandingPageReportSignupTagsItem {
    pub fn builder() -> LandingPageReportSignupTagsItemBuilder {
        <LandingPageReportSignupTagsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageReportSignupTagsItemBuilder {
    tag_id: Option<i64>,
    tag_name: Option<String>,
}

impl LandingPageReportSignupTagsItemBuilder {
    pub fn tag_id(mut self, value: i64) -> Self {
        self.tag_id = Some(value);
        self
    }

    pub fn tag_name(mut self, value: impl Into<String>) -> Self {
        self.tag_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LandingPageReportSignupTagsItem`].
    pub fn build(self) -> Result<LandingPageReportSignupTagsItem, BuildError> {
        Ok(LandingPageReportSignupTagsItem {
            tag_id: self.tag_id,
            tag_name: self.tag_name,
        })
    }
}
