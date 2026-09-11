use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct FileManagerClient {
    pub http_client: HttpClient,
}

impl FileManagerClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get information about the file-manager endpoint's resources
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
    ///     client.file_manager.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ListFileManagerResponseItem>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "3.0/file-manager", None, None, options)
            .await
    }

    /// Get a list of available images and files stored in the File Manager for the account.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - The file type for the File Manager file.
    /// * `created_by` - The Mailchimp account user who created the File Manager file.
    /// * `before_created_at` - Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_created_at` - Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
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
    ///         .file_manager
    ///         .list_files(
    ///             &ListFilesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 created_by: None,
    ///                 before_created_at: None,
    ///                 since_created_at: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_files(
        &self,
        request: &ListFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFilesFileManagerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/file-manager/files",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("type", request.r#type.clone())
                    .string("created_by", request.created_by.clone())
                    .string("before_created_at", request.before_created_at.clone())
                    .string("since_created_at", request.since_created_at.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_files_paginated(
        &self,
        request: &ListFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("type", request.r#type.clone())
            .string("created_by", request.created_by.clone())
            .string("before_created_at", request.before_created_at.clone())
            .string("since_created_at", request.since_created_at.clone())
            .serialize("sort_field", request.sort_field.clone())
            .serialize("sort_dir", request.sort_dir.clone())
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
                            "3.0/file-manager/files",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("files")
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

    /// Upload a new image or file to the File Manager.
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
    ///         .file_manager
    ///         .create_file(
    ///             &CreateFileFileManagerRequest {
    ///                 file_data: "file_data".to_string(),
    ///                 name: "name".to_string(),
    ///                 folder_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_file(
        &self,
        request: &CreateFileFileManagerRequest,
        options: Option<RequestOptions>,
    ) -> Result<GalleryFile, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/file-manager/files",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific file in the File Manager.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique id for the File Manager file.
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
    ///         .file_manager
    ///         .get_file(
    ///             &"file_id".to_string(),
    ///             &GetFileQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_file(
        &self,
        file_id: &str,
        request: &GetFileQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GalleryFile, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/file-manager/files/{}", file_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Remove a specific file from the File Manager.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique id for the File Manager file.
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
    ///         .file_manager
    ///         .delete_file(&"file_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_file(
        &self,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/file-manager/files/{}", file_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a file in the File Manager.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique id for the File Manager file.
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
    ///         .file_manager
    ///         .update_file(
    ///             &"file_id".to_string(),
    ///             &UpdateFileFileManagerRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_file(
        &self,
        file_id: &str,
        request: &UpdateFileFileManagerRequest,
        options: Option<RequestOptions>,
    ) -> Result<GalleryFile, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/file-manager/files/{}", file_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of all folders in the File Manager.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `created_by` - The Mailchimp account user who created the File Manager file.
    /// * `before_created_at` - Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_created_at` - Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
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
    ///         .file_manager
    ///         .list_folders(
    ///             &ListFoldersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 created_by: None,
    ///                 before_created_at: None,
    ///                 since_created_at: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_folders(
        &self,
        request: &ListFoldersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFoldersFileManagerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/file-manager/folders",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("created_by", request.created_by.clone())
                    .string("before_created_at", request.before_created_at.clone())
                    .string("since_created_at", request.since_created_at.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_folders_paginated(
        &self,
        request: &ListFoldersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("created_by", request.created_by.clone())
            .string("before_created_at", request.before_created_at.clone())
            .string("since_created_at", request.since_created_at.clone())
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
                            "3.0/file-manager/folders",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("folders")
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

    /// Create a new folder in the File Manager.
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
    ///         .file_manager
    ///         .create_folder(
    ///             &CreateFolderFileManagerRequest {
    ///                 name: "name".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_folder(
        &self,
        request: &CreateFolderFileManagerRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateFolderFileManagerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/file-manager/folders",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific folder in the File Manager.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the File Manager folder.
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
    ///         .file_manager
    ///         .get_folder(
    ///             &"folder_id".to_string(),
    ///             &GetFolderQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_folder(
        &self,
        folder_id: &str,
        request: &GetFolderQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetFolderFileManagerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/file-manager/folders/{}", folder_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific folder in the File Manager.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the File Manager folder.
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
    ///         .file_manager
    ///         .delete_folder(&"folder_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_folder(
        &self,
        folder_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/file-manager/folders/{}", folder_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific File Manager folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the File Manager folder.
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
    ///         .file_manager
    ///         .update_folder(
    ///             &"folder_id".to_string(),
    ///             &UpdateFolderFileManagerRequest {
    ///                 name: "name".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_folder(
        &self,
        folder_id: &str,
        request: &UpdateFolderFileManagerRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateFolderFileManagerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/file-manager/folders/{}", folder_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of available images and files stored in this folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the File Manager folder.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - The file type for the File Manager file.
    /// * `created_by` - The Mailchimp account user who created the File Manager file.
    /// * `before_created_at` - Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_created_at` - Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
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
    ///         .file_manager
    ///         .list_folder_files(
    ///             &"folder_id".to_string(),
    ///             &ListFolderFilesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 created_by: None,
    ///                 before_created_at: None,
    ///                 since_created_at: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_folder_files(
        &self,
        folder_id: &str,
        request: &ListFolderFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFolderFilesFileManagerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/file-manager/folders/{}/files", folder_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("type", request.r#type.clone())
                    .string("created_by", request.created_by.clone())
                    .string("before_created_at", request.before_created_at.clone())
                    .string("since_created_at", request.since_created_at.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_folder_files_paginated(
        &self,
        folder_id: &str,
        request: &ListFolderFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .string("type", request.r#type.clone())
            .string("created_by", request.created_by.clone())
            .string("before_created_at", request.before_created_at.clone())
            .string("since_created_at", request.since_created_at.clone())
            .serialize("sort_field", request.sort_field.clone())
            .serialize("sort_dir", request.sort_dir.clone())
            .build();
        let options_clone = options.clone();
        let folder_id_clone = folder_id.to_string();

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
                let folder_id_for_async = folder_id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("3.0/file-manager/folders/{}/files", folder_id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("files")
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
}
