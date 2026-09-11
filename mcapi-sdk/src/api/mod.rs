//! API client and types for the Mailchimp API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    AccountExportsClient, ActivityFeedClient, ApiClient, AuthorizedAppsClient, AutomationsClient,
    BatchWebhooksClient, BatchesClient, CampaignFoldersClient, CampaignsClient,
    ConnectedSitesClient, ConversationsClient, CustomerJourneysClient, EcommerceClient,
    FacebookAdsClient, FileManagerClient, LandingPagesClient, ListsClient, PingClient,
    ReportingClient, ReportsClient, RootClient, SearchCampaignsClient, SearchMembersClient,
    SmsCampaignsClient, SurveysClient, TemplateFoldersClient, TemplatesClient,
    VerifiedDomainsClient,
};

pub use mcapi_types::*;
