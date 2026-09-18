pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single email domain's performance
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDomainPerformanceResponseDomainsItem {
    /// The number of bounces at a domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounces: Option<i64>,
    /// The percentage of total bounces from this domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub bounces_pct: Option<f64>,
    /// The number of clicks for a domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    /// The percentage of total clicks from this domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub clicks_pct: Option<f64>,
    /// The number of successful deliveries for a domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    /// The name of the domain (gmail.com, hotmail.com, yahoo.com).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The percentage of total emails that went to this domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub emails_pct: Option<f64>,
    /// The number of emails sent to that specific domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<i64>,
    /// The number of opens for a domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<i64>,
    /// The percentage of total opens from this domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub opens_pct: Option<f64>,
    /// The total number of unsubscribes for a domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubs: Option<i64>,
    /// The percentage of total unsubscribes from this domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub unsubs_pct: Option<f64>,
}

impl ListDomainPerformanceResponseDomainsItem {
    pub fn builder() -> ListDomainPerformanceResponseDomainsItemBuilder {
        <ListDomainPerformanceResponseDomainsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDomainPerformanceResponseDomainsItemBuilder {
    bounces: Option<i64>,
    bounces_pct: Option<f64>,
    clicks: Option<i64>,
    clicks_pct: Option<f64>,
    delivered: Option<i64>,
    domain: Option<String>,
    emails_pct: Option<f64>,
    emails_sent: Option<i64>,
    opens: Option<i64>,
    opens_pct: Option<f64>,
    unsubs: Option<i64>,
    unsubs_pct: Option<f64>,
}

impl ListDomainPerformanceResponseDomainsItemBuilder {
    pub fn bounces(mut self, value: i64) -> Self {
        self.bounces = Some(value);
        self
    }

    pub fn bounces_pct(mut self, value: f64) -> Self {
        self.bounces_pct = Some(value);
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn clicks_pct(mut self, value: f64) -> Self {
        self.clicks_pct = Some(value);
        self
    }

    pub fn delivered(mut self, value: i64) -> Self {
        self.delivered = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn emails_pct(mut self, value: f64) -> Self {
        self.emails_pct = Some(value);
        self
    }

    pub fn emails_sent(mut self, value: i64) -> Self {
        self.emails_sent = Some(value);
        self
    }

    pub fn opens(mut self, value: i64) -> Self {
        self.opens = Some(value);
        self
    }

    pub fn opens_pct(mut self, value: f64) -> Self {
        self.opens_pct = Some(value);
        self
    }

    pub fn unsubs(mut self, value: i64) -> Self {
        self.unsubs = Some(value);
        self
    }

    pub fn unsubs_pct(mut self, value: f64) -> Self {
        self.unsubs_pct = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDomainPerformanceResponseDomainsItem`].
    pub fn build(self) -> Result<ListDomainPerformanceResponseDomainsItem, BuildError> {
        Ok(ListDomainPerformanceResponseDomainsItem {
            bounces: self.bounces,
            bounces_pct: self.bounces_pct,
            clicks: self.clicks,
            clicks_pct: self.clicks_pct,
            delivered: self.delivered,
            domain: self.domain,
            emails_pct: self.emails_pct,
            emails_sent: self.emails_sent,
            opens: self.opens,
            opens_pct: self.opens_pct,
            unsubs: self.unsubs,
            unsubs_pct: self.unsubs_pct,
        })
    }
}
