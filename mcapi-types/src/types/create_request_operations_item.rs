pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequestOperationsItem {
    /// A string containing the JSON body to use with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Any HTTP headers to include with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<CreateRequestOperationsItemHeaders>,
    /// The HTTP method to use for the operation.
    pub method: CreateRequestOperationsItemMethod,
    /// An optional client-supplied id returned with the operation results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// Any request query parameters. Example parameters: {"count":10, "offset":0}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<CreateRequestOperationsItemParams>,
    /// The relative path to use for the operation.
    #[serde(default)]
    pub path: String,
}

impl CreateRequestOperationsItem {
    pub fn builder() -> CreateRequestOperationsItemBuilder {
        <CreateRequestOperationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestOperationsItemBuilder {
    body: Option<String>,
    headers: Option<CreateRequestOperationsItemHeaders>,
    method: Option<CreateRequestOperationsItemMethod>,
    operation_id: Option<String>,
    params: Option<CreateRequestOperationsItemParams>,
    path: Option<String>,
}

impl CreateRequestOperationsItemBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn headers(mut self, value: CreateRequestOperationsItemHeaders) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn method(mut self, value: CreateRequestOperationsItemMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn operation_id(mut self, value: impl Into<String>) -> Self {
        self.operation_id = Some(value.into());
        self
    }

    pub fn params(mut self, value: CreateRequestOperationsItemParams) -> Self {
        self.params = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateRequestOperationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](CreateRequestOperationsItemBuilder::method)
    /// - [`path`](CreateRequestOperationsItemBuilder::path)
    pub fn build(self) -> Result<CreateRequestOperationsItem, BuildError> {
        Ok(CreateRequestOperationsItem {
            body: self.body,
            headers: self.headers,
            method: self.method.ok_or_else(|| BuildError::missing_field("method"))?,
            operation_id: self.operation_id,
            params: self.params,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
