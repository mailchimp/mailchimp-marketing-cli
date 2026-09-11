pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSegmentListsRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<SegmentType>,
    /// Match type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<CreateSegmentListsRequestOptionsMatch>,
}

impl CreateSegmentListsRequestOptions {
    pub fn builder() -> CreateSegmentListsRequestOptionsBuilder {
        <CreateSegmentListsRequestOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSegmentListsRequestOptionsBuilder {
    conditions: Option<SegmentType>,
    r#match: Option<CreateSegmentListsRequestOptionsMatch>,
}

impl CreateSegmentListsRequestOptionsBuilder {
    pub fn conditions(mut self, value: SegmentType) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn r#match(mut self, value: CreateSegmentListsRequestOptionsMatch) -> Self {
        self.r#match = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSegmentListsRequestOptions`].
    pub fn build(self) -> Result<CreateSegmentListsRequestOptions, BuildError> {
        Ok(CreateSegmentListsRequestOptions {
            conditions: self.conditions,
            r#match: self.r#match,
        })
    }
}
