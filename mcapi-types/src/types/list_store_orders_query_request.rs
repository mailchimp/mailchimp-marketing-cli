pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-store-orders
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListStoreOrdersQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// The number of records to return. Default value is 10. Maximum value is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Restrict results to orders made by a specific customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_outreach: Option<bool>,
    /// Restrict results to orders with a specific `campaign_id` value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// Restrict results to orders with a specific `outreach_id` value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outreach_id: Option<String>,
}

impl ListStoreOrdersQueryRequest {
    pub fn builder() -> ListStoreOrdersQueryRequestBuilder {
        <ListStoreOrdersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreOrdersQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    customer_id: Option<String>,
    has_outreach: Option<bool>,
    campaign_id: Option<String>,
    outreach_id: Option<String>,
}

impl ListStoreOrdersQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn has_outreach(mut self, value: bool) -> Self {
        self.has_outreach = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn outreach_id(mut self, value: impl Into<String>) -> Self {
        self.outreach_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListStoreOrdersQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListStoreOrdersQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListStoreOrdersQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListStoreOrdersQueryRequest, BuildError> {
        Ok(ListStoreOrdersQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            customer_id: self.customer_id,
            has_outreach: self.has_outreach,
            campaign_id: self.campaign_id,
            outreach_id: self.outreach_id,
        })
    }
}

