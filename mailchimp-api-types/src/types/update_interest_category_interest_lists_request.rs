pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateInterestCategoryInterestListsRequest {
    /// The display order for interests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// The name of the interest. This can be shown publicly on a subscription form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateInterestCategoryInterestListsRequest {
    pub fn builder() -> UpdateInterestCategoryInterestListsRequestBuilder {
        <UpdateInterestCategoryInterestListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateInterestCategoryInterestListsRequestBuilder {
    display_order: Option<i64>,
    name: Option<String>,
}

impl UpdateInterestCategoryInterestListsRequestBuilder {
    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateInterestCategoryInterestListsRequest`].
    pub fn build(self) -> Result<UpdateInterestCategoryInterestListsRequest, BuildError> {
        Ok(UpdateInterestCategoryInterestListsRequest {
            display_order: self.display_order,
            name: self.name,
        })
    }
}

