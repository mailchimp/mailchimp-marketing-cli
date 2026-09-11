use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct EcommerceClient {
    pub http_client: HttpClient,
}

impl EcommerceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get information about the e-commerce endpoint's resources.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client.ecommerce.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "3.0/ecommerce", None, None, options)
            .await
    }

    /// Get information about an account's orders.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `campaign_id` - Restrict results to orders with a specific `campaign_id` value.
    /// * `outreach_id` - Restrict results to orders with a specific `outreach_id` value.
    /// * `customer_id` - Restrict results to orders made by a specific customer.
    /// * `has_outreach` - Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_orders(
    ///             &ListOrdersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 campaign_id: None,
    ///                 outreach_id: None,
    ///                 customer_id: None,
    ///                 has_outreach: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_orders(
        &self,
        request: &ListOrdersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOrdersEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/ecommerce/orders",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("campaign_id", request.campaign_id.clone())
                    .string("outreach_id", request.outreach_id.clone())
                    .string("customer_id", request.customer_id.clone())
                    .bool("has_outreach", request.has_outreach.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_orders_paginated(
        &self,
        request: &ListOrdersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("campaign_id", request.campaign_id.clone())
            .string("outreach_id", request.outreach_id.clone())
            .string("customer_id", request.customer_id.clone())
            .bool("has_outreach", request.has_outreach.clone())
            .build();
        let options_clone = options.clone();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            "3.0/ecommerce/orders",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("orders")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Get information about all stores in the account.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_stores(
    ///             &ListStoresQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_stores(
        &self,
        request: &ListStoresQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoresEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/ecommerce/stores",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_stores_paginated(
        &self,
        request: &ListStoresQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            "3.0/ecommerce/stores",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("stores")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new store to your Mailchimp account.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .create_store(
    ///             &CreateStoreEcommerceRequest {
    ///                 currency_code: "USD".to_string(),
    ///                 id: "example_store".to_string(),
    ///                 list_id: "1a2df69511".to_string(),
    ///                 name: "Freddie's Cat Hat Emporium".to_string(),
    ///                 address: None,
    ///                 domain: None,
    ///                 email_address: None,
    ///                 is_syncing: None,
    ///                 money_format: None,
    ///                 phone: None,
    ///                 platform: None,
    ///                 primary_locale: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store(
        &self,
        request: &CreateStoreEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceStore, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/ecommerce/stores",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store(
    ///             &"store_id".to_string(),
    ///             &GetStoreQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store(
        &self,
        store_id: &str,
        request: &GetStoreQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceStore, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a store. Deleting a store will also delete any associated subresources, including Customers, Orders, Products, and Carts.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store(&"store_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store(
        &self,
        store_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/ecommerce/stores/{}", store_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store(
    ///             &"store_id".to_string(),
    ///             &UpdateStoreEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store(
        &self,
        store_id: &str,
        request: &UpdateStoreEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceStore, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/ecommerce/stores/{}", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's carts.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_store_carts(
    ///             &"store_id".to_string(),
    ///             &ListStoreCartsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_carts(
        &self,
        store_id: &str,
        request: &ListStoreCartsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreCartsEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/carts", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_store_carts_paginated(
        &self,
        store_id: &str,
        request: &ListStoreCartsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let store_id_clone = store_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let store_id_for_async = store_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/ecommerce/stores/{}/carts", store_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("carts")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new cart to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .create_store_cart(
    ///             &"store_id".to_string(),
    ///             &CreateStoreCartEcommerceRequest {
    ///                 currency_code: "currency_code".to_string(),
    ///                 customer: EcommerceStoresCartsPost {
    ///                     id: "id".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 id: CreateStoreCartEcommerceRequestID::String("id".to_string()),
    ///                 lines: vec![CreateStoreCartEcommerceRequestLinesItem {
    ///                     id: "id".to_string(),
    ///                     price: CreateStoreCartEcommerceRequestLinesItemPrice::Double(1.1),
    ///                     product_id: "product_id".to_string(),
    ///                     product_variant_id: "product_variant_id".to_string(),
    ///                     quantity: 1,
    ///                 }],
    ///                 order_total: CreateStoreCartEcommerceRequestOrderTotal::Double(1.1),
    ///                 campaign_id: None,
    ///                 checkout_url: None,
    ///                 tax_total: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_cart(
        &self,
        store_id: &str,
        request: &CreateStoreCartEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCart, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/carts", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific cart.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store_cart(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &GetStoreCartQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_cart(
        &self,
        store_id: &str,
        cart_id: &str,
        request: &GetStoreCartQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCart, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/carts/{}", store_id, cart_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a cart.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store_cart(&"store_id".to_string(), &"cart_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_cart(
        &self,
        store_id: &str,
        cart_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/ecommerce/stores/{}/carts/{}", store_id, cart_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific cart.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store_cart(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &UpdateStoreCartEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_cart(
        &self,
        store_id: &str,
        cart_id: &str,
        request: &UpdateStoreCartEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCart, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/ecommerce/stores/{}/carts/{}", store_id, cart_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a cart's line items.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_store_cart_lines(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &ListStoreCartLinesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_cart_lines(
        &self,
        store_id: &str,
        cart_id: &str,
        request: &ListStoreCartLinesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreCartLinesEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/carts/{}/lines", store_id, cart_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_store_cart_lines_paginated(
        &self,
        store_id: &str,
        cart_id: &str,
        request: &ListStoreCartLinesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let store_id_clone = store_id.to_string();
        let cart_id_clone = cart_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let store_id_for_async = store_id_clone.clone();
                let cart_id_for_async = cart_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/ecommerce/stores/{}/carts/{}/lines",
                                store_id_for_async, cart_id_for_async
                            ),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("lines")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new line item to an existing cart.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .create_store_cart_line(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &CreateStoreCartLineEcommerceRequest {
    ///                 id: "id".to_string(),
    ///                 price: CreateStoreCartLineEcommerceRequestPrice::Double(1.1),
    ///                 product_id: "product_id".to_string(),
    ///                 product_variant_id: "product_variant_id".to_string(),
    ///                 quantity: 1,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_cart_line(
        &self,
        store_id: &str,
        cart_id: &str,
        request: &CreateStoreCartLineEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCartLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/carts/{}/lines", store_id, cart_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific cart line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `line_id` - The id for the line item of a cart.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store_cart_line(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &"line_id".to_string(),
    ///             &GetStoreCartLineQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_cart_line(
        &self,
        store_id: &str,
        cart_id: &str,
        line_id: &str,
        request: &GetStoreCartLineQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCartLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/carts/{}/lines/{}",
                    store_id, cart_id, line_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific cart line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `line_id` - The id for the line item of a cart.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store_cart_line(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &"line_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_cart_line(
        &self,
        store_id: &str,
        cart_id: &str,
        line_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/carts/{}/lines/{}",
                    store_id, cart_id, line_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific cart line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `line_id` - The id for the line item of a cart.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store_cart_line(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &"line_id".to_string(),
    ///             &UpdateStoreCartLineEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_cart_line(
        &self,
        store_id: &str,
        cart_id: &str,
        line_id: &str,
        request: &UpdateStoreCartLineEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCartLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/carts/{}/lines/{}",
                    store_id, cart_id, line_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's customers.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `email_address` - Restrict the response to customers with the email address.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_store_customers(
    ///             &"store_id".to_string(),
    ///             &ListStoreCustomersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 email_address: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_customers(
        &self,
        store_id: &str,
        request: &ListStoreCustomersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreCustomersEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/customers", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("email_address", request.email_address.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_store_customers_paginated(
        &self,
        store_id: &str,
        request: &ListStoreCustomersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("email_address", request.email_address.clone())
            .build();
        let options_clone = options.clone();
        let store_id_clone = store_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let store_id_for_async = store_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/ecommerce/stores/{}/customers", store_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("customers")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new customer to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .create_store_customer(
    ///             &"store_id".to_string(),
    ///             &CreateStoreCustomerEcommerceRequest {
    ///                 id: "id".to_string(),
    ///                 opt_in_status: true,
    ///                 address: None,
    ///                 company: None,
    ///                 email_address: None,
    ///                 first_name: None,
    ///                 last_name: None,
    ///                 sms_phone_number: None,
    ///                 total_spent: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_customer(
        &self,
        store_id: &str,
        request: &CreateStoreCustomerEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCustomer, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/customers", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific customer.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `customer_id` - The id for the customer of a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store_customer(
    ///             &"store_id".to_string(),
    ///             &"customer_id".to_string(),
    ///             &GetStoreCustomerQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_customer(
        &self,
        store_id: &str,
        customer_id: &str,
        request: &GetStoreCustomerQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCustomer, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/customers/{}",
                    store_id, customer_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add or update a customer.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `customer_id` - The id for the customer of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .upsert_store_customer(
    ///             &"store_id".to_string(),
    ///             &"customer_id".to_string(),
    ///             &UpsertStoreCustomerEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_store_customer(
        &self,
        store_id: &str,
        customer_id: &str,
        request: &UpsertStoreCustomerEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCustomer, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "3.0/ecommerce/stores/{}/customers/{}",
                    store_id, customer_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a customer from a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `customer_id` - The id for the customer of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store_customer(&"store_id".to_string(), &"customer_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_customer(
        &self,
        store_id: &str,
        customer_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/customers/{}",
                    store_id, customer_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a customer.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `customer_id` - The id for the customer of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store_customer(
    ///             &"store_id".to_string(),
    ///             &"customer_id".to_string(),
    ///             &EcommerceStoresCartsPatch {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_customer(
        &self,
        store_id: &str,
        customer_id: &str,
        request: &EcommerceStoresCartsPatch,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCustomer, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/customers/{}",
                    store_id, customer_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's orders.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `customer_id` - Restrict results to orders made by a specific customer.
    /// * `has_outreach` - Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad.
    /// * `campaign_id` - Restrict results to orders with a specific `campaign_id` value.
    /// * `outreach_id` - Restrict results to orders with a specific `outreach_id` value.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_store_orders(
    ///             &"store_id".to_string(),
    ///             &ListStoreOrdersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 customer_id: None,
    ///                 has_outreach: None,
    ///                 campaign_id: None,
    ///                 outreach_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_orders(
        &self,
        store_id: &str,
        request: &ListStoreOrdersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreOrdersEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/orders", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("customer_id", request.customer_id.clone())
                    .bool("has_outreach", request.has_outreach.clone())
                    .string("campaign_id", request.campaign_id.clone())
                    .string("outreach_id", request.outreach_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_store_orders_paginated(
        &self,
        store_id: &str,
        request: &ListStoreOrdersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("customer_id", request.customer_id.clone())
            .bool("has_outreach", request.has_outreach.clone())
            .string("campaign_id", request.campaign_id.clone())
            .string("outreach_id", request.outreach_id.clone())
            .build();
        let options_clone = options.clone();
        let store_id_clone = store_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let store_id_for_async = store_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/ecommerce/stores/{}/orders", store_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("orders")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new order to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .create_store_order(
    ///             &"store_id".to_string(),
    ///             &CreateStoreOrderEcommerceRequest {
    ///                 currency_code: "currency_code".to_string(),
    ///                 customer: EcommerceStoresCartsPost {
    ///                     id: "id".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 id: "id".to_string(),
    ///                 lines: vec![CreateStoreOrderEcommerceRequestLinesItem {
    ///                     discount: None,
    ///                     id: "id".to_string(),
    ///                     price: CreateStoreOrderEcommerceRequestLinesItemPrice::Double(1.1),
    ///                     product: None,
    ///                     product_id: "product_id".to_string(),
    ///                     product_variant_id: "product_variant_id".to_string(),
    ///                     quantity: 1,
    ///                 }],
    ///                 order_total: CreateStoreOrderEcommerceRequestOrderTotal::Double(1.1),
    ///                 billing_address: None,
    ///                 campaign_id: None,
    ///                 cart_id: None,
    ///                 cancelled_at_foreign: None,
    ///                 discount_total: None,
    ///                 financial_status: None,
    ///                 fulfillment_status: None,
    ///                 landing_site: None,
    ///                 order_url: None,
    ///                 outreach: None,
    ///                 processed_at_foreign: None,
    ///                 promos: None,
    ///                 shipping_address: None,
    ///                 shipping_total: None,
    ///                 tax_total: None,
    ///                 tracking_carrier: None,
    ///                 tracking_code: None,
    ///                 tracking_number: None,
    ///                 tracking_url: None,
    ///                 updated_at_foreign: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_order(
        &self,
        store_id: &str,
        request: &CreateStoreOrderEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrder, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/orders", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific order.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store_order(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &GetStoreOrderQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_order(
        &self,
        store_id: &str,
        order_id: &str,
        request: &GetStoreOrderQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrder, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/orders/{}", store_id, order_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete an order.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store_order(&"store_id".to_string(), &"order_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_order(
        &self,
        store_id: &str,
        order_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/ecommerce/stores/{}/orders/{}", store_id, order_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific order.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store_order(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &UpdateStoreOrderEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_order(
        &self,
        store_id: &str,
        order_id: &str,
        request: &UpdateStoreOrderEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrder, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/ecommerce/stores/{}/orders/{}", store_id, order_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about an order's line items.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_store_order_lines(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &ListStoreOrderLinesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_order_lines(
        &self,
        store_id: &str,
        order_id: &str,
        request: &ListStoreOrderLinesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreOrderLinesEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/orders/{}/lines",
                    store_id, order_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_store_order_lines_paginated(
        &self,
        store_id: &str,
        order_id: &str,
        request: &ListStoreOrderLinesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let store_id_clone = store_id.to_string();
        let order_id_clone = order_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let store_id_for_async = store_id_clone.clone();
                let order_id_for_async = order_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/ecommerce/stores/{}/orders/{}/lines",
                                store_id_for_async, order_id_for_async
                            ),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("lines")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new line item to an existing order.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .create_store_order_line(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &CreateStoreOrderLineEcommerceRequest {
    ///                 id: "id".to_string(),
    ///                 price: CreateStoreOrderLineEcommerceRequestPrice::Double(1.1),
    ///                 product_id: "product_id".to_string(),
    ///                 product_variant_id: "product_variant_id".to_string(),
    ///                 quantity: 1,
    ///                 discount: None,
    ///                 product: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_order_line(
        &self,
        store_id: &str,
        order_id: &str,
        request: &CreateStoreOrderLineEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrderLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/ecommerce/stores/{}/orders/{}/lines",
                    store_id, order_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific order line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `line_id` - The id for the line item of an order.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store_order_line(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &"line_id".to_string(),
    ///             &GetStoreOrderLineQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_order_line(
        &self,
        store_id: &str,
        order_id: &str,
        line_id: &str,
        request: &GetStoreOrderLineQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrderLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/orders/{}/lines/{}",
                    store_id, order_id, line_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific order line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `line_id` - The id for the line item of an order.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store_order_line(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &"line_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_order_line(
        &self,
        store_id: &str,
        order_id: &str,
        line_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/orders/{}/lines/{}",
                    store_id, order_id, line_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific order line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `line_id` - The id for the line item of an order.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store_order_line(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &"line_id".to_string(),
    ///             &UpdateStoreOrderLineEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_order_line(
        &self,
        store_id: &str,
        order_id: &str,
        line_id: &str,
        request: &UpdateStoreOrderLineEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrderLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/orders/{}/lines/{}",
                    store_id, order_id, line_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's products.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_store_products(
    ///             &"store_id".to_string(),
    ///             &ListStoreProductsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_products(
        &self,
        store_id: &str,
        request: &ListStoreProductsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreProductsEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/products", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_store_products_paginated(
        &self,
        store_id: &str,
        request: &ListStoreProductsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let store_id_clone = store_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let store_id_for_async = store_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/ecommerce/stores/{}/products", store_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("products")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new product to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .create_store_product(
    ///             &"store_id".to_string(),
    ///             &EcommerceStoresOrdersPost {
    ///                 description: None,
    ///                 handle: None,
    ///                 id: EcommerceStoresOrdersPostID::String("id".to_string()),
    ///                 image_url: None,
    ///                 images: None,
    ///                 published_at_foreign: None,
    ///                 title: "Cat Hat".to_string(),
    ///                 r#type: None,
    ///                 url: None,
    ///                 variants: vec![EcommerceStoresOrdersPostVariantsItem {
    ///                     backorders: None,
    ///                     id: EcommerceStoresOrdersPostVariantsItemID::String("id".to_string()),
    ///                     image_url: None,
    ///                     inventory_quantity: None,
    ///                     price: None,
    ///                     sku: None,
    ///                     title: "Cat Hat".to_string(),
    ///                     url: None,
    ///                     visibility: None,
    ///                 }],
    ///                 vendor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_product(
        &self,
        store_id: &str,
        request: &EcommerceStoresOrdersPost,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/products", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store_product(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &GetStoreProductQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_product(
        &self,
        store_id: &str,
        product_id: &str,
        request: &GetStoreProductQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/products/{}", store_id, product_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Update a specific product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .upsert_store_product(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &UpsertStoreProductEcommerceRequest {
    ///                 id: UpsertStoreProductEcommerceRequestID::String("id".to_string()),
    ///                 description: None,
    ///                 handle: None,
    ///                 image_url: None,
    ///                 images: None,
    ///                 published_at_foreign: None,
    ///                 title: None,
    ///                 r#type: None,
    ///                 url: None,
    ///                 variants: None,
    ///                 vendor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_store_product(
        &self,
        store_id: &str,
        product_id: &str,
        request: &UpsertStoreProductEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("3.0/ecommerce/stores/{}/products/{}", store_id, product_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store_product(&"store_id".to_string(), &"product_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_product(
        &self,
        store_id: &str,
        product_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/ecommerce/stores/{}/products/{}", store_id, product_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store_product(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &UpdateStoreProductEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_product(
        &self,
        store_id: &str,
        product_id: &str,
        request: &UpdateStoreProductEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/ecommerce/stores/{}/products/{}", store_id, product_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a product's images.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_store_product_images(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &ListStoreProductImagesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_product_images(
        &self,
        store_id: &str,
        product_id: &str,
        request: &ListStoreProductImagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreProductImagesEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/images",
                    store_id, product_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_store_product_images_paginated(
        &self,
        store_id: &str,
        product_id: &str,
        request: &ListStoreProductImagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let store_id_clone = store_id.to_string();
        let product_id_clone = product_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let store_id_for_async = store_id_clone.clone();
                let product_id_for_async = product_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/ecommerce/stores/{}/products/{}/images",
                                store_id_for_async, product_id_for_async
                            ),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("images")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new image to the product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .create_store_product_image(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &CreateStoreProductImageEcommerceRequest {
    ///                 id: "id".to_string(),
    ///                 url: "url".to_string(),
    ///                 variant_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_product_image(
        &self,
        store_id: &str,
        product_id: &str,
        request: &CreateStoreProductImageEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateStoreProductImageEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/images",
                    store_id, product_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific product image.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `image_id` - The id for the product image.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store_product_image(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"image_id".to_string(),
    ///             &GetStoreProductImageQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_product_image(
        &self,
        store_id: &str,
        product_id: &str,
        image_id: &str,
        request: &GetStoreProductImageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetStoreProductImageEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/images/{}",
                    store_id, product_id, image_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a product image.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `image_id` - The id for the product image.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store_product_image(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"image_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_product_image(
        &self,
        store_id: &str,
        product_id: &str,
        image_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/images/{}",
                    store_id, product_id, image_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a product image.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `image_id` - The id for the product image.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store_product_image(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"image_id".to_string(),
    ///             &UpdateStoreProductImageEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_product_image(
        &self,
        store_id: &str,
        product_id: &str,
        image_id: &str,
        request: &UpdateStoreProductImageEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateStoreProductImageEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/images/{}",
                    store_id, product_id, image_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a product's variants.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_store_product_variants(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &ListStoreProductVariantsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_product_variants(
        &self,
        store_id: &str,
        product_id: &str,
        request: &ListStoreProductVariantsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreProductVariantsEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants",
                    store_id, product_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_store_product_variants_paginated(
        &self,
        store_id: &str,
        product_id: &str,
        request: &ListStoreProductVariantsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let store_id_clone = store_id.to_string();
        let product_id_clone = product_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let store_id_for_async = store_id_clone.clone();
                let product_id_for_async = product_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/ecommerce/stores/{}/products/{}/variants",
                                store_id_for_async, product_id_for_async
                            ),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("variants")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new variant to the product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .create_store_product_variant(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &CreateStoreProductVariantEcommerceRequest {
    ///                 id: CreateStoreProductVariantEcommerceRequestID::String("id".to_string()),
    ///                 title: "Cat Hat".to_string(),
    ///                 backorders: None,
    ///                 image_url: None,
    ///                 inventory_quantity: None,
    ///                 price: None,
    ///                 sku: None,
    ///                 url: None,
    ///                 visibility: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_product_variant(
        &self,
        store_id: &str,
        product_id: &str,
        request: &CreateStoreProductVariantEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProductVariant, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants",
                    store_id, product_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific product variant.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `variant_id` - The id for the product variant.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store_product_variant(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"variant_id".to_string(),
    ///             &GetStoreProductVariantQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_product_variant(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
        request: &GetStoreProductVariantQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProductVariant, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants/{}",
                    store_id, product_id, variant_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add or update a product variant.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `variant_id` - The id for the product variant.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .upsert_store_product_variant(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"variant_id".to_string(),
    ///             &UpsertStoreProductVariantEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_store_product_variant(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
        request: &UpsertStoreProductVariantEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProductVariant, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants/{}",
                    store_id, product_id, variant_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a product variant.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `variant_id` - The id for the product variant.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store_product_variant(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"variant_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_product_variant(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants/{}",
                    store_id, product_id, variant_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a product variant.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `variant_id` - The id for the product variant.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store_product_variant(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"variant_id".to_string(),
    ///             &UpdateStoreProductVariantEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_product_variant(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
        request: &UpdateStoreProductVariantEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProductVariant, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants/{}",
                    store_id, product_id, variant_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's promo rules.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_store_promo_rules(
    ///             &"store_id".to_string(),
    ///             &ListStorePromoRulesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_promo_rules(
        &self,
        store_id: &str,
        request: &ListStorePromoRulesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStorePromoRulesEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/promo-rules", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_store_promo_rules_paginated(
        &self,
        store_id: &str,
        request: &ListStorePromoRulesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let store_id_clone = store_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let store_id_for_async = store_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/ecommerce/stores/{}/promo-rules", store_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("promo_rules")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new promo rule to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .create_store_promo_rule(
    ///             &"store_id".to_string(),
    ///             &CreateStorePromoRuleEcommerceRequest {
    ///                 amount: CreateStorePromoRuleEcommerceRequestAmount::Double(1.1),
    ///                 description: "Save BIG during our summer sale!".to_string(),
    ///                 id: "id".to_string(),
    ///                 target: CreateStorePromoRuleEcommerceRequestTarget::PerItem,
    ///                 r#type: CreateStorePromoRuleEcommerceRequestType::Fixed,
    ///                 created_at_foreign: None,
    ///                 enabled: None,
    ///                 ends_at: None,
    ///                 starts_at: None,
    ///                 title: None,
    ///                 updated_at_foreign: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_promo_rule(
        &self,
        store_id: &str,
        request: &CreateStorePromoRuleEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoRule, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/promo-rules", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific promo rule.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store_promo_rule(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &GetStorePromoRuleQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_promo_rule(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        request: &GetStorePromoRuleQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoRule, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}",
                    store_id, promo_rule_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a promo rule from a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store_promo_rule(&"store_id".to_string(), &"promo_rule_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_promo_rule(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}",
                    store_id, promo_rule_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a promo rule.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store_promo_rule(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &UpdateStorePromoRuleEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_promo_rule(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        request: &UpdateStorePromoRuleEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoRule, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}",
                    store_id, promo_rule_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's promo codes.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .list_store_promo_rule_promo_codes(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &ListStorePromoRulePromoCodesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_promo_rule_promo_codes(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        request: &ListStorePromoRulePromoCodesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStorePromoRulePromoCodesEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes",
                    store_id, promo_rule_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_store_promo_rule_promo_codes_paginated(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        request: &ListStorePromoRulePromoCodesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let store_id_clone = store_id.to_string();
        let promo_rule_id_clone = promo_rule_id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let store_id_for_async = store_id_clone.clone();
                let promo_rule_id_for_async = promo_rule_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes",
                                store_id_for_async, promo_rule_id_for_async
                            ),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("promo_codes")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Add a new promo code to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client.ecommerce.create_store_promo_rule_promo_code(&"store_id".to_string(), &"promo_rule_id".to_string(), &CreateStorePromoRulePromoCodeEcommerceRequest {
    ///         code: "summersale".to_string(),
    ///         id: "id".to_string(),
    ///         redemption_url: "A url that applies promo code directly at checkout or a url that points to sale page or store url".to_string(),
    ///         created_at_foreign: None,
    ///         enabled: None,
    ///         updated_at_foreign: None,
    ///         usage_count: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn create_store_promo_rule_promo_code(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        request: &CreateStorePromoRulePromoCodeEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoCode, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes",
                    store_id, promo_rule_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific promo code.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `promo_code_id` - The id for the promo code of a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .get_store_promo_rule_promo_code(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &"promo_code_id".to_string(),
    ///             &GetStorePromoRulePromoCodeQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_promo_rule_promo_code(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        promo_code_id: &str,
        request: &GetStorePromoRulePromoCodeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoCode, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes/{}",
                    store_id, promo_rule_id, promo_code_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a promo code from a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `promo_code_id` - The id for the promo code of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .delete_store_promo_rule_promo_code(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &"promo_code_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_promo_rule_promo_code(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        promo_code_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes/{}",
                    store_id, promo_rule_id, promo_code_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a promo code.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `promo_code_id` - The id for the promo code of a store.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ecommerce
    ///         .update_store_promo_rule_promo_code(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &"promo_code_id".to_string(),
    ///             &UpdateStorePromoRulePromoCodeEcommerceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_promo_rule_promo_code(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        promo_code_id: &str,
        request: &UpdateStorePromoRulePromoCodeEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoCode, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes/{}",
                    store_id, promo_rule_id, promo_code_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
