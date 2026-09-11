pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A string that identifies the element selector.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateSignupFormListsRequestStylesItemSelector {
    PageBackground,
    PageHeader,
    PageOuterWrapper,
    BodyBackground,
    BodyLinkStyle,
    FormsButtons,
    FormsButtonsHovered,
    FormsFieldLabel,
    FormsFieldText,
    FormsRequired,
    FormsRequiredLegend,
    FormsHelpText,
    FormsErrors,
    MonkeyRewardsBadge,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateSignupFormListsRequestStylesItemSelector {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PageBackground => serializer.serialize_str("page_background"),
            Self::PageHeader => serializer.serialize_str("page_header"),
            Self::PageOuterWrapper => serializer.serialize_str("page_outer_wrapper"),
            Self::BodyBackground => serializer.serialize_str("body_background"),
            Self::BodyLinkStyle => serializer.serialize_str("body_link_style"),
            Self::FormsButtons => serializer.serialize_str("forms_buttons"),
            Self::FormsButtonsHovered => serializer.serialize_str("forms_buttons_hovered"),
            Self::FormsFieldLabel => serializer.serialize_str("forms_field_label"),
            Self::FormsFieldText => serializer.serialize_str("forms_field_text"),
            Self::FormsRequired => serializer.serialize_str("forms_required"),
            Self::FormsRequiredLegend => serializer.serialize_str("forms_required_legend"),
            Self::FormsHelpText => serializer.serialize_str("forms_help_text"),
            Self::FormsErrors => serializer.serialize_str("forms_errors"),
            Self::MonkeyRewardsBadge => serializer.serialize_str("monkey_rewards_badge"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateSignupFormListsRequestStylesItemSelector {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "page_background" => Ok(Self::PageBackground),
            "page_header" => Ok(Self::PageHeader),
            "page_outer_wrapper" => Ok(Self::PageOuterWrapper),
            "body_background" => Ok(Self::BodyBackground),
            "body_link_style" => Ok(Self::BodyLinkStyle),
            "forms_buttons" => Ok(Self::FormsButtons),
            "forms_buttons_hovered" => Ok(Self::FormsButtonsHovered),
            "forms_field_label" => Ok(Self::FormsFieldLabel),
            "forms_field_text" => Ok(Self::FormsFieldText),
            "forms_required" => Ok(Self::FormsRequired),
            "forms_required_legend" => Ok(Self::FormsRequiredLegend),
            "forms_help_text" => Ok(Self::FormsHelpText),
            "forms_errors" => Ok(Self::FormsErrors),
            "monkey_rewards_badge" => Ok(Self::MonkeyRewardsBadge),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateSignupFormListsRequestStylesItemSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PageBackground => write!(f, "page_background"),
            Self::PageHeader => write!(f, "page_header"),
            Self::PageOuterWrapper => write!(f, "page_outer_wrapper"),
            Self::BodyBackground => write!(f, "body_background"),
            Self::BodyLinkStyle => write!(f, "body_link_style"),
            Self::FormsButtons => write!(f, "forms_buttons"),
            Self::FormsButtonsHovered => write!(f, "forms_buttons_hovered"),
            Self::FormsFieldLabel => write!(f, "forms_field_label"),
            Self::FormsFieldText => write!(f, "forms_field_text"),
            Self::FormsRequired => write!(f, "forms_required"),
            Self::FormsRequiredLegend => write!(f, "forms_required_legend"),
            Self::FormsHelpText => write!(f, "forms_help_text"),
            Self::FormsErrors => write!(f, "forms_errors"),
            Self::MonkeyRewardsBadge => write!(f, "monkey_rewards_badge"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
