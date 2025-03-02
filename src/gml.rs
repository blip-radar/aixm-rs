use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmlTimePeriod {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "beginPosition")]
    pub gml_begin_position: String,
    #[serde(rename = "endPosition")]
    pub gml_end_position: GmlTimePeriodGmlEndPosition,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmlTimeInstant {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "timePosition")]
    pub gml_time_position: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmlTimePeriodGmlEndPosition {
    #[serde(rename = "@indeterminatePosition")]
    pub indeterminate_position: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmlIdentifier {
    #[serde(rename = "@codeSpace")]
    pub code_space: String,
    #[serde(rename = "$text")]
    pub text: String,
}

// FIXME: enum?
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmlValidTime {
    #[serde(rename = "TimePeriod")]
    pub gml_time_period: Option<GmlTimePeriod>,
    #[serde(rename = "TimeInstant")]
    pub gml_time_instant: Option<GmlTimeInstant>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmlSegments {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "LineStringSegment")]
    pub gml_line_string_segment: GmlLineStringSegment,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmlLineStringSegment {
    #[serde(rename = "posList")]
    pub gml_pos_list: String,
}
