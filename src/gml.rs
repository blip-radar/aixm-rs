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
    #[serde(rename = "$value", default)]
    pub segments: Vec<GmlSegment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GmlSegment {
    GeodesicString(GmlGeodesicString),
    CircleByCenterPoint(GmlCircleByCenterPoint),
    ArcByCenterPoint(GmlArcByCenterPoint),
    LineStringSegment(GmlLineStringSegment),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GmlLineStringSegment {
    #[serde(rename = "posList")]
    pub gml_pos_list: Option<String>,
    #[serde(rename = "pos", default)]
    pub gml_pos: Vec<GmlPos>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GmlPos {
    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GmlRadius {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GmlGeodesicString {
    #[serde(rename = "pos", default)]
    pub positions: Vec<GmlPos>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GmlCircleByCenterPoint {
    #[serde(rename = "@numArc")]
    pub num_arc: Option<u32>,
    #[serde(rename = "pos")]
    pub center: GmlPos,
    #[serde(rename = "radius")]
    pub radius: GmlRadius,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GmlAngle {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GmlArcByCenterPoint {
    #[serde(rename = "@numArc")]
    pub num_arc: Option<u32>,
    #[serde(rename = "pos")]
    pub center: GmlPos,
    #[serde(rename = "radius")]
    pub radius: GmlRadius,
    #[serde(rename = "startAngle")]
    pub start_angle: GmlAngle,
    #[serde(rename = "endAngle")]
    pub end_angle: GmlAngle,
}
