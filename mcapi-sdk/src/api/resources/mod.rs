//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **root**
//! - **AccountExports**
//! - **ActivityFeed**
//! - **AuthorizedApps**
//! - **automations**
//! - **BatchWebhooks**
//! - **batches**
//! - **CampaignFolders**
//! - **campaigns**
//! - **ConnectedSites**
//! - **conversations**
//! - **CustomerJourneys**
//! - **ecommerce**
//! - **FacebookAds**
//! - **FileManager**
//! - **LandingPages**
//! - **lists**
//! - **surveys**
//! - **ping**
//! - **reporting**
//! - **reports**
//! - **SearchCampaigns**
//! - **SmsCampaigns**
//! - **SearchMembers**
//! - **TemplateFolders**
//! - **templates**
//! - **VerifiedDomains**

use crate::{ApiError, ClientConfig};

pub mod account_exports;
pub mod activity_feed;
pub mod authorized_apps;
pub mod automations;
pub mod batch_webhooks;
pub mod batches;
pub mod campaign_folders;
pub mod campaigns;
pub mod connected_sites;
pub mod conversations;
pub mod customer_journeys;
pub mod ecommerce;
pub mod facebook_ads;
pub mod file_manager;
pub mod landing_pages;
pub mod lists;
pub mod ping;
pub mod reporting;
pub mod reports;
pub mod root;
pub mod search_campaigns;
pub mod search_members;
pub mod sms_campaigns;
pub mod surveys;
pub mod template_folders;
pub mod templates;
pub mod verified_domains;
pub struct ApiClient {
    pub config: ClientConfig,
    pub root: RootClient,
    pub account_exports: AccountExportsClient,
    pub activity_feed: ActivityFeedClient,
    pub authorized_apps: AuthorizedAppsClient,
    pub automations: AutomationsClient,
    pub batch_webhooks: BatchWebhooksClient,
    pub batches: BatchesClient,
    pub campaign_folders: CampaignFoldersClient,
    pub campaigns: CampaignsClient,
    pub connected_sites: ConnectedSitesClient,
    pub conversations: ConversationsClient,
    pub customer_journeys: CustomerJourneysClient,
    pub ecommerce: EcommerceClient,
    pub facebook_ads: FacebookAdsClient,
    pub file_manager: FileManagerClient,
    pub landing_pages: LandingPagesClient,
    pub lists: ListsClient,
    pub surveys: SurveysClient,
    pub ping: PingClient,
    pub reporting: ReportingClient,
    pub reports: ReportsClient,
    pub search_campaigns: SearchCampaignsClient,
    pub sms_campaigns: SmsCampaignsClient,
    pub search_members: SearchMembersClient,
    pub template_folders: TemplateFoldersClient,
    pub templates: TemplatesClient,
    pub verified_domains: VerifiedDomainsClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            root: RootClient::new(config.clone())?,
            account_exports: AccountExportsClient::new(config.clone())?,
            activity_feed: ActivityFeedClient::new(config.clone())?,
            authorized_apps: AuthorizedAppsClient::new(config.clone())?,
            automations: AutomationsClient::new(config.clone())?,
            batch_webhooks: BatchWebhooksClient::new(config.clone())?,
            batches: BatchesClient::new(config.clone())?,
            campaign_folders: CampaignFoldersClient::new(config.clone())?,
            campaigns: CampaignsClient::new(config.clone())?,
            connected_sites: ConnectedSitesClient::new(config.clone())?,
            conversations: ConversationsClient::new(config.clone())?,
            customer_journeys: CustomerJourneysClient::new(config.clone())?,
            ecommerce: EcommerceClient::new(config.clone())?,
            facebook_ads: FacebookAdsClient::new(config.clone())?,
            file_manager: FileManagerClient::new(config.clone())?,
            landing_pages: LandingPagesClient::new(config.clone())?,
            lists: ListsClient::new(config.clone())?,
            surveys: SurveysClient::new(config.clone())?,
            ping: PingClient::new(config.clone())?,
            reporting: ReportingClient::new(config.clone())?,
            reports: ReportsClient::new(config.clone())?,
            search_campaigns: SearchCampaignsClient::new(config.clone())?,
            sms_campaigns: SmsCampaignsClient::new(config.clone())?,
            search_members: SearchMembersClient::new(config.clone())?,
            template_folders: TemplateFoldersClient::new(config.clone())?,
            templates: TemplatesClient::new(config.clone())?,
            verified_domains: VerifiedDomainsClient::new(config.clone())?,
        })
    }
}

pub use account_exports::AccountExportsClient;
pub use activity_feed::ActivityFeedClient;
pub use authorized_apps::AuthorizedAppsClient;
pub use automations::AutomationsClient;
pub use batch_webhooks::BatchWebhooksClient;
pub use batches::BatchesClient;
pub use campaign_folders::CampaignFoldersClient;
pub use campaigns::CampaignsClient;
pub use connected_sites::ConnectedSitesClient;
pub use conversations::ConversationsClient;
pub use customer_journeys::CustomerJourneysClient;
pub use ecommerce::EcommerceClient;
pub use facebook_ads::FacebookAdsClient;
pub use file_manager::FileManagerClient;
pub use landing_pages::LandingPagesClient;
pub use lists::ListsClient;
pub use ping::PingClient;
pub use reporting::ReportingClient;
pub use reports::ReportsClient;
pub use root::RootClient;
pub use search_campaigns::SearchCampaignsClient;
pub use search_members::SearchMembersClient;
pub use sms_campaigns::SmsCampaignsClient;
pub use surveys::SurveysClient;
pub use template_folders::TemplateFoldersClient;
pub use templates::TemplatesClient;
pub use verified_domains::VerifiedDomainsClient;
