use serde::{Deserialize, Serialize};

use crate::annotation::AixmAnnotation;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTimeInterval {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "Timesheet")]
    pub aixm_timesheet: Option<AixmTimesheet>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTimesheet {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "timeReference")]
    pub aixm_time_reference: String,
    #[serde(rename = "startDate")]
    pub aixm_start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub aixm_end_date: Option<String>,
    #[serde(rename = "day")]
    pub aixm_day: String,
    #[serde(rename = "dayTil")]
    pub aixm_day_til: Option<AixmDayTil>,
    #[serde(rename = "startTime")]
    pub aixm_start_time: AixmStartTime,
    #[serde(rename = "startEvent")]
    pub aixm_start_event: Option<AixmStartEvent>,
    #[serde(rename = "startTimeRelativeEvent")]
    pub aixm_start_time_relative_event: Option<AixmStartTimeRelativeEvent>,
    #[serde(rename = "startEventInterpretation")]
    pub aixm_start_event_interpretation: Option<AixmStartEventInterpretation>,
    #[serde(rename = "endTime")]
    pub aixm_end_time: AixmEndTime,
    #[serde(rename = "endEvent")]
    pub aixm_end_event: Option<AixmEndEvent>,
    #[serde(rename = "endTimeRelativeEvent")]
    pub aixm_end_time_relative_event: Option<AixmEndTimeRelativeEvent>,
    #[serde(rename = "endEventInterpretation")]
    pub aixm_end_event_interpretation: Option<AixmEndEventInterpretation>,
    #[serde(rename = "daylightSavingAdjust")]
    pub aixm_daylight_saving_adjust: Option<AixmDaylightSavingAdjust>,
    #[serde(rename = "excluded")]
    pub aixm_excluded: Option<AixmExcluded>,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDayTil {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStartTime {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStartEvent {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStartTimeRelativeEvent {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStartEventInterpretation {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEndTime {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEndEvent {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEndTimeRelativeEvent {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEndEventInterpretation {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDaylightSavingAdjust {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmExcluded {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}
