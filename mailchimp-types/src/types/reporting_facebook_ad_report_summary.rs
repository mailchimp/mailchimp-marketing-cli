pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Report summary of facebook ad
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReportingFacebookAdReportSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_daily_budget: Option<ReportingFacebookAdReportSummaryAverageDailyBudget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_order_amount: Option<ReportingFacebookAdReportSummaryAverageOrderAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_click: Option<ReportingFacebookAdReportSummaryCostPerClick>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<ReportingFacebookAdReportSummaryEcommerce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_at: Option<ReportingFacebookAdReportSummaryExtendedAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_time_buyers: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_extended_ad_duration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impressions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reach: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub return_on_investment: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_orders: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_products_sold: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_clicks: Option<i64>,
}

impl ReportingFacebookAdReportSummary {
    pub fn builder() -> ReportingFacebookAdReportSummaryBuilder {
        <ReportingFacebookAdReportSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdReportSummaryBuilder {
    average_daily_budget: Option<ReportingFacebookAdReportSummaryAverageDailyBudget>,
    average_order_amount: Option<ReportingFacebookAdReportSummaryAverageOrderAmount>,
    click_rate: Option<f64>,
    clicks: Option<i64>,
    comments: Option<i64>,
    cost_per_click: Option<ReportingFacebookAdReportSummaryCostPerClick>,
    ecommerce: Option<ReportingFacebookAdReportSummaryEcommerce>,
    extended_at: Option<ReportingFacebookAdReportSummaryExtendedAt>,
    first_time_buyers: Option<i64>,
    has_extended_ad_duration: Option<bool>,
    impressions: Option<i64>,
    likes: Option<i64>,
    reach: Option<i64>,
    return_on_investment: Option<f64>,
    shares: Option<i64>,
    total_orders: Option<i64>,
    total_products_sold: Option<i64>,
    unique_clicks: Option<i64>,
}

impl ReportingFacebookAdReportSummaryBuilder {
    pub fn average_daily_budget(mut self, value: ReportingFacebookAdReportSummaryAverageDailyBudget) -> Self {
        self.average_daily_budget = Some(value);
        self
    }

    pub fn average_order_amount(mut self, value: ReportingFacebookAdReportSummaryAverageOrderAmount) -> Self {
        self.average_order_amount = Some(value);
        self
    }

    pub fn click_rate(mut self, value: f64) -> Self {
        self.click_rate = Some(value);
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn comments(mut self, value: i64) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn cost_per_click(mut self, value: ReportingFacebookAdReportSummaryCostPerClick) -> Self {
        self.cost_per_click = Some(value);
        self
    }

    pub fn ecommerce(mut self, value: ReportingFacebookAdReportSummaryEcommerce) -> Self {
        self.ecommerce = Some(value);
        self
    }

    pub fn extended_at(mut self, value: ReportingFacebookAdReportSummaryExtendedAt) -> Self {
        self.extended_at = Some(value);
        self
    }

    pub fn first_time_buyers(mut self, value: i64) -> Self {
        self.first_time_buyers = Some(value);
        self
    }

    pub fn has_extended_ad_duration(mut self, value: bool) -> Self {
        self.has_extended_ad_duration = Some(value);
        self
    }

    pub fn impressions(mut self, value: i64) -> Self {
        self.impressions = Some(value);
        self
    }

    pub fn likes(mut self, value: i64) -> Self {
        self.likes = Some(value);
        self
    }

    pub fn reach(mut self, value: i64) -> Self {
        self.reach = Some(value);
        self
    }

    pub fn return_on_investment(mut self, value: f64) -> Self {
        self.return_on_investment = Some(value);
        self
    }

    pub fn shares(mut self, value: i64) -> Self {
        self.shares = Some(value);
        self
    }

    pub fn total_orders(mut self, value: i64) -> Self {
        self.total_orders = Some(value);
        self
    }

    pub fn total_products_sold(mut self, value: i64) -> Self {
        self.total_products_sold = Some(value);
        self
    }

    pub fn unique_clicks(mut self, value: i64) -> Self {
        self.unique_clicks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdReportSummary`].
    pub fn build(self) -> Result<ReportingFacebookAdReportSummary, BuildError> {
        Ok(ReportingFacebookAdReportSummary {
            average_daily_budget: self.average_daily_budget,
            average_order_amount: self.average_order_amount,
            click_rate: self.click_rate,
            clicks: self.clicks,
            comments: self.comments,
            cost_per_click: self.cost_per_click,
            ecommerce: self.ecommerce,
            extended_at: self.extended_at,
            first_time_buyers: self.first_time_buyers,
            has_extended_ad_duration: self.has_extended_ad_duration,
            impressions: self.impressions,
            likes: self.likes,
            reach: self.reach,
            return_on_investment: self.return_on_investment,
            shares: self.shares,
            total_orders: self.total_orders,
            total_products_sold: self.total_products_sold,
            unique_clicks: self.unique_clicks,
        })
    }
}
