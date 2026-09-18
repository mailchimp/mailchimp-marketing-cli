pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateInterestCategoryInterestRequest {
    /// The display order for interests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// The name of the interest. This can be shown publicly on a subscription form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateInterestCategoryInterestRequest {
    pub fn builder() -> UpdateInterestCategoryInterestRequestBuilder {
        <UpdateInterestCategoryInterestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateInterestCategoryInterestRequestBuilder {
    display_order: Option<i64>,
    name: Option<String>,
}

impl UpdateInterestCategoryInterestRequestBuilder {
    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateInterestCategoryInterestRequest`].
    pub fn build(self) -> Result<UpdateInterestCategoryInterestRequest, BuildError> {
        Ok(UpdateInterestCategoryInterestRequest {
            display_order: self.display_order,
            name: self.name,
        })
    }
}

