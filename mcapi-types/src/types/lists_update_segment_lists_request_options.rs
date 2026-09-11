pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateSegmentListsRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<SegmentType>,
    /// Match type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<UpdateSegmentListsRequestOptionsMatch>,
}

impl UpdateSegmentListsRequestOptions {
    pub fn builder() -> UpdateSegmentListsRequestOptionsBuilder {
        <UpdateSegmentListsRequestOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSegmentListsRequestOptionsBuilder {
    conditions: Option<SegmentType>,
    r#match: Option<UpdateSegmentListsRequestOptionsMatch>,
}

impl UpdateSegmentListsRequestOptionsBuilder {
    pub fn conditions(mut self, value: SegmentType) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn r#match(mut self, value: UpdateSegmentListsRequestOptionsMatch) -> Self {
        self.r#match = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSegmentListsRequestOptions`].
    pub fn build(self) -> Result<UpdateSegmentListsRequestOptions, BuildError> {
        Ok(UpdateSegmentListsRequestOptions {
            conditions: self.conditions,
            r#match: self.r#match,
        })
    }
}
