pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateInterestCategoryInterestRequest {
    /// The display order for interests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// The name of the interest. This can be shown publicly on a subscription form.
    #[serde(default)]
    pub name: String,
}

impl CreateInterestCategoryInterestRequest {
    pub fn builder() -> CreateInterestCategoryInterestRequestBuilder {
        <CreateInterestCategoryInterestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInterestCategoryInterestRequestBuilder {
    display_order: Option<i64>,
    name: Option<String>,
}

impl CreateInterestCategoryInterestRequestBuilder {
    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateInterestCategoryInterestRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateInterestCategoryInterestRequestBuilder::name)
    pub fn build(self) -> Result<CreateInterestCategoryInterestRequest, BuildError> {
        Ok(CreateInterestCategoryInterestRequest {
            display_order: self.display_order,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

