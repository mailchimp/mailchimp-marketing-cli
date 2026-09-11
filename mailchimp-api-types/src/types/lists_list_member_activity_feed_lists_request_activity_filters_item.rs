pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMemberActivityFeedListsRequestActivityFiltersItem {
    Bounce,
    Click,
    Conversation,
    EcommerceSignup,
    Event,
    WebEngagement,
    GenericSignup,
    LandingPageSignup,
    MarketingPermission,
    Note,
    Open,
    Order,
    PostcardSent,
    Sent,
    Signup,
    SquatterSignup,
    Unsub,
    WebsiteSignup,
    SurveyResponse,
    SmsBulkSent,
    InboxThread,
    QboPaymentLink,
    VideoCallTranscripts,
    WhatsappBulkSent,
    WhatsappDelivered,
    WhatsappLinkClick,
    WhatsappSubscribe,
    WhatsappSubscriptionPhoneUpdated,
    WhatsappUnsubscribe,
    WhatsappUsedKeyword,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMemberActivityFeedListsRequestActivityFiltersItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bounce => serializer.serialize_str("bounce"),
            Self::Click => serializer.serialize_str("click"),
            Self::Conversation => serializer.serialize_str("conversation"),
            Self::EcommerceSignup => serializer.serialize_str("ecommerce_signup"),
            Self::Event => serializer.serialize_str("event"),
            Self::WebEngagement => serializer.serialize_str("web_engagement"),
            Self::GenericSignup => serializer.serialize_str("generic_signup"),
            Self::LandingPageSignup => serializer.serialize_str("landing_page_signup"),
            Self::MarketingPermission => serializer.serialize_str("marketing_permission"),
            Self::Note => serializer.serialize_str("note"),
            Self::Open => serializer.serialize_str("open"),
            Self::Order => serializer.serialize_str("order"),
            Self::PostcardSent => serializer.serialize_str("postcard_sent"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Signup => serializer.serialize_str("signup"),
            Self::SquatterSignup => serializer.serialize_str("squatter_signup"),
            Self::Unsub => serializer.serialize_str("unsub"),
            Self::WebsiteSignup => serializer.serialize_str("website_signup"),
            Self::SurveyResponse => serializer.serialize_str("survey_response"),
            Self::SmsBulkSent => serializer.serialize_str("sms_bulk_sent"),
            Self::InboxThread => serializer.serialize_str("inbox_thread"),
            Self::QboPaymentLink => serializer.serialize_str("qbo_payment_link"),
            Self::VideoCallTranscripts => serializer.serialize_str("video_call_transcripts"),
            Self::WhatsappBulkSent => serializer.serialize_str("whatsapp_bulk_sent"),
            Self::WhatsappDelivered => serializer.serialize_str("whatsapp_delivered"),
            Self::WhatsappLinkClick => serializer.serialize_str("whatsapp_link_click"),
            Self::WhatsappSubscribe => serializer.serialize_str("whatsapp_subscribe"),
            Self::WhatsappSubscriptionPhoneUpdated => serializer.serialize_str("whatsapp_subscription_phone_updated"),
            Self::WhatsappUnsubscribe => serializer.serialize_str("whatsapp_unsubscribe"),
            Self::WhatsappUsedKeyword => serializer.serialize_str("whatsapp_used_keyword"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMemberActivityFeedListsRequestActivityFiltersItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bounce" => Ok(Self::Bounce),
            "click" => Ok(Self::Click),
            "conversation" => Ok(Self::Conversation),
            "ecommerce_signup" => Ok(Self::EcommerceSignup),
            "event" => Ok(Self::Event),
            "web_engagement" => Ok(Self::WebEngagement),
            "generic_signup" => Ok(Self::GenericSignup),
            "landing_page_signup" => Ok(Self::LandingPageSignup),
            "marketing_permission" => Ok(Self::MarketingPermission),
            "note" => Ok(Self::Note),
            "open" => Ok(Self::Open),
            "order" => Ok(Self::Order),
            "postcard_sent" => Ok(Self::PostcardSent),
            "sent" => Ok(Self::Sent),
            "signup" => Ok(Self::Signup),
            "squatter_signup" => Ok(Self::SquatterSignup),
            "unsub" => Ok(Self::Unsub),
            "website_signup" => Ok(Self::WebsiteSignup),
            "survey_response" => Ok(Self::SurveyResponse),
            "sms_bulk_sent" => Ok(Self::SmsBulkSent),
            "inbox_thread" => Ok(Self::InboxThread),
            "qbo_payment_link" => Ok(Self::QboPaymentLink),
            "video_call_transcripts" => Ok(Self::VideoCallTranscripts),
            "whatsapp_bulk_sent" => Ok(Self::WhatsappBulkSent),
            "whatsapp_delivered" => Ok(Self::WhatsappDelivered),
            "whatsapp_link_click" => Ok(Self::WhatsappLinkClick),
            "whatsapp_subscribe" => Ok(Self::WhatsappSubscribe),
            "whatsapp_subscription_phone_updated" => Ok(Self::WhatsappSubscriptionPhoneUpdated),
            "whatsapp_unsubscribe" => Ok(Self::WhatsappUnsubscribe),
            "whatsapp_used_keyword" => Ok(Self::WhatsappUsedKeyword),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMemberActivityFeedListsRequestActivityFiltersItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bounce => write!(f, "bounce"),
            Self::Click => write!(f, "click"),
            Self::Conversation => write!(f, "conversation"),
            Self::EcommerceSignup => write!(f, "ecommerce_signup"),
            Self::Event => write!(f, "event"),
            Self::WebEngagement => write!(f, "web_engagement"),
            Self::GenericSignup => write!(f, "generic_signup"),
            Self::LandingPageSignup => write!(f, "landing_page_signup"),
            Self::MarketingPermission => write!(f, "marketing_permission"),
            Self::Note => write!(f, "note"),
            Self::Open => write!(f, "open"),
            Self::Order => write!(f, "order"),
            Self::PostcardSent => write!(f, "postcard_sent"),
            Self::Sent => write!(f, "sent"),
            Self::Signup => write!(f, "signup"),
            Self::SquatterSignup => write!(f, "squatter_signup"),
            Self::Unsub => write!(f, "unsub"),
            Self::WebsiteSignup => write!(f, "website_signup"),
            Self::SurveyResponse => write!(f, "survey_response"),
            Self::SmsBulkSent => write!(f, "sms_bulk_sent"),
            Self::InboxThread => write!(f, "inbox_thread"),
            Self::QboPaymentLink => write!(f, "qbo_payment_link"),
            Self::VideoCallTranscripts => write!(f, "video_call_transcripts"),
            Self::WhatsappBulkSent => write!(f, "whatsapp_bulk_sent"),
            Self::WhatsappDelivered => write!(f, "whatsapp_delivered"),
            Self::WhatsappLinkClick => write!(f, "whatsapp_link_click"),
            Self::WhatsappSubscribe => write!(f, "whatsapp_subscribe"),
            Self::WhatsappSubscriptionPhoneUpdated => write!(f, "whatsapp_subscription_phone_updated"),
            Self::WhatsappUnsubscribe => write!(f, "whatsapp_unsubscribe"),
            Self::WhatsappUsedKeyword => write!(f, "whatsapp_used_keyword"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
