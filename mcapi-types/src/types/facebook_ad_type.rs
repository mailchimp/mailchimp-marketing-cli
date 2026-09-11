pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of outreach this object is.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FacebookAdType {
    Regular,
    EmailTouchpoint,
    Plaintext,
    Rss,
    Reconfirm,
    Variate,
    Absplit,
    Automation,
    Facebook,
    Google,
    Autoresponder,
    Transactional,
    Page,
    Website,
    SocialPost,
    Survey,
    CustomerJourney,
    Sms,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FacebookAdType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Regular => serializer.serialize_str("regular"),
            Self::EmailTouchpoint => serializer.serialize_str("email-touchpoint"),
            Self::Plaintext => serializer.serialize_str("plaintext"),
            Self::Rss => serializer.serialize_str("rss"),
            Self::Reconfirm => serializer.serialize_str("reconfirm"),
            Self::Variate => serializer.serialize_str("variate"),
            Self::Absplit => serializer.serialize_str("absplit"),
            Self::Automation => serializer.serialize_str("automation"),
            Self::Facebook => serializer.serialize_str("facebook"),
            Self::Google => serializer.serialize_str("google"),
            Self::Autoresponder => serializer.serialize_str("autoresponder"),
            Self::Transactional => serializer.serialize_str("transactional"),
            Self::Page => serializer.serialize_str("page"),
            Self::Website => serializer.serialize_str("website"),
            Self::SocialPost => serializer.serialize_str("social_post"),
            Self::Survey => serializer.serialize_str("survey"),
            Self::CustomerJourney => serializer.serialize_str("customer_journey"),
            Self::Sms => serializer.serialize_str("sms"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FacebookAdType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "regular" => Ok(Self::Regular),
            "email-touchpoint" => Ok(Self::EmailTouchpoint),
            "plaintext" => Ok(Self::Plaintext),
            "rss" => Ok(Self::Rss),
            "reconfirm" => Ok(Self::Reconfirm),
            "variate" => Ok(Self::Variate),
            "absplit" => Ok(Self::Absplit),
            "automation" => Ok(Self::Automation),
            "facebook" => Ok(Self::Facebook),
            "google" => Ok(Self::Google),
            "autoresponder" => Ok(Self::Autoresponder),
            "transactional" => Ok(Self::Transactional),
            "page" => Ok(Self::Page),
            "website" => Ok(Self::Website),
            "social_post" => Ok(Self::SocialPost),
            "survey" => Ok(Self::Survey),
            "customer_journey" => Ok(Self::CustomerJourney),
            "sms" => Ok(Self::Sms),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FacebookAdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "regular"),
            Self::EmailTouchpoint => write!(f, "email-touchpoint"),
            Self::Plaintext => write!(f, "plaintext"),
            Self::Rss => write!(f, "rss"),
            Self::Reconfirm => write!(f, "reconfirm"),
            Self::Variate => write!(f, "variate"),
            Self::Absplit => write!(f, "absplit"),
            Self::Automation => write!(f, "automation"),
            Self::Facebook => write!(f, "facebook"),
            Self::Google => write!(f, "google"),
            Self::Autoresponder => write!(f, "autoresponder"),
            Self::Transactional => write!(f, "transactional"),
            Self::Page => write!(f, "page"),
            Self::Website => write!(f, "website"),
            Self::SocialPost => write!(f, "social_post"),
            Self::Survey => write!(f, "survey"),
            Self::CustomerJourney => write!(f, "customer_journey"),
            Self::Sms => write!(f, "sms"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
