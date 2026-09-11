pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of Automation workflow.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationWorkflowTriggerSettingsWorkflowType {
    AbandonedBrowse,
    AbandonedCart,
    Api,
    BestCustomers,
    CategoryFollowup,
    DateAdded,
    EmailFollowup,
    EmailSeries,
    GroupAdd,
    GroupRemove,
    Mandrill,
    ProductFollowup,
    PurchaseFollowup,
    RecurringEvent,
    SpecialEvent,
    VisitUrl,
    WelcomeSeries,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationWorkflowTriggerSettingsWorkflowType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AbandonedBrowse => serializer.serialize_str("abandonedBrowse"),
            Self::AbandonedCart => serializer.serialize_str("abandonedCart"),
            Self::Api => serializer.serialize_str("api"),
            Self::BestCustomers => serializer.serialize_str("bestCustomers"),
            Self::CategoryFollowup => serializer.serialize_str("categoryFollowup"),
            Self::DateAdded => serializer.serialize_str("dateAdded"),
            Self::EmailFollowup => serializer.serialize_str("emailFollowup"),
            Self::EmailSeries => serializer.serialize_str("emailSeries"),
            Self::GroupAdd => serializer.serialize_str("groupAdd"),
            Self::GroupRemove => serializer.serialize_str("groupRemove"),
            Self::Mandrill => serializer.serialize_str("mandrill"),
            Self::ProductFollowup => serializer.serialize_str("productFollowup"),
            Self::PurchaseFollowup => serializer.serialize_str("purchaseFollowup"),
            Self::RecurringEvent => serializer.serialize_str("recurringEvent"),
            Self::SpecialEvent => serializer.serialize_str("specialEvent"),
            Self::VisitUrl => serializer.serialize_str("visitUrl"),
            Self::WelcomeSeries => serializer.serialize_str("welcomeSeries"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationWorkflowTriggerSettingsWorkflowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "abandonedBrowse" => Ok(Self::AbandonedBrowse),
            "abandonedCart" => Ok(Self::AbandonedCart),
            "api" => Ok(Self::Api),
            "bestCustomers" => Ok(Self::BestCustomers),
            "categoryFollowup" => Ok(Self::CategoryFollowup),
            "dateAdded" => Ok(Self::DateAdded),
            "emailFollowup" => Ok(Self::EmailFollowup),
            "emailSeries" => Ok(Self::EmailSeries),
            "groupAdd" => Ok(Self::GroupAdd),
            "groupRemove" => Ok(Self::GroupRemove),
            "mandrill" => Ok(Self::Mandrill),
            "productFollowup" => Ok(Self::ProductFollowup),
            "purchaseFollowup" => Ok(Self::PurchaseFollowup),
            "recurringEvent" => Ok(Self::RecurringEvent),
            "specialEvent" => Ok(Self::SpecialEvent),
            "visitUrl" => Ok(Self::VisitUrl),
            "welcomeSeries" => Ok(Self::WelcomeSeries),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationWorkflowTriggerSettingsWorkflowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AbandonedBrowse => write!(f, "abandonedBrowse"),
            Self::AbandonedCart => write!(f, "abandonedCart"),
            Self::Api => write!(f, "api"),
            Self::BestCustomers => write!(f, "bestCustomers"),
            Self::CategoryFollowup => write!(f, "categoryFollowup"),
            Self::DateAdded => write!(f, "dateAdded"),
            Self::EmailFollowup => write!(f, "emailFollowup"),
            Self::EmailSeries => write!(f, "emailSeries"),
            Self::GroupAdd => write!(f, "groupAdd"),
            Self::GroupRemove => write!(f, "groupRemove"),
            Self::Mandrill => write!(f, "mandrill"),
            Self::ProductFollowup => write!(f, "productFollowup"),
            Self::PurchaseFollowup => write!(f, "purchaseFollowup"),
            Self::RecurringEvent => write!(f, "recurringEvent"),
            Self::SpecialEvent => write!(f, "specialEvent"),
            Self::VisitUrl => write!(f, "visitUrl"),
            Self::WelcomeSeries => write!(f, "welcomeSeries"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
