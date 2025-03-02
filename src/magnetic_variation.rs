use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMagneticVariation {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMagneticVariationAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDateMagneticVariation {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMagneticVariationChange {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}
