pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The API root resource links to all other resources available in the API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListRootResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListRootResponseLinksItem>>,
    /// The Mailchimp account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The user-specified industry associated with the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_industry: Option<String>,
    /// The name of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// The timezone currently set for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_timezone: Option<String>,
    /// URL of the avatar for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Information about the account contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ListRootResponseContact>,
    /// The account email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The first name tied to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Date of first payment for monthly plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_payment: Option<ListRootResponseFirstPayment>,
    /// The [average campaign statistics](https://mailchimp.com/resources/research/email-marketing-benchmarks/?utm_source=mc-api&utm_medium=docs&utm_campaign=apidocs) for all campaigns in the account's specified industry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_stats: Option<ListRootResponseIndustryStats>,
    /// The date and time of the last login for this account in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_login: Option<DateTime<FixedOffset>>,
    /// The last name tied to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The ID associated with the user who owns this API key. If you can login to multiple accounts, this ID will be the same for each account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_id: Option<String>,
    /// The date and time that the account was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub member_since: Option<DateTime<FixedOffset>>,
    /// The type of pricing plan the account is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan_type: Option<ListRootResponsePricingPlanType>,
    /// Legacy - whether the account includes [Mailchimp Pro](https://mailchimp.com/help/about-legacy-pricing-plan/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro_enabled: Option<bool>,
    /// The [user role](https://mailchimp.com/help/manage-user-levels-in-your-account/) for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The total number of subscribers across all lists in the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_subscribers: Option<i64>,
    /// The username tied to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl ListRootResponse {
    pub fn builder() -> ListRootResponseBuilder {
        <ListRootResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRootResponseBuilder {
    links: Option<Vec<ListRootResponseLinksItem>>,
    account_id: Option<String>,
    account_industry: Option<String>,
    account_name: Option<String>,
    account_timezone: Option<String>,
    avatar_url: Option<String>,
    contact: Option<ListRootResponseContact>,
    email: Option<String>,
    first_name: Option<String>,
    first_payment: Option<ListRootResponseFirstPayment>,
    industry_stats: Option<ListRootResponseIndustryStats>,
    last_login: Option<DateTime<FixedOffset>>,
    last_name: Option<String>,
    login_id: Option<String>,
    member_since: Option<DateTime<FixedOffset>>,
    pricing_plan_type: Option<ListRootResponsePricingPlanType>,
    pro_enabled: Option<bool>,
    role: Option<String>,
    total_subscribers: Option<i64>,
    username: Option<String>,
}

impl ListRootResponseBuilder {
    pub fn links(mut self, value: Vec<ListRootResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn account_industry(mut self, value: impl Into<String>) -> Self {
        self.account_industry = Some(value.into());
        self
    }

    pub fn account_name(mut self, value: impl Into<String>) -> Self {
        self.account_name = Some(value.into());
        self
    }

    pub fn account_timezone(mut self, value: impl Into<String>) -> Self {
        self.account_timezone = Some(value.into());
        self
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    pub fn contact(mut self, value: ListRootResponseContact) -> Self {
        self.contact = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn first_payment(mut self, value: ListRootResponseFirstPayment) -> Self {
        self.first_payment = Some(value);
        self
    }

    pub fn industry_stats(mut self, value: ListRootResponseIndustryStats) -> Self {
        self.industry_stats = Some(value);
        self
    }

    pub fn last_login(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_login = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn login_id(mut self, value: impl Into<String>) -> Self {
        self.login_id = Some(value.into());
        self
    }

    pub fn member_since(mut self, value: DateTime<FixedOffset>) -> Self {
        self.member_since = Some(value);
        self
    }

    pub fn pricing_plan_type(mut self, value: ListRootResponsePricingPlanType) -> Self {
        self.pricing_plan_type = Some(value);
        self
    }

    pub fn pro_enabled(mut self, value: bool) -> Self {
        self.pro_enabled = Some(value);
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn total_subscribers(mut self, value: i64) -> Self {
        self.total_subscribers = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListRootResponse`].
    pub fn build(self) -> Result<ListRootResponse, BuildError> {
        Ok(ListRootResponse {
            links: self.links,
            account_id: self.account_id,
            account_industry: self.account_industry,
            account_name: self.account_name,
            account_timezone: self.account_timezone,
            avatar_url: self.avatar_url,
            contact: self.contact,
            email: self.email,
            first_name: self.first_name,
            first_payment: self.first_payment,
            industry_stats: self.industry_stats,
            last_login: self.last_login,
            last_name: self.last_name,
            login_id: self.login_id,
            member_since: self.member_since,
            pricing_plan_type: self.pricing_plan_type,
            pro_enabled: self.pro_enabled,
            role: self.role,
            total_subscribers: self.total_subscribers,
            username: self.username,
        })
    }
}
