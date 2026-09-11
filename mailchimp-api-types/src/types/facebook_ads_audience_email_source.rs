pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FacebookAdsAudienceEmailSource {
    /// Is the source reference a segment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_segment: Option<bool>,
    /// Associated list name to the source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// Email source name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Segment type if this source is tied to a segment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<String>,
    /// Type of the email source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl FacebookAdsAudienceEmailSource {
    pub fn builder() -> FacebookAdsAudienceEmailSourceBuilder {
        <FacebookAdsAudienceEmailSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdsAudienceEmailSourceBuilder {
    is_segment: Option<bool>,
    list_name: Option<String>,
    name: Option<String>,
    segment_type: Option<String>,
    r#type: Option<String>,
}

impl FacebookAdsAudienceEmailSourceBuilder {
    pub fn is_segment(mut self, value: bool) -> Self {
        self.is_segment = Some(value);
        self
    }

    pub fn list_name(mut self, value: impl Into<String>) -> Self {
        self.list_name = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn segment_type(mut self, value: impl Into<String>) -> Self {
        self.segment_type = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdsAudienceEmailSource`].
    pub fn build(self) -> Result<FacebookAdsAudienceEmailSource, BuildError> {
        Ok(FacebookAdsAudienceEmailSource {
            is_segment: self.is_segment,
            list_name: self.list_name,
            name: self.name,
            segment_type: self.segment_type,
            r#type: self.r#type,
        })
    }
}
