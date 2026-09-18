//! # Mailchimp API SDK
//!
//! The official Rust SDK for the Mailchimp API.
//!
//! ## Getting Started
//!
//! ```rust
//! use mailchimp_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         token: Some("<token>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = MailchimpClient::new(config).expect("Failed to build client");
//!     client
//!         .audiences
//!         .get_audience_contact_list(
//!             &"audience_id".to_string(),
//!             &GetAudienceContactListQueryRequest {
//!                 fields: vec![],
//!                 exclude_fields: vec![],
//!                 count: None,
//!                 cursor: None,
//!                 created_before: None,
//!                 created_since: None,
//!                 updated_before: None,
//!                 updated_since: None,
//!                 sort_field: None,
//!                 sort_dir: None,
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
