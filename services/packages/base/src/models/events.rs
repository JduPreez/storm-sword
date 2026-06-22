use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address {
  #[serde(rename = "countryCode")]
  pub country_code: String,
  #[serde(rename = "addressLine1")]
  pub address_line1: Option<String>,
  #[serde(rename = "addressLine2")]
  pub address_line2: Option<String>,
  #[serde(rename = "cityTownLocality")]
  pub city_town_locality: Option<String>,
  #[serde(rename = "sublocality")]
  pub sublocality: Option<String>, // Suburb, Borough, Barrio
  #[serde(rename = "administrativeArea")]
  pub administrative_area: String, // State, Province, Prefecture, County
  #[serde(rename = "postalCode")]
  pub postal_code: Option<String>, // ZIP code, Postcode, PIN code
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
  pub id: Option<String>,
  pub ns: Option<String>,
  #[serde(rename = "eventType")]
  pub event_type: Option<String>,
  pub name: String,
  #[serde(rename = "startDate")]
  pub start_date: Option<i64>,
  #[serde(rename = "endDate")]
  pub end_date: Option<i64>,
  #[serde(rename = "distanceMin")]
  pub distance_min: Option<f64>,
  #[serde(rename = "distanceMax")]
  pub distance_max: Option<f64>,
  pub address: Address,
  pub description: Option<String>,
  #[serde(skip_serializing)]
  pub address_administrative_area_idx: Option<String>,
  pub metadata: Option<serde_json::Value>,
}