pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSegmentRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<SegmentType>,
    /// Match type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<CreateSegmentRequestOptionsMatch>,
}

impl CreateSegmentRequestOptions {
    pub fn builder() -> CreateSegmentRequestOptionsBuilder {
        <CreateSegmentRequestOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSegmentRequestOptionsBuilder {
    conditions: Option<SegmentType>,
    r#match: Option<CreateSegmentRequestOptionsMatch>,
}

impl CreateSegmentRequestOptionsBuilder {
    pub fn conditions(mut self, value: SegmentType) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn r#match(mut self, value: CreateSegmentRequestOptionsMatch) -> Self {
        self.r#match = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSegmentRequestOptions`].
    pub fn build(self) -> Result<CreateSegmentRequestOptions, BuildError> {
        Ok(CreateSegmentRequestOptions {
            conditions: self.conditions,
            r#match: self.r#match,
        })
    }
}
