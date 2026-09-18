pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateInterestCategoryRequest {
    /// The order that the categories are displayed in the list. Lower numbers display first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// The text description of this category. This field appears on signup forms and is often phrased as a question.
    #[serde(default)]
    pub title: String,
    /// Determines how this category’s interests appear on signup forms.
    pub r#type: CreateInterestCategoryRequestType,
}

impl CreateInterestCategoryRequest {
    pub fn builder() -> CreateInterestCategoryRequestBuilder {
        <CreateInterestCategoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInterestCategoryRequestBuilder {
    display_order: Option<i64>,
    title: Option<String>,
    r#type: Option<CreateInterestCategoryRequestType>,
}

impl CreateInterestCategoryRequestBuilder {
    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: CreateInterestCategoryRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateInterestCategoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](CreateInterestCategoryRequestBuilder::title)
    /// - [`r#type`](CreateInterestCategoryRequestBuilder::r#type)
    pub fn build(self) -> Result<CreateInterestCategoryRequest, BuildError> {
        Ok(CreateInterestCategoryRequest {
            display_order: self.display_order,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

