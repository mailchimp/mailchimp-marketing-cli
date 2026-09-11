pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The days of the week to send a daily RSS Campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCampaignsRequestRssOptsScheduleDailySend {
    /// Sends the daily RSS Campaign on Fridays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friday: Option<bool>,
    /// Sends the daily RSS Campaign on Mondays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday: Option<bool>,
    /// Sends the daily RSS Campaign on Saturdays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturday: Option<bool>,
    /// Sends the daily RSS Campaign on Sundays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday: Option<bool>,
    /// Sends the daily RSS Campaign on Thursdays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thursday: Option<bool>,
    /// Sends the daily RSS Campaign on Tuesdays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<bool>,
    /// Sends the daily RSS Campaign on Wednesdays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<bool>,
}

impl UpdateCampaignsRequestRssOptsScheduleDailySend {
    pub fn builder() -> UpdateCampaignsRequestRssOptsScheduleDailySendBuilder {
        <UpdateCampaignsRequestRssOptsScheduleDailySendBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCampaignsRequestRssOptsScheduleDailySendBuilder {
    friday: Option<bool>,
    monday: Option<bool>,
    saturday: Option<bool>,
    sunday: Option<bool>,
    thursday: Option<bool>,
    tuesday: Option<bool>,
    wednesday: Option<bool>,
}

impl UpdateCampaignsRequestRssOptsScheduleDailySendBuilder {
    pub fn friday(mut self, value: bool) -> Self {
        self.friday = Some(value);
        self
    }

    pub fn monday(mut self, value: bool) -> Self {
        self.monday = Some(value);
        self
    }

    pub fn saturday(mut self, value: bool) -> Self {
        self.saturday = Some(value);
        self
    }

    pub fn sunday(mut self, value: bool) -> Self {
        self.sunday = Some(value);
        self
    }

    pub fn thursday(mut self, value: bool) -> Self {
        self.thursday = Some(value);
        self
    }

    pub fn tuesday(mut self, value: bool) -> Self {
        self.tuesday = Some(value);
        self
    }

    pub fn wednesday(mut self, value: bool) -> Self {
        self.wednesday = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCampaignsRequestRssOptsScheduleDailySend`].
    pub fn build(self) -> Result<UpdateCampaignsRequestRssOptsScheduleDailySend, BuildError> {
        Ok(UpdateCampaignsRequestRssOptsScheduleDailySend {
            friday: self.friday,
            monday: self.monday,
            saturday: self.saturday,
            sunday: self.sunday,
            thursday: self.thursday,
            tuesday: self.tuesday,
            wednesday: self.wednesday,
        })
    }
}
