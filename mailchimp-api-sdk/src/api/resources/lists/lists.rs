use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct ListsClient {
    pub http_client: HttpClient,
}

impl ListsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get information about all lists in the account.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `before_date_created` - Restrict response to lists created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_date_created` - Restrict results to lists created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_campaign_last_sent` - Restrict results to lists created before the last campaign send date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_campaign_last_sent` - Restrict results to lists created after the last campaign send date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `email` - Restrict results to lists that include a specific subscriber's email address.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `has_ecommerce_store` - Restrict results to lists that contain an active, connected, undeleted ecommerce store.
    /// * `include_total_contacts` - Deprecated. Return the total_contacts field in the stats response, which contains an approximate count of subscribed, unsubscribed, and transactional contacts. For a complete audience contact count, use the /audiences endpoint instead.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list(
    ///             &ListsListQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 before_date_created: None,
    ///                 since_date_created: None,
    ///                 before_campaign_last_sent: None,
    ///                 since_campaign_last_sent: None,
    ///                 email: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///                 has_ecommerce_store: None,
    ///                 include_total_contacts: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ListsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/lists",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("before_date_created", request.before_date_created.clone())
                    .string("since_date_created", request.since_date_created.clone())
                    .string(
                        "before_campaign_last_sent",
                        request.before_campaign_last_sent.clone(),
                    )
                    .string(
                        "since_campaign_last_sent",
                        request.since_campaign_last_sent.clone(),
                    )
                    .string("email", request.email.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .bool("has_ecommerce_store", request.has_ecommerce_store.clone())
                    .bool(
                        "include_total_contacts",
                        request.include_total_contacts.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &ListsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("before_date_created", request.before_date_created.clone())
            .string("since_date_created", request.since_date_created.clone())
            .string(
                "before_campaign_last_sent",
                request.before_campaign_last_sent.clone(),
            )
            .string(
                "since_campaign_last_sent",
                request.since_campaign_last_sent.clone(),
            )
            .string("email", request.email.clone())
            .serialize("sort_field", request.sort_field.clone())
            .serialize("sort_dir", request.sort_dir.clone())
            .bool("has_ecommerce_store", request.has_ecommerce_store.clone())
            .bool(
                "include_total_contacts",
                request.include_total_contacts.clone(),
            )
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
                            "3.0/lists",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("lists")
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

    /// Create a new list in your Mailchimp account.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create(
    ///             &CreateListsRequest {
    ///                 campaign_defaults: CreateListsRequestCampaignDefaults {
    ///                     from_email: "from_email".to_string(),
    ///                     from_name: "from_name".to_string(),
    ///                     language: "language".to_string(),
    ///                     subject: "subject".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 contact: CreateListsRequestContact {
    ///                     address1: "address1".to_string(),
    ///                     city: "city".to_string(),
    ///                     company: "company".to_string(),
    ///                     country: "country".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 email_type_option: true,
    ///                 name: "name".to_string(),
    ///                 permission_reminder: "permission_reminder".to_string(),
    ///                 double_optin: None,
    ///                 marketing_permissions: None,
    ///                 notify_on_subscribe: None,
    ///                 notify_on_unsubscribe: None,
    ///                 use_archive_bar: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriberList, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/lists",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific list in your Mailchimp account. Results include list members who have signed up but haven't confirmed their subscription yet and unsubscribed or cleaned.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `include_total_contacts` - Deprecated. Return the total_contacts field in the stats response, which contains an approximate count of subscribed, unsubscribed, and transactional contacts. For a complete audience contact count, use the /audiences endpoint instead.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get(
    ///             &"list_id".to_string(),
    ///             &ListsGetQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 include_total_contacts: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        list_id: &str,
        request: &ListsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriberList, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .bool(
                        "include_total_contacts",
                        request.include_total_contacts.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Batch subscribe or unsubscribe list members.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `skip_merge_validation` - If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    /// * `skip_duplicate_check` - If skip_duplicate_check is true, we will ignore duplicates sent in the request when using the batch sub/unsub on the lists endpoint. The status of the first appearance in the request will be saved. This defaults to false.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .batch_subscribe_or_unsubscribe(
    ///             &"list_id".to_string(),
    ///             &BatchSubscribeOrUnsubscribeListsRequest {
    ///                 members: vec![],
    ///                 skip_merge_validation: None,
    ///                 skip_duplicate_check: None,
    ///                 sync_tags: None,
    ///                 update_existing: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn batch_subscribe_or_unsubscribe(
        &self,
        list_id: &str,
        request: &BatchSubscribeOrUnsubscribeListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchSubscribeOrUnsubscribeListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "skip_merge_validation",
                        request.skip_merge_validation.clone(),
                    )
                    .bool("skip_duplicate_check", request.skip_duplicate_check.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a list from your Mailchimp account. If you delete a list, you'll lose the list history—including subscriber activity, unsubscribes, complaints, and bounces. You’ll also lose subscribers’ email addresses, unless you exported and backed up your list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client.lists.delete(&"list_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        list_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}", list_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the settings for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .update(
    ///             &"list_id".to_string(),
    ///             &UpdateListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        list_id: &str,
        request: &UpdateListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriberList, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get all abuse reports for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_abuse_reports(
    ///             &"list_id".to_string(),
    ///             &ListsListAbuseReportsQueryRequest {
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
    pub async fn list_abuse_reports(
        &self,
        list_id: &str,
        request: &ListsListAbuseReportsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAbuseReportsListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/abuse-reports", list_id),
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

    pub async fn list_abuse_reports_paginated(
        &self,
        list_id: &str,
        request: &ListsListAbuseReportsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();

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
                let list_id_for_async = list_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/lists/{}/abuse-reports", list_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("abuse_reports")
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

    /// Get details about a specific abuse report.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `report_id` - The id for the abuse report.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get_abuse_report(
    ///             &"list_id".to_string(),
    ///             &"report_id".to_string(),
    ///             &ListsGetAbuseReportQueryRequest {
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
    pub async fn get_abuse_report(
        &self,
        list_id: &str,
        report_id: &str,
        request: &ListsGetAbuseReportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListsAbuseReports, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/abuse-reports/{}", list_id, report_id),
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

    /// Get up to the previous 180 days of daily detailed aggregated activity stats for a list, not including Automation activity.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_activity(
    ///             &"list_id".to_string(),
    ///             &ListActivityQueryRequest {
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
    pub async fn list_activity(
        &self,
        list_id: &str,
        request: &ListActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListActivityListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/activity", list_id),
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

    pub async fn list_activity_paginated(
        &self,
        list_id: &str,
        request: &ListActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();

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
                let list_id_for_async = list_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/lists/{}/activity", list_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("activity")
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

    /// Get a list of the top email clients based on user-agent strings.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_clients(
    ///             &"list_id".to_string(),
    ///             &ListClientsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_clients(
        &self,
        list_id: &str,
        request: &ListClientsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListClientsListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/clients", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a month-by-month summary of a specific list's growth activity.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_growth_history(
    ///             &"list_id".to_string(),
    ///             &ListGrowthHistoryQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_growth_history(
        &self,
        list_id: &str,
        request: &ListGrowthHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListGrowthHistoryListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/growth-history", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_growth_history_paginated(
        &self,
        list_id: &str,
        request: &ListGrowthHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .serialize("sort_field", request.sort_field.clone())
            .serialize("sort_dir", request.sort_dir.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();

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
                let list_id_for_async = list_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/lists/{}/growth-history", list_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("history")
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

    /// Get a summary of a specific list's growth activity for a specific month and year.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `month` - A specific month of list growth history.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get_growth_history(
    ///             &"list_id".to_string(),
    ///             &"month".to_string(),
    ///             &GetGrowthHistoryQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_growth_history(
        &self,
        list_id: &str,
        month: &str,
        request: &GetGrowthHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GrowthHistory, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/growth-history/{}", list_id, month),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about a list's interest categories.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - Restrict results a type of interest group
    /// * `sort_field` - Returns interest categories sorted by the specified field. Defaults to display_order.
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_interest_categories(
    ///             &"list_id".to_string(),
    ///             &ListInterestCategoriesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_interest_categories(
        &self,
        list_id: &str,
        request: &ListInterestCategoriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListInterestCategoriesListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/interest-categories", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("type", request.r#type.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_interest_categories_paginated(
        &self,
        list_id: &str,
        request: &ListInterestCategoriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("type", request.r#type.clone())
            .serialize("sort_field", request.sort_field.clone())
            .serialize("sort_dir", request.sort_dir.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();

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
                let list_id_for_async = list_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/lists/{}/interest-categories", list_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("categories")
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

    /// Create a new interest category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_interest_category(
    ///             &"list_id".to_string(),
    ///             &CreateInterestCategoryListsRequest {
    ///                 title: "title".to_string(),
    ///                 r#type: CreateInterestCategoryListsRequestType::Checkboxes,
    ///                 display_order: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_interest_category(
        &self,
        list_id: &str,
        request: &CreateInterestCategoryListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<InterestCategory, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/interest-categories", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific interest category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get_interest_category(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &GetInterestCategoryQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_interest_category(
        &self,
        list_id: &str,
        interest_category_id: &str,
        request: &GetInterestCategoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<InterestCategory, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/lists/{}/interest-categories/{}",
                    list_id, interest_category_id
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

    /// Delete a specific interest category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .delete_interest_category(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_interest_category(
        &self,
        list_id: &str,
        interest_category_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/lists/{}/interest-categories/{}",
                    list_id, interest_category_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific interest category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .update_interest_category(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &UpdateInterestCategoryListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_interest_category(
        &self,
        list_id: &str,
        interest_category_id: &str,
        request: &UpdateInterestCategoryListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<InterestCategory, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/lists/{}/interest-categories/{}",
                    list_id, interest_category_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of this category's interests.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_interest_category_interests(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &ListInterestCategoryInterestsQueryRequest {
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
    pub async fn list_interest_category_interests(
        &self,
        list_id: &str,
        interest_category_id: &str,
        request: &ListInterestCategoryInterestsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListInterestCategoryInterestsListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/lists/{}/interest-categories/{}/interests",
                    list_id, interest_category_id
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

    pub async fn list_interest_category_interests_paginated(
        &self,
        list_id: &str,
        interest_category_id: &str,
        request: &ListInterestCategoryInterestsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();
        let interest_category_id_clone = interest_category_id.to_string();

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
                let list_id_for_async = list_id_clone.clone();
                let interest_category_id_for_async = interest_category_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/lists/{}/interest-categories/{}/interests",
                                list_id_for_async, interest_category_id_for_async
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
                        .get("interests")
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

    /// Create a new interest or 'group name' for a specific category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_interest_category_interest(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &CreateInterestCategoryInterestListsRequest {
    ///                 name: "name".to_string(),
    ///                 display_order: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_interest_category_interest(
        &self,
        list_id: &str,
        interest_category_id: &str,
        request: &CreateInterestCategoryInterestListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Interest, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/lists/{}/interest-categories/{}/interests",
                    list_id, interest_category_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get interests or 'group names' for a specific category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
    /// * `interest_id` - The specific interest or 'group name'.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get_interest_category_interest(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &"interest_id".to_string(),
    ///             &GetInterestCategoryInterestQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_interest_category_interest(
        &self,
        list_id: &str,
        interest_category_id: &str,
        interest_id: &str,
        request: &GetInterestCategoryInterestQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Interest, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/lists/{}/interest-categories/{}/interests/{}",
                    list_id, interest_category_id, interest_id
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

    /// Delete interests or group names in a specific category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
    /// * `interest_id` - The specific interest or 'group name'.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .delete_interest_category_interest(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &"interest_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_interest_category_interest(
        &self,
        list_id: &str,
        interest_category_id: &str,
        interest_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/lists/{}/interest-categories/{}/interests/{}",
                    list_id, interest_category_id, interest_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update interests or 'group names' for a specific category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
    /// * `interest_id` - The specific interest or 'group name'.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .update_interest_category_interest(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &"interest_id".to_string(),
    ///             &UpdateInterestCategoryInterestListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_interest_category_interest(
        &self,
        list_id: &str,
        interest_category_id: &str,
        interest_id: &str,
        request: &UpdateInterestCategoryInterestListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Interest, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/lists/{}/interest-categories/{}/interests/{}",
                    list_id, interest_category_id, interest_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get the locations (countries) that the list's subscribers have been tagged to based on geocoding their IP address.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_locations(
    ///             &"list_id".to_string(),
    ///             &ListsListLocationsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_locations(
        &self,
        list_id: &str,
        request: &ListsListLocationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListLocationsListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/locations", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about members in a specific Mailchimp list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `email_type` - The email type.
    /// * `status` - The subscriber's status.
    /// * `since_timestamp_opt` - Restrict results to subscribers who opted-in after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_timestamp_opt` - Restrict results to subscribers who opted-in before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_last_changed` - Restrict results to subscribers whose information changed after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_last_changed` - Restrict results to subscribers whose information changed before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `unique_email_id` - A unique identifier for the email address across all Mailchimp lists.
    /// * `vip_only` - A filter to return only the list's VIP members. Passing `true` will restrict results to VIP list members, passing `false` will return all list members.
    /// * `interest_category_id` - The unique id for the interest category.
    /// * `interest_ids` - Used to filter list members by interests. Must be accompanied by interest_category_id and interest_match. The value must be a comma separated list of interest ids present for any supplied interest categories.
    /// * `interest_match` - Used to filter list members by interests. Must be accompanied by interest_category_id and interest_ids. "any" will match a member with any of the interest supplied, "all" will only match members with every interest supplied, and "none" will match members without any of the interest supplied.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `since_last_campaign` - Filter subscribers by those subscribed/unsubscribed/pending/cleaned since last email campaign send. Member status is required to use this filter.
    /// * `unsubscribed_since` - Filter subscribers by those unsubscribed since a specific date. Using any status other than unsubscribed with this filter will result in an error.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_members(
    ///             &"list_id".to_string(),
    ///             &ListMembersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 email_type: None,
    ///                 status: None,
    ///                 since_timestamp_opt: None,
    ///                 before_timestamp_opt: None,
    ///                 since_last_changed: None,
    ///                 before_last_changed: None,
    ///                 unique_email_id: None,
    ///                 vip_only: None,
    ///                 interest_category_id: None,
    ///                 interest_ids: None,
    ///                 interest_match: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///                 since_last_campaign: None,
    ///                 unsubscribed_since: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_members(
        &self,
        list_id: &str,
        request: &ListMembersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembersListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("email_type", request.email_type.clone())
                    .serialize("status", request.status.clone())
                    .string("since_timestamp_opt", request.since_timestamp_opt.clone())
                    .string("before_timestamp_opt", request.before_timestamp_opt.clone())
                    .string("since_last_changed", request.since_last_changed.clone())
                    .string("before_last_changed", request.before_last_changed.clone())
                    .string("unique_email_id", request.unique_email_id.clone())
                    .bool("vip_only", request.vip_only.clone())
                    .string("interest_category_id", request.interest_category_id.clone())
                    .string("interest_ids", request.interest_ids.clone())
                    .serialize("interest_match", request.interest_match.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .bool("since_last_campaign", request.since_last_campaign.clone())
                    .string("unsubscribed_since", request.unsubscribed_since.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_members_paginated(
        &self,
        list_id: &str,
        request: &ListMembersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("email_type", request.email_type.clone())
            .serialize("status", request.status.clone())
            .string("since_timestamp_opt", request.since_timestamp_opt.clone())
            .string("before_timestamp_opt", request.before_timestamp_opt.clone())
            .string("since_last_changed", request.since_last_changed.clone())
            .string("before_last_changed", request.before_last_changed.clone())
            .string("unique_email_id", request.unique_email_id.clone())
            .bool("vip_only", request.vip_only.clone())
            .string("interest_category_id", request.interest_category_id.clone())
            .string("interest_ids", request.interest_ids.clone())
            .serialize("interest_match", request.interest_match.clone())
            .serialize("sort_field", request.sort_field.clone())
            .serialize("sort_dir", request.sort_dir.clone())
            .bool("since_last_campaign", request.since_last_campaign.clone())
            .string("unsubscribed_since", request.unsubscribed_since.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();

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
                let list_id_for_async = list_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/lists/{}/members", list_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("members")
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

    /// Add a new member to the list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `skip_merge_validation` - If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_member(
    ///             &"list_id".to_string(),
    ///             &CreateMemberListsRequest {
    ///                 email_address: "email_address".to_string(),
    ///                 status: CreateMemberListsRequestStatus::Subscribed,
    ///                 skip_merge_validation: None,
    ///                 email_type: None,
    ///                 interests: None,
    ///                 ip_opt: None,
    ///                 ip_signup: None,
    ///                 language: None,
    ///                 location: None,
    ///                 marketing_permissions: None,
    ///                 merge_fields: None,
    ///                 tags: None,
    ///                 timestamp_opt: None,
    ///                 timestamp_signup: None,
    ///                 vip: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_member(
        &self,
        list_id: &str,
        request: &CreateMemberListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembers, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/members", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "skip_merge_validation",
                        request.skip_merge_validation.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Get information about a specific list member, including a currently subscribed, unsubscribed, or bounced member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get_member(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &GetMemberQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_member(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &GetMemberQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembers, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add or update a list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `skip_merge_validation` - If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .upsert_member(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &UpsertMemberListsRequest {
    ///                 email_address: "email_address".to_string(),
    ///                 skip_merge_validation: None,
    ///                 email_type: None,
    ///                 interests: None,
    ///                 ip_opt: None,
    ///                 ip_signup: None,
    ///                 language: None,
    ///                 location: None,
    ///                 marketing_permissions: None,
    ///                 merge_fields: None,
    ///                 status: None,
    ///                 status_if_new: None,
    ///                 tags: None,
    ///                 timestamp_opt: None,
    ///                 timestamp_signup: None,
    ///                 vip: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_member(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &UpsertMemberListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembers, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("3.0/lists/{}/members/{}", list_id, subscriber_hash),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "skip_merge_validation",
                        request.skip_merge_validation.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Archive a list member. To permanently delete, use the delete-permanent action.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .delete_member(&"list_id".to_string(), &"subscriber_hash".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_member(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}/members/{}", list_id, subscriber_hash),
                None,
                None,
                options,
            )
            .await
    }

    /// Update information for a specific list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `skip_merge_validation` - If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .update_member(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &UpdateMemberListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_member(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &UpdateMemberListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembers, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}/members/{}", list_id, subscriber_hash),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "skip_merge_validation",
                        request.skip_merge_validation.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Delete all personally identifiable information related to a list member, and remove them from a list. This will make it impossible to re-import the list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_member_action_delete_permanent(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_member_action_delete_permanent(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/lists/{}/members/{}/actions/delete-permanent",
                    list_id, subscriber_hash
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get the last 50 events of a member's activity on a specific list, including opens, clicks, and unsubscribes.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `action` - A comma seperated list of actions to return.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_member_activity(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberActivityQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 action: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_member_activity(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberActivityListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}/activity", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .serialize_array("action", request.action.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a member's activity on a specific list, including opens, clicks, and unsubscribes.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `activity_filters` - A comma-separated list of activity filters that correspond to a set of activity types, e.g "?activity_filters=open,bounce,click".
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_member_activity_feed(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberActivityFeedQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 activity_filters: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_member_activity_feed(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberActivityFeedQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberActivityFeedListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/lists/{}/members/{}/activity-feed",
                    list_id, subscriber_hash
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize_array("activity_filters", request.activity_filters.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_member_activity_feed_paginated(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberActivityFeedQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .serialize_array("activity_filters", request.activity_filters.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();
        let subscriber_hash_clone = subscriber_hash.to_string();

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
                let list_id_for_async = list_id_clone.clone();
                let subscriber_hash_for_async = subscriber_hash_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/lists/{}/members/{}/activity-feed",
                                list_id_for_async, subscriber_hash_for_async
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
                        .get("activity")
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

    /// Get events for a contact.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_member_events(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberEventsQueryRequest {
    ///                 count: None,
    ///                 offset: None,
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_member_events(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberEventsListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}/events", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_member_events_paginated(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("count", request.count.clone())
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();
        let subscriber_hash_clone = subscriber_hash.to_string();

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
                let list_id_for_async = list_id_clone.clone();
                let subscriber_hash_for_async = subscriber_hash_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/lists/{}/members/{}/events",
                                list_id_for_async, subscriber_hash_for_async
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
                        .get("events")
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

    /// Add an event for a list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_member_event(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &CreateMemberEventListsRequest {
    ///                 name: "name".to_string(),
    ///                 is_syncing: None,
    ///                 occurred_at: None,
    ///                 properties: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_member_event(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &CreateMemberEventListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/members/{}/events", list_id, subscriber_hash),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get the last 50 Goal events for a member on a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_member_goals(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberGoalsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_member_goals(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberGoalsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberGoalsListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}/goals", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get recent notes for a specific list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
    /// * `sort_field` - Returns notes sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_member_notes(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberNotesQueryRequest {
    ///                 sort_field: None,
    ///                 sort_dir: None,
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
    pub async fn list_member_notes(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberNotesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberNotesListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}/notes", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_member_notes_paginated(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberNotesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .serialize("sort_field", request.sort_field.clone())
            .serialize("sort_dir", request.sort_dir.clone())
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();
        let subscriber_hash_clone = subscriber_hash.to_string();

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
                let list_id_for_async = list_id_clone.clone();
                let subscriber_hash_for_async = subscriber_hash_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/lists/{}/members/{}/notes",
                                list_id_for_async, subscriber_hash_for_async
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
                        .get("notes")
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

    /// Add a new note for a specific subscriber.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_member_note(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &CreateMemberNoteListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_member_note(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &CreateMemberNoteListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemberNotes, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/members/{}/notes", list_id, subscriber_hash),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a specific note for a specific list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `note_id` - The id for the note.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get_member_note(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &"note_id".to_string(),
    ///             &GetMemberNoteQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_member_note(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        note_id: &str,
        request: &GetMemberNoteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemberNotes, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/lists/{}/members/{}/notes/{}",
                    list_id, subscriber_hash, note_id
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

    /// Delete a specific note for a specific list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `note_id` - The id for the note.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .delete_member_note(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &"note_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_member_note(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        note_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/lists/{}/members/{}/notes/{}",
                    list_id, subscriber_hash, note_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific note for a specific list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `note_id` - The id for the note.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .update_member_note(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &"note_id".to_string(),
    ///             &UpdateMemberNoteListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_member_note(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        note_id: &str,
        request: &UpdateMemberNoteListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemberNotes, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/lists/{}/members/{}/notes/{}",
                    list_id, subscriber_hash, note_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get the tags on a list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_member_tags(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberTagsQueryRequest {
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
    pub async fn list_member_tags(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberTagsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberTagsListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}/tags", list_id, subscriber_hash),
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

    pub async fn list_member_tags_paginated(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberTagsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();
        let subscriber_hash_clone = subscriber_hash.to_string();

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
                let list_id_for_async = list_id_clone.clone();
                let subscriber_hash_for_async = subscriber_hash_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/lists/{}/members/{}/tags",
                                list_id_for_async, subscriber_hash_for_async
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
                        .get("tags")
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

    /// Add or remove tags from a list member. If a tag that does not exist is passed in and set as 'active', a new tag will be created.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_member_tag(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &CreateMemberTagListsRequest {
    ///                 tags: vec![CreateMemberTagListsRequestTagsItem {
    ///                     name: "name".to_string(),
    ///                     status: CreateMemberTagListsRequestTagsItemStatus::Inactive,
    ///                 }],
    ///                 is_syncing: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_member_tag(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &CreateMemberTagListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/members/{}/tags", list_id, subscriber_hash),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of all merge fields for an audience.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - The merge field type.
    /// * `required` - Whether it's a required merge field.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_merge_fields(
    ///             &"list_id".to_string(),
    ///             &ListMergeFieldsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 required: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_merge_fields(
        &self,
        list_id: &str,
        request: &ListMergeFieldsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMergeFieldsListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/merge-fields", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("type", request.r#type.clone())
                    .bool("required", request.required.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_merge_fields_paginated(
        &self,
        list_id: &str,
        request: &ListMergeFieldsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("type", request.r#type.clone())
            .bool("required", request.required.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();

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
                let list_id_for_async = list_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/lists/{}/merge-fields", list_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("merge_fields")
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

    /// Add a new merge field for a specific audience.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_merge_field(
    ///             &"list_id".to_string(),
    ///             &CreateMergeFieldListsRequest {
    ///                 name: "name".to_string(),
    ///                 r#type: CreateMergeFieldListsRequestType::Text,
    ///                 default_value: None,
    ///                 display_order: None,
    ///                 help_text: None,
    ///                 options: None,
    ///                 public: None,
    ///                 required: None,
    ///                 tag: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_merge_field(
        &self,
        list_id: &str,
        request: &CreateMergeFieldListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MergeField, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/merge-fields", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific merge field.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `merge_id` - The id for the merge field.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get_merge_field(
    ///             &"list_id".to_string(),
    ///             &"merge_id".to_string(),
    ///             &GetMergeFieldQueryRequest {
    ///                 exclude_fields: vec![],
    ///                 fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_merge_field(
        &self,
        list_id: &str,
        merge_id: &str,
        request: &GetMergeFieldQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MergeField, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/merge-fields/{}", list_id, merge_id),
                None,
                QueryBuilder::new()
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .string_array("fields", request.fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific merge field.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `merge_id` - The id for the merge field.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .delete_merge_field(&"list_id".to_string(), &"merge_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_merge_field(
        &self,
        list_id: &str,
        merge_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}/merge-fields/{}", list_id, merge_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific merge field.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `merge_id` - The id for the merge field.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .update_merge_field(
    ///             &"list_id".to_string(),
    ///             &"merge_id".to_string(),
    ///             &UpdateMergeFieldListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_merge_field(
        &self,
        list_id: &str,
        merge_id: &str,
        request: &UpdateMergeFieldListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MergeField, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}/merge-fields/{}", list_id, merge_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about all available segments for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - Limit results based on segment type.
    /// * `since_created_at` - Restrict results to segments created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_created_at` - Restrict results to segments created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `include_cleaned` - Include cleaned members in response
    /// * `include_transactional` - Include transactional members in response
    /// * `include_unsubscribed` - Include unsubscribed members in response
    /// * `since_updated_at` - Restrict results to segments update after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_updated_at` - Restrict results to segments update before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `exclude_type` - Exclude results based on segment type. For example, use `exclude_type=static` to exclude tags from the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_segments(
    ///             &"list_id".to_string(),
    ///             &ListSegmentsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 since_created_at: None,
    ///                 before_created_at: None,
    ///                 include_cleaned: None,
    ///                 include_transactional: None,
    ///                 include_unsubscribed: None,
    ///                 since_updated_at: None,
    ///                 before_updated_at: None,
    ///                 exclude_type: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_segments(
        &self,
        list_id: &str,
        request: &ListSegmentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSegmentsListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/segments", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("type", request.r#type.clone())
                    .string("since_created_at", request.since_created_at.clone())
                    .string("before_created_at", request.before_created_at.clone())
                    .bool("include_cleaned", request.include_cleaned.clone())
                    .bool(
                        "include_transactional",
                        request.include_transactional.clone(),
                    )
                    .bool("include_unsubscribed", request.include_unsubscribed.clone())
                    .string("since_updated_at", request.since_updated_at.clone())
                    .string("before_updated_at", request.before_updated_at.clone())
                    .serialize("exclude_type", request.exclude_type.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_segments_paginated(
        &self,
        list_id: &str,
        request: &ListSegmentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("type", request.r#type.clone())
            .string("since_created_at", request.since_created_at.clone())
            .string("before_created_at", request.before_created_at.clone())
            .bool("include_cleaned", request.include_cleaned.clone())
            .bool(
                "include_transactional",
                request.include_transactional.clone(),
            )
            .bool("include_unsubscribed", request.include_unsubscribed.clone())
            .string("since_updated_at", request.since_updated_at.clone())
            .string("before_updated_at", request.before_updated_at.clone())
            .serialize("exclude_type", request.exclude_type.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();

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
                let list_id_for_async = list_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/lists/{}/segments", list_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("segments")
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

    /// Create a new segment in a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_segment(
    ///             &"list_id".to_string(),
    ///             &CreateSegmentListsRequest {
    ///                 name: "name".to_string(),
    ///                 options: None,
    ///                 static_segment: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_segment(
        &self,
        list_id: &str,
        request: &CreateSegmentListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<List, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/segments", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific segment.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `include_cleaned` - Include cleaned members in response
    /// * `include_transactional` - Include transactional members in response
    /// * `include_unsubscribed` - Include unsubscribed members in response
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get_segment(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &GetSegmentQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 include_cleaned: None,
    ///                 include_transactional: None,
    ///                 include_unsubscribed: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_segment(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &GetSegmentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<List, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/segments/{}", list_id, segment_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .bool("include_cleaned", request.include_cleaned.clone())
                    .bool(
                        "include_transactional",
                        request.include_transactional.clone(),
                    )
                    .bool("include_unsubscribed", request.include_unsubscribed.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Batch add/remove list members to static segment
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .batch_add_or_remove_members(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &BatchAddOrRemoveMembersListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn batch_add_or_remove_members(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &BatchAddOrRemoveMembersListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchAddOrRemoveMembersListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/segments/{}", list_id, segment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a specific segment in a list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .delete_segment(&"list_id".to_string(), &"segment_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_segment(
        &self,
        list_id: &str,
        segment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}/segments/{}", list_id, segment_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific segment in a list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .update_segment(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &UpdateSegmentListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_segment(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &UpdateSegmentListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<List, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}/segments/{}", list_id, segment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about members in a saved segment.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `include_cleaned` - Include cleaned members in response
    /// * `include_transactional` - Include transactional members in response
    /// * `include_unsubscribed` - Include unsubscribed members in response
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_segment_members(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &ListSegmentMembersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 include_cleaned: None,
    ///                 include_transactional: None,
    ///                 include_unsubscribed: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_segment_members(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &ListSegmentMembersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSegmentMembersListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/segments/{}/members", list_id, segment_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .bool("include_cleaned", request.include_cleaned.clone())
                    .bool(
                        "include_transactional",
                        request.include_transactional.clone(),
                    )
                    .bool("include_unsubscribed", request.include_unsubscribed.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_segment_members_paginated(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &ListSegmentMembersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .bool("include_cleaned", request.include_cleaned.clone())
            .bool(
                "include_transactional",
                request.include_transactional.clone(),
            )
            .bool("include_unsubscribed", request.include_unsubscribed.clone())
            .build();
        let options_clone = options.clone();
        let list_id_clone = list_id.to_string();
        let segment_id_clone = segment_id.to_string();

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
                let list_id_for_async = list_id_clone.clone();
                let segment_id_for_async = segment_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!(
                                "3.0/lists/{}/segments/{}/members",
                                list_id_for_async, segment_id_for_async
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
                        .get("members")
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

    /// Add a member to a static segment.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_segment_member(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &CreateSegmentMemberListsRequest {
    ///                 email_address: "email_address".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_segment_member(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &CreateSegmentMemberListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListsSegmentsMembers, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/segments/{}/members", list_id, segment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a member from the specified static segment.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .delete_segment_member(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_segment_member(
        &self,
        list_id: &str,
        segment_id: &str,
        subscriber_hash: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/lists/{}/segments/{}/members/{}",
                    list_id, segment_id, subscriber_hash
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get signup forms for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_signup_forms(&"list_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_signup_forms(
        &self,
        list_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListSignupFormsListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/signup-forms", list_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Customize a list's default signup form.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_signup_form(
    ///             &"list_id".to_string(),
    ///             &CreateSignupFormListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_signup_form(
        &self,
        list_id: &str,
        request: &CreateSignupFormListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SignupForm, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/signup-forms", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about all available surveys for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_surveys(&"list_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_surveys(
        &self,
        list_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/surveys", list_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Create a draft survey for an audience.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_survey(
    ///             &"list_id".to_string(),
    ///             &CreateSurveyListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_survey(
        &self,
        list_id: &str,
        request: &CreateSurveyListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/surveys", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get details about a specific survey.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `survey_id` - The ID of the survey.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get_survey(&"list_id".to_string(), &"survey_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_survey(
        &self,
        list_id: &str,
        survey_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/surveys/{}", list_id, survey_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a survey.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `survey_id` - The ID of the survey.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .delete_survey(&"list_id".to_string(), &"survey_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_survey(
        &self,
        list_id: &str,
        survey_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}/surveys/{}", list_id, survey_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a survey. When sections is provided, send the complete section list in display order. Any existing section not included is deleted.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `survey_id` - The ID of the survey.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .update_survey(
    ///             &"list_id".to_string(),
    ///             &"survey_id".to_string(),
    ///             &UpdateSurveyListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_survey(
        &self,
        list_id: &str,
        survey_id: &str,
        request: &UpdateSurveyListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}/surveys/{}", list_id, survey_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replicate a survey.
    ///
    /// # Arguments
    ///
    /// * `list_id_path_param` - The unique ID for the list.
    /// * `survey_id` - The ID of the survey.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_list_survey_action_replicate(
    ///             &"list_id".to_string(),
    ///             &"survey_id".to_string(),
    ///             &CreateListSurveyActionReplicateListsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_list_survey_action_replicate(
        &self,
        list_id_path_param: &str,
        survey_id: &str,
        request: &CreateListSurveyActionReplicateListsRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/lists/{}/surveys/{}/actions/replicate",
                    list_id_path_param, survey_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Search for tags on a list by name. If no name is provided, will return all tags on the list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `name` - The search query used to filter tags.  The search query will be compared to each tag as a prefix, so all tags that have a name starting with this field will be returned.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_tag_search(
    ///             &"list_id".to_string(),
    ///             &ListTagSearchQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_tag_search(
        &self,
        list_id: &str,
        request: &ListTagSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListTagSearchListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/tag-search", list_id),
                None,
                QueryBuilder::new()
                    .string("name", request.name.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about all webhooks for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .list_webhooks(&"list_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_webhooks(
        &self,
        list_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhooksListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/webhooks", list_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Create a new webhook for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .create_webhook(
    ///             &"list_id".to_string(),
    ///             &AddWebhook {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_webhook(
        &self,
        list_id: &str,
        request: &AddWebhook,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhooks, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/webhooks", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific webhook.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `webhook_id` - The webhook's id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .get_webhook(&"list_id".to_string(), &"webhook_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_webhook(
        &self,
        list_id: &str,
        webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhooks, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/webhooks/{}", list_id, webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a specific webhook in a list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `webhook_id` - The webhook's id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .delete_webhook(&"list_id".to_string(), &"webhook_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_webhook(
        &self,
        list_id: &str,
        webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}/webhooks/{}", list_id, webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the settings for an existing webhook.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `webhook_id` - The webhook's id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .lists
    ///         .update_webhook(
    ///             &"list_id".to_string(),
    ///             &"webhook_id".to_string(),
    ///             &AddWebhook {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_webhook(
        &self,
        list_id: &str,
        webhook_id: &str,
        request: &AddWebhook,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhooks, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}/webhooks/{}", list_id, webhook_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
