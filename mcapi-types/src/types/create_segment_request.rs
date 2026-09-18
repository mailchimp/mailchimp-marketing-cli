pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSegmentRequest {
    /// The name of the segment.
    #[serde(default)]
    pub name: String,
    /// The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateSegmentRequestOptions>,
    /// An array of emails to be used for a static segment. Any emails provided that are not present on the list will be ignored. Passing an empty array will create a static segment without any subscribers. This field cannot be provided with the options field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_segment: Option<Vec<String>>,
}

impl CreateSegmentRequest {
    pub fn builder() -> CreateSegmentRequestBuilder {
        <CreateSegmentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSegmentRequestBuilder {
    name: Option<String>,
    options: Option<CreateSegmentRequestOptions>,
    static_segment: Option<Vec<String>>,
}

impl CreateSegmentRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn options(mut self, value: CreateSegmentRequestOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn static_segment(mut self, value: Vec<String>) -> Self {
        self.static_segment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSegmentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateSegmentRequestBuilder::name)
    pub fn build(self) -> Result<CreateSegmentRequest, BuildError> {
        Ok(CreateSegmentRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            options: self.options,
            static_segment: self.static_segment,
        })
    }
}

