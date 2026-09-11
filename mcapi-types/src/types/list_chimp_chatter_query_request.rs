pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-chimp-chatter
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListChimpChatterQueryRequest {
    /// The number of records to return. Default value is 10. Maximum value is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl ListChimpChatterQueryRequest {
    pub fn builder() -> ListChimpChatterQueryRequestBuilder {
        <ListChimpChatterQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListChimpChatterQueryRequestBuilder {
    count: Option<i64>,
    offset: Option<i64>,
}

impl ListChimpChatterQueryRequestBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListChimpChatterQueryRequest`].
    pub fn build(self) -> Result<ListChimpChatterQueryRequest, BuildError> {
        Ok(ListChimpChatterQueryRequest {
            count: self.count,
            offset: self.offset,
        })
    }
}

