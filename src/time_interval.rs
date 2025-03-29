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
    pub aixm_day_til: Option<String>,
    #[serde(rename = "startTime")]
    pub aixm_start_time: Option<String>,
    #[serde(rename = "startEvent")]
    pub aixm_start_event: Option<String>,
    #[serde(rename = "startTimeRelativeEvent")]
    pub aixm_start_time_relative_event: Option<AixmStartTimeRelativeEvent>,
    #[serde(rename = "startEventInterpretation")]
    pub aixm_start_event_interpretation: Option<String>,
    #[serde(rename = "endTime")]
    pub aixm_end_time: Option<String>,
    #[serde(rename = "endEvent")]
    pub aixm_end_event: Option<String>,
    #[serde(rename = "endTimeRelativeEvent")]
    pub aixm_end_time_relative_event: Option<AixmEndTimeRelativeEvent>,
    #[serde(rename = "endEventInterpretation")]
    pub aixm_end_event_interpretation: Option<String>,
    #[serde(rename = "daylightSavingAdjust")]
    pub aixm_daylight_saving_adjust: Option<String>,
    #[serde(rename = "excluded")]
    pub aixm_excluded: Option<String>,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStartTimeRelativeEvent {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEndTimeRelativeEvent {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: i32,
}
