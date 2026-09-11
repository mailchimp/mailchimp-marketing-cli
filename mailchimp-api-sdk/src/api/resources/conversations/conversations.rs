use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct ConversationsClient {
    pub http_client: HttpClient,
}

impl ConversationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a list of conversations for the account. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `has_unread_messages` - Whether the conversation has any unread messages.
    /// * `list_id` - The unique id for the list.
    /// * `campaign_id` - The unique id for the campaign.
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
    ///         .conversations
    ///         .list(
    ///             &ConversationsListQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 has_unread_messages: None,
    ///                 list_id: None,
    ///                 campaign_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ConversationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListConversationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/conversations",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize("has_unread_messages", request.has_unread_messages.clone())
                    .string("list_id", request.list_id.clone())
                    .string("campaign_id", request.campaign_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &ConversationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .serialize("has_unread_messages", request.has_unread_messages.clone())
            .string("list_id", request.list_id.clone())
            .string("campaign_id", request.campaign_id.clone())
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
                            "3.0/conversations",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("conversations")
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

    /// Get details about an individual conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The unique id for the conversation.
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
    ///         .conversations
    ///         .get(
    ///             &"conversation_id".to_string(),
    ///             &ConversationsGetQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        conversation_id: &str,
        request: &ConversationsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Conversation, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/conversations/{}", conversation_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get messages from a specific conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The unique id for the conversation.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `is_read` - Whether a conversation message has been marked as read.
    /// * `before_timestamp` - Restrict the response to messages created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_timestamp` - Restrict the response to messages created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
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
    ///         .conversations
    ///         .list_messages(
    ///             &"conversation_id".to_string(),
    ///             &ListMessagesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 is_read: None,
    ///                 before_timestamp: None,
    ///                 since_timestamp: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_messages(
        &self,
        conversation_id: &str,
        request: &ListMessagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMessagesConversationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/conversations/{}/messages", conversation_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .serialize("is_read", request.is_read.clone())
                    .datetime("before_timestamp", request.before_timestamp.clone())
                    .datetime("since_timestamp", request.since_timestamp.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get an individual message in a conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The unique id for the conversation.
    /// * `message_id` - The unique id for the conversation message.
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
    ///         .conversations
    ///         .get_message(
    ///             &"conversation_id".to_string(),
    ///             &"message_id".to_string(),
    ///             &GetMessageQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_message(
        &self,
        conversation_id: &str,
        message_id: &str,
        request: &GetMessageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationMessage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/conversations/{}/messages/{}",
                    conversation_id, message_id
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
}
