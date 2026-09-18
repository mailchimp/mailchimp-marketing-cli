use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AudiencesClient {
    pub http_client: HttpClient,
}

impl AudiencesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a list of omni-channel contacts for a given audience.
    ///
    /// # Arguments
    ///
    /// * `audience_id` - The unique ID for the audience.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `cursor` - Paginate through a collection of records by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request. Default value fetches the first "page" of results.
    /// * `created_before` - Restricts the response to contacts created at or before the specified time (inclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00.
    /// * `created_since` - Restricts the response to contacts created after the specified time (exclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00.
    /// * `updated_before` - Restricts the response to contacts updated at or before the specified time (inclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00.
    /// * `updated_since` - Restricts the response to contacts updated after the specified time (exclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00.
    /// * `sort_field` - Specifies the field to sort the returned contacts by.
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
    /// use mailchimp_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpClient::new(config).expect("Failed to build client");
    ///     client
    ///         .audiences
    ///         .get_audience_contact_list(
    ///             &"audience_id".to_string(),
    ///             &GetAudienceContactListQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 cursor: None,
    ///                 created_before: None,
    ///                 created_since: None,
    ///                 updated_before: None,
    ///                 updated_since: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_audience_contact_list(
        &self,
        audience_id: &str,
        request: &GetAudienceContactListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAudienceContactListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/audiences/{}/contacts", audience_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .string("cursor", request.cursor.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_since", request.created_since.clone())
                    .datetime("updated_before", request.updated_before.clone())
                    .datetime("updated_since", request.updated_since.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new omni-channel contact for an audience.
    ///
    /// # Arguments
    ///
    /// * `audience_id` - The unique ID for the audience.
    /// * `merge_field_validation_mode` - Defines how merge field validation is handled. When set to `ignore_required_checks`, the API does not raise an error if required merge fields are missing from the request. When set to `strict`, the API enforces validation and returns an error if any required merge field is not provided. If this setting is omitted, `strict` is applied by default.
    /// * `data_mode` - Indicates the data processing mode. In `historical` mode, contact data changes do not trigger automations or webhooks. In `live mode`, such changes do trigger them.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpClient::new(config).expect("Failed to build client");
    ///     client
    ///         .audiences
    ///         .create_audience_contact(
    ///             &"audience_id".to_string(),
    ///             &CreateAudienceContactRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_audience_contact(
        &self,
        audience_id: &str,
        request: &CreateAudienceContactRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudiencesContact, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/audiences/{}/contacts", audience_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize(
                        "merge_field_validation_mode",
                        request.merge_field_validation_mode.clone(),
                    )
                    .serialize("data_mode", request.data_mode.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a specific omni-channel contact in an audience.
    ///
    /// # Arguments
    ///
    /// * `audience_id` - The unique ID for the audience.
    /// * `contact_id` - A unique identifier for the contact, which can be a Mailchimp contact ID or a channel hash. A channel hash must follow the format email:[md5_hash] (where the hash is the MD5 of the lowercased email address) or sms:[sha256_hash] (where the hash is the SHA256 of the E.164-formatted phone number).
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
    /// use mailchimp_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpClient::new(config).expect("Failed to build client");
    ///     client
    ///         .audiences
    ///         .get_audience_contact(
    ///             &"audience_id".to_string(),
    ///             &"contact_id".to_string(),
    ///             &GetAudienceContactQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_audience_contact(
        &self,
        audience_id: &str,
        contact_id: &str,
        request: &GetAudienceContactQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudiencesContact, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/audiences/{}/contacts/{}", audience_id, contact_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Update an existing omni-channel contact.
    ///
    /// # Arguments
    ///
    /// * `audience_id` - The unique ID for the audience.
    /// * `contact_id` - The unique id for the contact.
    /// * `merge_field_validation_mode` - Defines how merge field validation is handled. When set to `ignore_required_checks`, the API does not raise an error if required merge fields are missing from the request. When set to `strict`, the API enforces validation and returns an error if any required merge field is not provided. If this setting is omitted, `strict` is applied by default.
    /// * `data_mode` - Indicates the data processing mode. In `historical` mode, contact data changes do not trigger automations or webhooks. In `live mode`, such changes do trigger them.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpClient::new(config).expect("Failed to build client");
    ///     client
    ///         .audiences
    ///         .patch_audience_contact(
    ///             &"audience_id".to_string(),
    ///             &"contact_id".to_string(),
    ///             &PatchAudienceContactRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn patch_audience_contact(
        &self,
        audience_id: &str,
        contact_id: &str,
        request: &PatchAudienceContactRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudiencesContact, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/audiences/{}/contacts/{}", audience_id, contact_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize(
                        "merge_field_validation_mode",
                        request.merge_field_validation_mode.clone(),
                    )
                    .serialize("data_mode", request.data_mode.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Archives a Contact.
    ///
    /// # Arguments
    ///
    /// * `audience_id` - The unique ID for the audience.
    /// * `contact_id` - The unique id for the contact.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpClient::new(config).expect("Failed to build client");
    ///     client
    ///         .audiences
    ///         .post_audiences_contacts_actions_archive(
    ///             &"audience_id".to_string(),
    ///             &"contact_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn post_audiences_contacts_actions_archive(
        &self,
        audience_id: &str,
        contact_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/audiences/{}/contacts/{}/actions/archive",
                    audience_id, contact_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Forgets a Contact.
    ///
    /// # Arguments
    ///
    /// * `audience_id` - The unique ID for the audience.
    /// * `contact_id` - The unique id for the contact.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpClient::new(config).expect("Failed to build client");
    ///     client
    ///         .audiences
    ///         .post_audiences_contacts_actions_forget(
    ///             &"audience_id".to_string(),
    ///             &"contact_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn post_audiences_contacts_actions_forget(
        &self,
        audience_id: &str,
        contact_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/audiences/{}/contacts/{}/actions/forget",
                    audience_id, contact_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
