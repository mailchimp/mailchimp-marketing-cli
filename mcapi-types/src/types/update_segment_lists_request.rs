pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateSegmentListsRequest {
    /// The name of the segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<UpdateSegmentListsRequestOptions>,
    /// An array of emails to be used for a static segment. Any emails provided that are not present on the list will be ignored. Passing an empty array for an existing static segment will reset that segment and remove all members. This field cannot be provided with the `options` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_segment: Option<Vec<String>>,
}

impl UpdateSegmentListsRequest {
    pub fn builder() -> UpdateSegmentListsRequestBuilder {
        <UpdateSegmentListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSegmentListsRequestBuilder {
    name: Option<String>,
    options: Option<UpdateSegmentListsRequestOptions>,
    static_segment: Option<Vec<String>>,
}

impl UpdateSegmentListsRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn options(mut self, value: UpdateSegmentListsRequestOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn static_segment(mut self, value: Vec<String>) -> Self {
        self.static_segment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSegmentListsRequest`].
    pub fn build(self) -> Result<UpdateSegmentListsRequest, BuildError> {
        Ok(UpdateSegmentListsRequest {
            name: self.name,
            options: self.options,
            static_segment: self.static_segment,
        })
    }
}

