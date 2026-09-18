pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The social network to segment against.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemSocialNetworkMemberValue {
    Twitter,
    Facebook,
    Linkedin,
    Flickr,
    Foursquare,
    Lastfm,
    Myspace,
    Quora,
    Vimeo,
    Yelp,
    Youtube,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemSocialNetworkMemberValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Twitter => serializer.serialize_str("twitter"),
            Self::Facebook => serializer.serialize_str("facebook"),
            Self::Linkedin => serializer.serialize_str("linkedin"),
            Self::Flickr => serializer.serialize_str("flickr"),
            Self::Foursquare => serializer.serialize_str("foursquare"),
            Self::Lastfm => serializer.serialize_str("lastfm"),
            Self::Myspace => serializer.serialize_str("myspace"),
            Self::Quora => serializer.serialize_str("quora"),
            Self::Vimeo => serializer.serialize_str("vimeo"),
            Self::Yelp => serializer.serialize_str("yelp"),
            Self::Youtube => serializer.serialize_str("youtube"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemSocialNetworkMemberValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "twitter" => Ok(Self::Twitter),
            "facebook" => Ok(Self::Facebook),
            "linkedin" => Ok(Self::Linkedin),
            "flickr" => Ok(Self::Flickr),
            "foursquare" => Ok(Self::Foursquare),
            "lastfm" => Ok(Self::Lastfm),
            "myspace" => Ok(Self::Myspace),
            "quora" => Ok(Self::Quora),
            "vimeo" => Ok(Self::Vimeo),
            "yelp" => Ok(Self::Yelp),
            "youtube" => Ok(Self::Youtube),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemSocialNetworkMemberValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Twitter => write!(f, "twitter"),
            Self::Facebook => write!(f, "facebook"),
            Self::Linkedin => write!(f, "linkedin"),
            Self::Flickr => write!(f, "flickr"),
            Self::Foursquare => write!(f, "foursquare"),
            Self::Lastfm => write!(f, "lastfm"),
            Self::Myspace => write!(f, "myspace"),
            Self::Quora => write!(f, "quora"),
            Self::Vimeo => write!(f, "vimeo"),
            Self::Yelp => write!(f, "yelp"),
            Self::Youtube => write!(f, "youtube"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
