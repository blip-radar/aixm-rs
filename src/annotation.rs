use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAnnotation {
    #[serde(rename = "Note")]
    pub aixm_note: Option<AixmNote>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNote {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "propertyName")]
    pub aixm_property_name: Option<String>,
    #[serde(rename = "purpose")]
    pub aixm_purpose: String,
    #[serde(rename = "translatedNote")]
    pub aixm_translated_note: AixmTranslatedNote,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTranslatedNote {
    #[serde(rename = "LinguisticNote")]
    pub aixm_linguistic_note: AixmLinguisticNote,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLinguisticNote {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "note")]
    pub aixm_note: String,
}
