use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
  pub id: Option<String>,
  pub url: String,
  #[serde(rename = "imageType")]
  pub image_type: String,
  #[serde(rename = "fileName")]
  pub file_name: String,
}
