pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateInterestCategoryRequest {
    /// The order that the categories are displayed in the list. Lower numbers display first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// The text description of this category. This field appears on signup forms and is often phrased as a question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Determines how this category’s interests appear on signup forms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UpdateInterestCategoryRequestType>,
}

impl UpdateInterestCategoryRequest {
    pub fn builder() -> UpdateInterestCategoryRequestBuilder {
        <UpdateInterestCategoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateInterestCategoryRequestBuilder {
    display_order: Option<i64>,
    title: Option<String>,
    r#type: Option<UpdateInterestCategoryRequestType>,
}

impl UpdateInterestCategoryRequestBuilder {
    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: UpdateInterestCategoryRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateInterestCategoryRequest`].
    pub fn build(self) -> Result<UpdateInterestCategoryRequest, BuildError> {
        Ok(UpdateInterestCategoryRequest {
            display_order: self.display_order,
            title: self.title,
            r#type: self.r#type,
        })
    }
}

