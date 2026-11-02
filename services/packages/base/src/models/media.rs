use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,
  #[serde(rename = "imageType", skip_serializing_if = "Option::is_none")]
  pub image_type: Option<String>,
  #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
  pub file_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub alt: Option<String>,
}

impl Image {
  /// An ID-only reference, until images are hydrated from their own table.
  pub fn from_id(id: String) -> Self {
    Image {
      id: Some(id),
      url: None,
      image_type: None,
      file_name: None,
      alt: None,
    }
  }
}
