#![allow(clippy::struct_field_names, reason = "XML namespaces")]
use annotation::AixmAnnotation;
use gmd::GmdMdMetadata;
use gml::{GmlIdentifier, GmlSegments, GmlTimePeriod, GmlValidTime};
use magnetic_variation::{
    AixmDateMagneticVariation, AixmMagneticVariation, AixmMagneticVariationAccuracy,
    AixmMagneticVariationChange,
};
use serde::{Deserialize, Serialize};
use time_interval::AixmTimeInterval;

mod annotation;
mod gmd;
mod gml;
mod magnetic_variation;
mod time_interval;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageAixmBasicMessage {
    #[serde(rename = "@xmlns:gss")]
    pub xmlns_gss: String,
    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "@xmlns:message")]
    pub xmlns_message: String,
    #[serde(rename = "@xmlns:gsr")]
    pub xmlns_gsr: String,
    #[serde(rename = "@xmlns:gco")]
    pub xmlns_gco: String,
    #[serde(rename = "@xmlns:gml")]
    pub xmlns_gml: String,
    #[serde(rename = "@xmlns:gmd")]
    pub xmlns_gmd: String,
    #[serde(rename = "@xmlns:aixm")]
    pub xmlns_aixm: String,
    #[serde(rename = "@xmlns:xlink")]
    pub xmlns_xlink: String,
    #[serde(rename = "@xmlns:gts")]
    pub xmlns_gts: String,
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "messageMetadata")]
    pub aixm_message_metadata: AixmMessageMetadata,
    #[serde(rename = "hasMember")]
    pub message_has_member: Vec<MessageHasMember>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMessageMetadata {
    #[serde(rename = "@xmlns:gmx")]
    pub xmlns_gmx: String,
    #[serde(rename = "MD_Metadata")]
    pub gmd_md_metadata: GmdMdMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmFeatureLifetime {
    #[serde(rename = "TimePeriod")]
    pub gml_time_period: GmlTimePeriod,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEmissionClass {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMobile {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmReferenceTemperature {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTransitionAltitude {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTransitionLevel {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLowestTemperature {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmFlightChecked {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmArp {
    #[serde(rename = "ElevatedPoint")]
    pub aixm_elevated_point: AixmElevatedPoint,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLocation {
    #[serde(rename = "$value")]
    location: LocationType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum LocationType {
    ElevatedPoint(Box<AixmElevatedPoint>),
    Point(Box<AixmPoint>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPoint {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "pos")]
    pub gml_pos: String,
    #[serde(rename = "horizontalAccuracy")]
    pub aixm_horizontal_accuracy: AixmHorizontalAccuracy,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmElevatedPoint {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "pos")]
    pub gml_pos: String,
    #[serde(rename = "horizontalAccuracy")]
    pub aixm_horizontal_accuracy: AixmHorizontalAccuracy,
    #[serde(rename = "elevation")]
    pub aixm_elevation: AixmElevation,
    #[serde(rename = "geoidUndulation")]
    pub aixm_geoid_undulation: AixmGeoidUndulation,
    #[serde(rename = "verticalDatum")]
    pub aixm_vertical_datum: AixmVerticalDatum,
    #[serde(rename = "verticalAccuracy")]
    pub aixm_vertical_accuracy: AixmVerticalAccuracy,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmHorizontalAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmElevation {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmGeoidUndulation {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVerticalDatum {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVerticalAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmCertificationDate {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmCertificationExpirationDate {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmResponsibleOrganisation {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "AirportHeliportResponsibilityOrganisation")]
    pub aixm_airport_heliport_responsibility_organisation:
        Option<AixmAirportHeliportResponsibilityOrganisation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportResponsibilityOrganisation {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "role")]
    pub aixm_role: AixmRole,
    #[serde(rename = "theOrganisationAuthority")]
    pub aixm_the_organisation_authority: AixmTheOrganisationAuthority,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRole {
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmServedCity {
    #[serde(rename = "City")]
    pub aixm_city: AixmServedCityAixmCity,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmServedCityAixmCity {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "name")]
    pub aixm_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmContact {
    #[serde(rename = "ContactInformation")]
    pub aixm_contact_information: AixmContactInformation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmContactInformation {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "name")]
    pub aixm_name: AixmName,
    #[serde(rename = "address")]
    pub aixm_address: Option<Vec<AixmAddress>>,
    #[serde(rename = "phoneFax")]
    pub aixm_phone_fax: Option<AixmPhoneFax>,
    #[serde(rename = "networkNode")]
    pub aixm_network_node: Option<Vec<AixmNetworkNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAddress {
    #[serde(rename = "PostalAddress")]
    pub aixm_postal_address: AixmPostalAddress,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPostalAddress {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "deliveryPoint")]
    pub aixm_delivery_point: String,
    #[serde(rename = "city")]
    pub aixm_city: AixmCity,
    #[serde(rename = "postalCode")]
    pub aixm_postal_code: AixmPostalCode,
    #[serde(rename = "country")]
    pub aixm_country: AixmCountry,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmCity {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPostalCode {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmCountry {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPhoneFax {
    #[serde(rename = "TelephoneContact")]
    pub aixm_telephone_contact: AixmTelephoneContact,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTelephoneContact {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Vec<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNetworkNode {
    #[serde(rename = "OnlineContact")]
    pub aixm_online_contact: AixmOnlineContact,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmOnlineContact {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "network")]
    pub aixm_network: String,
    #[serde(rename = "linkage")]
    pub aixm_linkage: AixmLinkage,
    #[serde(rename = "eMail")]
    pub aixm_e_mail: AixmEMail,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLinkage {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEMail {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAuthority {
    #[serde(rename = "AuthorityForNavaidEquipment")]
    pub aixm_authority_for_navaid_equipment: AixmAuthorityForNavaidEquipment,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAuthorityForNavaidEquipment {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "theOrganisationAuthority")]
    pub aixm_the_organisation_authority: AixmTheOrganisationAuthority,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTheOrganisationAuthority {
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNavaidAvailability {
    #[serde(rename = "NavaidOperationalStatus")]
    pub aixm_navaid_operational_status: AixmNavaidOperationalStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportAixmAvailability {
    #[serde(rename = "AirportHeliportAvailability")]
    pub aixm_airport_heliport_availability: AixmAirportHeliportAvailability,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportAvailability {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "timeInterval")]
    pub aixm_time_interval: Vec<AixmTimeInterval>,
    #[serde(rename = "operationalStatus")]
    pub aixm_operational_status: AixmOperationalStatus,
    #[serde(rename = "warning")]
    pub aixm_warning: AixmWarning,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<Vec<AixmAnnotation>>,
    #[serde(rename = "usage")]
    pub aixm_usage: Option<Vec<AixmUsage>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmOperationalStatus {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWarning {
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNavaidOperationalStatus {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "timeInterval")]
    pub aixm_time_interval: AixmTimeInterval,
    #[serde(rename = "operationalStatus")]
    pub aixm_operational_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmUsage {
    #[serde(rename = "AirportHeliportUsage")]
    pub aixm_airport_heliport_usage: AixmAirportHeliportUsage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportUsage {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "selection")]
    pub aixm_selection: Option<AixmSelection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmSelection {
    #[serde(rename = "ConditionCombination")]
    pub aixm_condition_combination: AixmConditionCombination,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmConditionCombination {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "logicalOperator")]
    pub aixm_logical_operator: String,
    #[serde(rename = "flight")]
    pub aixm_flight: Vec<AixmFlight>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmFlight {
    #[serde(rename = "FlightCharacteristic")]
    pub aixm_flight_characteristic: AixmFlightCharacteristic,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmFlightCharacteristic {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "rule")]
    pub aixm_rule: String,
    #[serde(rename = "status")]
    pub aixm_status: String,
    #[serde(rename = "military")]
    pub aixm_military: String,
    #[serde(rename = "origin")]
    pub aixm_origin: AixmOrigin,
    #[serde(rename = "purpose")]
    pub aixm_purpose: AixmPurpose,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmOrigin {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmType {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmGhostFrequency {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDisplace {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmName {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmFrequency {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAngleAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRdh {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRdhAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMagneticBearingAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTrueBearingAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDeclination {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidthCourse {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidthCourseAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmBackCourseUsable {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmClass {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAxisBearing {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEmissionBand {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPurpose {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmSignalPerformance {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmCourseQuality {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmIntegrityLevel {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmCollocationGroup {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmProvidesNavigableLocation {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTheNavaidEquipment {
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNavaidEquipment {
    #[serde(rename = "NavaidComponent")]
    pub aixm_navaid_component: AixmNavaidComponent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNavaidComponent {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "collocationGroup")]
    pub aixm_collocation_group: Option<AixmCollocationGroup>,
    #[serde(rename = "providesNavigableLocation")]
    pub aixm_provides_navigable_location: AixmProvidesNavigableLocation,
    #[serde(rename = "theNavaidEquipment")]
    pub aixm_the_navaid_equipment: AixmTheNavaidEquipment,
    #[serde(rename = "markerPosition")]
    pub aixm_marker_position: Option<AixmMarkerPosition>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMarkerPosition {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayDirectionLink {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmServedAirport {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAngleScallop {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEquipmentNavaidEquipment {
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmSector {
    #[serde(rename = "CircleSector")]
    pub aixm_circle_sector: AixmCircleSector,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmCircleSector {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "arcDirection")]
    pub aixm_arc_direction: AixmArcDirection,
    #[serde(rename = "fromAngle")]
    pub aixm_from_angle: String,
    #[serde(rename = "toAngle")]
    pub aixm_to_angle: String,
    #[serde(rename = "angleType")]
    pub aixm_angle_type: AixmAngleType,
    #[serde(rename = "angleDirectionReference")]
    pub aixm_angle_direction_reference: AixmAngleDirectionReference,
    #[serde(rename = "innerDistance")]
    pub aixm_inner_distance: AixmInnerDistance,
    #[serde(rename = "outerDistance")]
    pub aixm_outer_distance: AixmOuterDistance,
    #[serde(rename = "upperLimit")]
    pub aixm_upper_limit: AixmUpperLimit,
    #[serde(rename = "upperLimitReference")]
    pub aixm_upper_limit_reference: AixmUpperLimitReference,
    #[serde(rename = "lowerLimit")]
    pub aixm_lower_limit: AixmLowerLimit,
    #[serde(rename = "lowerLimitReference")]
    pub aixm_lower_limit_reference: AixmLowerLimitReference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmArcDirection {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAngleType {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAngleDirectionReference {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmInnerDistance {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmOuterDistance {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmUpperLimit {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmUpperLimitReference {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLowerLimit {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLowerLimitReference {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AixmIndicationDirection {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AixmTrueAngle {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AixmCardinalDirection {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AixmMinimumReceptionAltitude {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AixmFix {
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AixmPointChoiceNavaidSystem {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AixmAirportHeliportLink {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AixmDistance {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLocationIndicatorIcao {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDesignatorIata {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmControlType {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmFieldElevation {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmHostAirport {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDependentAirport {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDesignatorPrefix {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMultipleIdentifier {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmFlightRule {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmInternationalUse {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMilitaryUse {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMilitaryTrainingType {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMinimumObstacleClearanceAltitude {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPathType {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTrueTrack {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMagneticTrack {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmReverseTrueTrack {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmReverseMagneticTrack {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLength {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidth {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidthLeft {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidthRight {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTurnDirection {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmSignalGap {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMinimumEnrouteAltitude {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMinimumCrossingAtEnd {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMinimumCrossingAtEndReference {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMaximumCrossingAtEnd {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMaximumCrossingAtEndReference {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRequiredNavigationPerformance {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDesignatorSuffix {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStart {
    #[serde(rename = "EnRouteSegmentPoint")]
    pub aixm_en_route_segment_point: AixmStartAixmEnRouteSegmentPoint,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStartAixmEnRouteSegmentPoint {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "pointChoice_fixDesignatedPoint")]
    pub aixm_point_choice_fix_designated_point:
        Option<AixmStartAixmEnRouteSegmentPointAixmPointChoiceFixDesignatedPoint>,
    #[serde(rename = "pointChoice_navaidSystem")]
    pub aixm_point_choice_navaid_system:
        Option<AixmStartAixmEnRouteSegmentPointAixmPointChoiceNavaidSystem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStartAixmEnRouteSegmentPointAixmPointChoiceFixDesignatedPoint {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStartAixmEnRouteSegmentPointAixmPointChoiceNavaidSystem {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRouteFormed {
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmCurveExtent {
    #[serde(rename = "Curve")]
    pub aixm_curve: AixmCurve,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmCurve {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "segments")]
    pub gml_segments: GmlSegments,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEnd {
    #[serde(rename = "EnRouteSegmentPoint")]
    pub aixm_en_route_segment_point: AixmEndAixmEnRouteSegmentPoint,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEndAixmEnRouteSegmentPoint {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "pointChoice_fixDesignatedPoint")]
    pub aixm_point_choice_fix_designated_point:
        Option<AixmEndAixmEnRouteSegmentPointAixmPointChoiceFixDesignatedPoint>,
    #[serde(rename = "pointChoice_navaidSystem")]
    pub aixm_point_choice_navaid_system:
        Option<AixmEndAixmEnRouteSegmentPointAixmPointChoiceNavaidSystem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEndAixmEnRouteSegmentPointAixmPointChoiceFixDesignatedPoint {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEndAixmEnRouteSegmentPointAixmPointChoiceNavaidSystem {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRouteSegmentAixmAvailability {
    #[serde(rename = "RouteAvailability")]
    pub aixm_route_availability: AixmRouteAvailability,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRouteAvailability {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "timeInterval")]
    pub aixm_time_interval: Option<AixmTimeInterval>,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Vec<AixmAnnotation>,
    #[serde(rename = "direction")]
    pub aixm_direction: String,
    #[serde(rename = "cardinalDirection")]
    pub aixm_cardinal_direction: AixmCardinalDirection,
    #[serde(rename = "status")]
    pub aixm_status: String,
    #[serde(rename = "levels")]
    pub aixm_levels: Vec<AixmLevels>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLevels {
    #[serde(rename = "AirspaceLayer")]
    pub aixm_airspace_layer: AixmAirspaceLayer,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirspaceLayer {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "upperLimit")]
    pub aixm_upper_limit: AixmUpperLimit,
    #[serde(rename = "upperLimitReference")]
    pub aixm_upper_limit_reference: AixmUpperLimitReference,
    #[serde(rename = "lowerLimit")]
    pub aixm_lower_limit: AixmLowerLimit,
    #[serde(rename = "lowerLimitReference")]
    pub aixm_lower_limit_reference: String,
    #[serde(rename = "altitudeInterpretation")]
    pub aixm_altitude_interpretation: AixmAltitudeInterpretation,
    #[serde(rename = "discreteLevelSeries")]
    pub aixm_discrete_level_series: Option<AixmDiscreteLevelSeries>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAltitudeInterpretation {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDiscreteLevelSeries {
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmUnitOfMeasurement {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLevel {
    #[serde(rename = "StandardLevel")]
    pub aixm_standard_level: AixmStandardLevel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStandardLevel {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "verticalDistance")]
    pub aixm_vertical_distance: AixmVerticalDistance,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVerticalDistance {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLevelTable {
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStandardIcao {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNominalLength {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLengthAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNominalWidth {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidthAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidthShoulder {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLengthStrip {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidthStrip {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLengthOffset {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidthOffset {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAbandoned {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmSurfaceProperties {
    #[serde(rename = "SurfaceCharacteristics")]
    pub aixm_surface_characteristics: AixmSurfaceCharacteristics,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmSurfaceCharacteristics {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "composition")]
    pub aixm_composition: String,
    #[serde(rename = "preparation")]
    pub aixm_preparation: AixmPreparation,
    #[serde(rename = "surfaceCondition")]
    pub aixm_surface_condition: AixmSurfaceCondition,
    #[serde(rename = "classPCN")]
    pub aixm_class_pcn: AixmClassPcn,
    #[serde(rename = "pavementTypePCN")]
    pub aixm_pavement_type_pcn: AixmPavementTypePcn,
    #[serde(rename = "pavementSubgradePCN")]
    pub aixm_pavement_subgrade_pcn: AixmPavementSubgradePcn,
    #[serde(rename = "maxTyrePressurePCN")]
    pub aixm_max_tyre_pressure_pcn: AixmMaxTyrePressurePcn,
    #[serde(rename = "evaluationMethodPCN")]
    pub aixm_evaluation_method_pcn: AixmEvaluationMethodPcn,
    #[serde(rename = "classLCN")]
    pub aixm_class_lcn: AixmClassLcn,
    #[serde(rename = "weightSIWL")]
    pub aixm_weight_siwl: AixmWeightSiwl,
    #[serde(rename = "tyrePressureSIWL")]
    pub aixm_tyre_pressure_siwl: AixmTyrePressureSiwl,
    #[serde(rename = "weightAUW")]
    pub aixm_weight_auw: AixmWeightAuw,
    #[serde(rename = "annotation")]
    pub aixm_annotation: AixmAnnotation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPreparation {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmSurfaceCondition {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmClassPcn {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPavementTypePcn {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPavementSubgradePcn {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMaxTyrePressurePcn {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEvaluationMethodPcn {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmClassLcn {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWeightSiwl {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTyrePressureSiwl {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWeightAuw {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmOnRunway {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDesignator {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAssociatedDeclaredDistance {
    #[serde(rename = "RunwayDeclaredDistance")]
    pub aixm_runway_declared_distance: AixmRunwayDeclaredDistance,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayDeclaredDistance {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "declaredValue")]
    pub aixm_declared_value: AixmDeclaredValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDeclaredValue {
    #[serde(rename = "RunwayDeclaredDistanceValue")]
    pub aixm_runway_declared_distance_value: AixmRunwayDeclaredDistanceValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayDeclaredDistanceValue {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "distance")]
    pub aixm_distance: AixmDistance,
    #[serde(rename = "distanceAccuracy")]
    pub aixm_distance_accuracy: AixmDistanceAccuracy,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDistanceAccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTrueBearing {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMagneticBearing {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPatternVfr {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmSlopeTdz {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmElevationTdz {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmElevationTdzaccuracy {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmApproachMarkingType {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmApproachMarkingCondition {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmClassLightingJar {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPrecisionApproachGuidance {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmUsedRunway {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayProtectArea {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmRunwayProtectAreaAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayProtectAreaAixmTimeSlice {
    #[serde(rename = "RunwayProtectAreaTimeSlice")]
    pub aixm_runway_protect_area_time_slice: AixmRunwayProtectAreaTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayProtectAreaTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: String,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: String,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "width")]
    pub aixm_width: AixmWidth,
    #[serde(rename = "length")]
    pub aixm_length: AixmLength,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "status")]
    pub aixm_status: AixmStatus,
    #[serde(rename = "protectedRunwayDirection")]
    pub aixm_protected_runway_direction: AixmProtectedRunwayDirection,
    #[serde(rename = "surfaceProperties")]
    pub aixm_surface_properties: Option<AixmSurfaceProperties>,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStatus {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmProtectedRunwayDirection {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmEmergencyLighting {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmIntensityLevel {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmColour {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPosition {
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNumberBox {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPortable {
    #[serde(rename = "@nil")]
    pub xsi_nil: String,
    #[serde(rename = "@nilReason")]
    pub nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmSlopeAngle {
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMinimumEyeHeightOverThreshold {
    #[serde(rename = "@uom")]
    pub uom: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageHasMember {
    #[serde(rename = "$value")]
    pub member: Member,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Member {
    AirportHeliport(Box<AixmAirportHeliport>),
    AirportHeliportCollocation(Box<AixmAirportHeliportCollocation>),
    AngleIndication(Box<AixmAngleIndication>),
    DesignatedPoint(Box<AixmDesignatedPoint>),
    DistanceIndication(Box<AixmDistanceIndication>),
    #[serde(rename = "DME")]
    Dme(Box<AixmDme>),
    Glidepath(Box<AixmGlidepath>),
    Localizer(Box<AixmLocalizer>),
    MarkerBeacon(Box<AixmMarkerBeacon>),
    #[serde(rename = "NDB")]
    Ndb(Box<AixmNdb>),
    Navaid(Box<AixmNavaid>),
    RadioFrequencyArea(Box<AixmRadioFrequencyArea>),
    Route(Box<AixmRoute>),
    RouteSegment(Box<AixmRouteSegment>),
    Runway(Box<AixmRunway>),
    RunwayCentrelinePoint(Box<AixmRunwayCentrelinePoint>),
    RunwayDirection(Box<AixmRunwayDirection>),
    RunwayProtectArea(Box<AixmRunwayProtectArea>),
    StandardLevelColumn(Box<AixmStandardLevelColumn>),
    StandardLevelTable(Box<AixmStandardLevelTable>),
    #[serde(rename = "TACAN")]
    Tacan(Box<AixmTacan>),
    VisualGlideSlopeIndicator(Box<AixmVisualGlideSlopeIndicator>),
    #[serde(rename = "VOR")]
    Vor(Box<AixmVor>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDme {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmDmeAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDmeAixmTimeSlice {
    #[serde(rename = "DMETimeSlice")]
    pub aixm_dmetime_slice: AixmDmetimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDmetimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "name")]
    pub aixm_name: String,
    #[serde(rename = "emissionClass")]
    pub aixm_emission_class: AixmEmissionClass,
    #[serde(rename = "mobile")]
    pub aixm_mobile: AixmMobile,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: AixmMagneticVariation,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: AixmMagneticVariationAccuracy,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: AixmDateMagneticVariation,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: AixmFlightChecked,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "authority")]
    pub aixm_authority: Option<AixmAuthority>,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmNavaidAvailability,
    #[serde(rename = "type")]
    pub aixm_type: AixmType,
    #[serde(rename = "channel")]
    pub aixm_channel: String,
    #[serde(rename = "ghostFrequency")]
    pub aixm_ghost_frequency: AixmGhostFrequency,
    #[serde(rename = "displace")]
    pub aixm_displace: AixmDisplace,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmGlidepath {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmGlidepathAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmGlidepathAixmTimeSlice {
    #[serde(rename = "GlidepathTimeSlice")]
    pub aixm_glidepath_time_slice: AixmGlidepathTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmGlidepathTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "name")]
    pub aixm_name: AixmName,
    #[serde(rename = "emissionClass")]
    pub aixm_emission_class: AixmEmissionClass,
    #[serde(rename = "mobile")]
    pub aixm_mobile: AixmMobile,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: AixmMagneticVariation,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: AixmMagneticVariationAccuracy,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: AixmDateMagneticVariation,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: AixmFlightChecked,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmNavaidAvailability,
    #[serde(rename = "frequency")]
    pub aixm_frequency: AixmFrequency,
    #[serde(rename = "slope")]
    pub aixm_slope: String,
    #[serde(rename = "angleAccuracy")]
    pub aixm_angle_accuracy: AixmAngleAccuracy,
    #[serde(rename = "rdh")]
    pub aixm_rdh: AixmRdh,
    #[serde(rename = "rdhAccuracy")]
    pub aixm_rdh_accuracy: AixmRdhAccuracy,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLocalizer {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmLocalizerAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLocalizerAixmTimeSlice {
    #[serde(rename = "LocalizerTimeSlice")]
    pub aixm_localizer_time_slice: AixmLocalizerTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLocalizerTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "name")]
    pub aixm_name: AixmName,
    #[serde(rename = "emissionClass")]
    pub aixm_emission_class: AixmEmissionClass,
    #[serde(rename = "mobile")]
    pub aixm_mobile: AixmMobile,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: String,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: AixmMagneticVariationAccuracy,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: String,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: AixmFlightChecked,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmNavaidAvailability,
    #[serde(rename = "frequency")]
    pub aixm_frequency: AixmFrequency,
    #[serde(rename = "magneticBearing")]
    pub aixm_magnetic_bearing: String,
    #[serde(rename = "magneticBearingAccuracy")]
    pub aixm_magnetic_bearing_accuracy: AixmMagneticBearingAccuracy,
    #[serde(rename = "trueBearing")]
    pub aixm_true_bearing: String,
    #[serde(rename = "trueBearingAccuracy")]
    pub aixm_true_bearing_accuracy: AixmTrueBearingAccuracy,
    #[serde(rename = "declination")]
    pub aixm_declination: AixmDeclination,
    #[serde(rename = "widthCourse")]
    pub aixm_width_course: AixmWidthCourse,
    #[serde(rename = "widthCourseAccuracy")]
    pub aixm_width_course_accuracy: AixmWidthCourseAccuracy,
    #[serde(rename = "backCourseUsable")]
    pub aixm_back_course_usable: AixmBackCourseUsable,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMarkerBeacon {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmMarkerBeaconAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMarkerBeaconAixmTimeSlice {
    #[serde(rename = "MarkerBeaconTimeSlice")]
    pub aixm_marker_beacon_time_slice: AixmMarkerBeaconTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmMarkerBeaconTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "name")]
    pub aixm_name: String,
    #[serde(rename = "emissionClass")]
    pub aixm_emission_class: AixmEmissionClass,
    #[serde(rename = "mobile")]
    pub aixm_mobile: AixmMobile,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: AixmMagneticVariation,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: AixmMagneticVariationAccuracy,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: AixmDateMagneticVariation,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: AixmFlightChecked,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "authority")]
    pub aixm_authority: Option<AixmAuthority>,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<Vec<AixmAnnotation>>,
    #[serde(rename = "class")]
    pub aixm_class: AixmClass,
    #[serde(rename = "frequency")]
    pub aixm_frequency: AixmFrequency,
    #[serde(rename = "axisBearing")]
    pub aixm_axis_bearing: AixmAxisBearing,
    #[serde(rename = "auralMorseCode")]
    pub aixm_aural_morse_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNdb {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmNdbAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNdbAixmTimeSlice {
    #[serde(rename = "NDBTimeSlice")]
    pub aixm_ndbtime_slice: AixmNdbtimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNdbtimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "name")]
    pub aixm_name: String,
    #[serde(rename = "emissionClass")]
    pub aixm_emission_class: AixmEmissionClass,
    #[serde(rename = "mobile")]
    pub aixm_mobile: AixmMobile,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: String,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: AixmMagneticVariationAccuracy,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: AixmDateMagneticVariation,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: AixmFlightChecked,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "authority")]
    pub aixm_authority: Option<AixmAuthority>,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmNavaidAvailability,
    #[serde(rename = "frequency")]
    pub aixm_frequency: AixmFrequency,
    #[serde(rename = "class")]
    pub aixm_class: String,
    #[serde(rename = "emissionBand")]
    pub aixm_emission_band: AixmEmissionBand,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<Vec<AixmAnnotation>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNavaid {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmNavaidAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNavaidAixmTimeSlice {
    #[serde(rename = "NavaidTimeSlice")]
    pub aixm_navaid_time_slice: AixmNavaidTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNavaidTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "name")]
    pub aixm_name: AixmName,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: AixmFlightChecked,
    #[serde(rename = "purpose")]
    pub aixm_purpose: AixmPurpose,
    #[serde(rename = "signalPerformance")]
    pub aixm_signal_performance: AixmSignalPerformance,
    #[serde(rename = "courseQuality")]
    pub aixm_course_quality: AixmCourseQuality,
    #[serde(rename = "integrityLevel")]
    pub aixm_integrity_level: AixmIntegrityLevel,
    #[serde(rename = "navaidEquipment")]
    pub aixm_navaid_equipment: Vec<AixmNavaidEquipment>,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "runwayDirection")]
    pub aixm_runway_direction: Option<AixmRunwayDirectionLink>,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<Vec<AixmAnnotation>>,
    #[serde(rename = "servedAirport")]
    pub aixm_served_airport: Option<Vec<AixmServedAirport>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRadioFrequencyArea {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmRadioFrequencyAreaAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRadioFrequencyAreaAixmTimeSlice {
    #[serde(rename = "RadioFrequencyAreaTimeSlice")]
    pub aixm_radio_frequency_area_time_slice: AixmRadioFrequencyAreaTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRadioFrequencyAreaTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "angleScallop")]
    pub aixm_angle_scallop: AixmAngleScallop,
    #[serde(rename = "signalType")]
    pub aixm_signal_type: String,
    #[serde(rename = "equipment_navaidEquipment")]
    pub aixm_equipment_navaid_equipment: AixmEquipmentNavaidEquipment,
    #[serde(rename = "sector")]
    pub aixm_sector: AixmSector,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTacan {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmTacanAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTacanAixmTimeSlice {
    #[serde(rename = "TACANTimeSlice")]
    pub aixm_tacantime_slice: AixmTacantimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTacantimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "name")]
    pub aixm_name: String,
    #[serde(rename = "emissionClass")]
    pub aixm_emission_class: AixmEmissionClass,
    #[serde(rename = "mobile")]
    pub aixm_mobile: AixmMobile,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: String,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: AixmMagneticVariationAccuracy,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: String,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: AixmFlightChecked,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "authority")]
    pub aixm_authority: Option<AixmAuthority>,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmNavaidAvailability,
    #[serde(rename = "channel")]
    pub aixm_channel: String,
    #[serde(rename = "declination")]
    pub aixm_declination: AixmDeclination,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVor {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmVorAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVorAixmTimeSlice {
    #[serde(rename = "VORTimeSlice")]
    pub aixm_vortime_slice: AixmVortimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVortimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "name")]
    pub aixm_name: String,
    #[serde(rename = "emissionClass")]
    pub aixm_emission_class: AixmEmissionClass,
    #[serde(rename = "mobile")]
    pub aixm_mobile: AixmMobile,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: String,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: AixmMagneticVariationAccuracy,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: String,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: AixmFlightChecked,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "authority")]
    pub aixm_authority: Option<AixmAuthority>,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmNavaidAvailability,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "frequency")]
    pub aixm_frequency: AixmFrequency,
    #[serde(rename = "zeroBearingDirection")]
    pub aixm_zero_bearing_direction: String,
    #[serde(rename = "declination")]
    pub aixm_declination: AixmDeclination,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAngleIndication {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmAngleIndicationAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAngleIndicationAixmTimeSlice {
    #[serde(rename = "AngleIndicationTimeSlice")]
    pub aixm_angle_indication_time_slice: AixmAngleIndicationTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAngleIndicationTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "angle")]
    pub aixm_angle: String,
    #[serde(rename = "angleType")]
    pub aixm_angle_type: AixmAngleType,
    #[serde(rename = "indicationDirection")]
    pub aixm_indication_direction: AixmIndicationDirection,
    #[serde(rename = "trueAngle")]
    pub aixm_true_angle: AixmTrueAngle,
    #[serde(rename = "cardinalDirection")]
    pub aixm_cardinal_direction: AixmCardinalDirection,
    #[serde(rename = "minimumReceptionAltitude")]
    pub aixm_minimum_reception_altitude: AixmMinimumReceptionAltitude,
    #[serde(rename = "fix")]
    pub aixm_fix: AixmFix,
    #[serde(rename = "pointChoice_navaidSystem")]
    pub aixm_point_choice_navaid_system: AixmPointChoiceNavaidSystem,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDesignatedPoint {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmDesignatedPointAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDesignatedPointAixmTimeSlice {
    #[serde(rename = "DesignatedPointTimeSlice")]
    pub aixm_designated_point_time_slice: AixmDesignatedPointTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDesignatedPointTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "name")]
    pub aixm_name: AixmName,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<Vec<AixmAnnotation>>,
    #[serde(rename = "airportHeliport")]
    pub aixm_airport_heliport: Option<AixmAirportHeliportLink>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDistanceIndication {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmDistanceIndicationAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDistanceIndicationAixmTimeSlice {
    #[serde(rename = "DistanceIndicationTimeSlice")]
    pub aixm_distance_indication_time_slice: AixmDistanceIndicationTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDistanceIndicationTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "distance")]
    pub aixm_distance: AixmDistance,
    #[serde(rename = "minimumReceptionAltitude")]
    pub aixm_minimum_reception_altitude: AixmMinimumReceptionAltitude,
    #[serde(rename = "type")]
    pub aixm_type: AixmType,
    #[serde(rename = "fix")]
    pub aixm_fix: AixmFix,
    #[serde(rename = "pointChoice_navaidSystem")]
    pub aixm_point_choice_navaid_system: AixmPointChoiceNavaidSystem,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliport {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmAirportHeliportAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportAixmTimeSlice {
    #[serde(rename = "AirportHeliportTimeSlice")]
    pub aixm_airport_heliport_time_slice: AixmAirportHeliportTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: Option<String>,
    #[serde(rename = "name")]
    pub aixm_name: String,
    #[serde(rename = "locationIndicatorICAO")]
    pub aixm_location_indicator_icao: AixmLocationIndicatorIcao,
    #[serde(rename = "designatorIATA")]
    pub aixm_designator_iata: AixmDesignatorIata,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "controlType")]
    pub aixm_control_type: AixmControlType,
    #[serde(rename = "fieldElevation")]
    pub aixm_field_elevation: AixmFieldElevation,
    #[serde(rename = "verticalDatum")]
    pub aixm_vertical_datum: AixmVerticalDatum,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: AixmMagneticVariation,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: AixmMagneticVariationAccuracy,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: AixmDateMagneticVariation,
    #[serde(rename = "magneticVariationChange")]
    pub aixm_magnetic_variation_change: AixmMagneticVariationChange,
    #[serde(rename = "referenceTemperature")]
    pub aixm_reference_temperature: AixmReferenceTemperature,
    #[serde(rename = "transitionAltitude")]
    pub aixm_transition_altitude: AixmTransitionAltitude,
    #[serde(rename = "transitionLevel")]
    pub aixm_transition_level: AixmTransitionLevel,
    #[serde(rename = "lowestTemperature")]
    pub aixm_lowest_temperature: AixmLowestTemperature,
    #[serde(rename = "abandoned")]
    pub aixm_abandoned: String,
    #[serde(rename = "certificationDate")]
    pub aixm_certification_date: AixmCertificationDate,
    #[serde(rename = "certificationExpirationDate")]
    pub aixm_certification_expiration_date: AixmCertificationExpirationDate,
    #[serde(rename = "responsibleOrganisation")]
    pub aixm_responsible_organisation: AixmResponsibleOrganisation,
    #[serde(rename = "ARP")]
    pub aixm_arp: AixmArp,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmAirportHeliportAixmAvailability,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Vec<AixmAnnotation>,
    #[serde(rename = "servedCity")]
    pub aixm_served_city: Option<AixmServedCity>,
    #[serde(rename = "contact")]
    pub aixm_contact: Option<AixmContact>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportCollocation {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmAirportHeliportCollocationAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportCollocationAixmTimeSlice {
    #[serde(rename = "AirportHeliportCollocationTimeSlice")]
    pub aixm_airport_heliport_collocation_time_slice: AixmAirportHeliportCollocationTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportCollocationTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: Option<String>,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: Option<String>,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "hostAirport")]
    pub aixm_host_airport: AixmHostAirport,
    #[serde(rename = "dependentAirport")]
    pub aixm_dependent_airport: AixmDependentAirport,
    #[serde(rename = "annotation")]
    pub aixm_annotation: AixmAnnotation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRoute {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmRouteAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRouteAixmTimeSlice {
    #[serde(rename = "RouteTimeSlice")]
    pub aixm_route_time_slice: AixmRouteTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRouteTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designatorPrefix")]
    pub aixm_designator_prefix: AixmDesignatorPrefix,
    #[serde(rename = "designatorSecondLetter")]
    pub aixm_designator_second_letter: String,
    #[serde(rename = "designatorNumber")]
    pub aixm_designator_number: String,
    #[serde(rename = "multipleIdentifier")]
    pub aixm_multiple_identifier: AixmMultipleIdentifier,
    #[serde(rename = "locationDesignator")]
    pub aixm_location_designator: String,
    #[serde(rename = "name")]
    pub aixm_name: AixmName,
    #[serde(rename = "type")]
    pub aixm_type: AixmType,
    #[serde(rename = "flightRule")]
    pub aixm_flight_rule: AixmFlightRule,
    #[serde(rename = "internationalUse")]
    pub aixm_international_use: AixmInternationalUse,
    #[serde(rename = "militaryUse")]
    pub aixm_military_use: AixmMilitaryUse,
    #[serde(rename = "militaryTrainingType")]
    pub aixm_military_training_type: AixmMilitaryTrainingType,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRouteSegment {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmRouteSegmentAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRouteSegmentAixmTimeSlice {
    #[serde(rename = "RouteSegmentTimeSlice")]
    pub aixm_route_segment_time_slice: AixmRouteSegmentTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRouteSegmentTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "level")]
    pub aixm_level: String,
    #[serde(rename = "upperLimit")]
    pub aixm_upper_limit: AixmUpperLimit,
    #[serde(rename = "upperLimitReference")]
    pub aixm_upper_limit_reference: String,
    #[serde(rename = "lowerLimit")]
    pub aixm_lower_limit: AixmLowerLimit,
    #[serde(rename = "lowerLimitReference")]
    pub aixm_lower_limit_reference: String,
    #[serde(rename = "minimumObstacleClearanceAltitude")]
    pub aixm_minimum_obstacle_clearance_altitude: AixmMinimumObstacleClearanceAltitude,
    #[serde(rename = "pathType")]
    pub aixm_path_type: AixmPathType,
    #[serde(rename = "trueTrack")]
    pub aixm_true_track: AixmTrueTrack,
    #[serde(rename = "magneticTrack")]
    pub aixm_magnetic_track: AixmMagneticTrack,
    #[serde(rename = "reverseTrueTrack")]
    pub aixm_reverse_true_track: AixmReverseTrueTrack,
    #[serde(rename = "reverseMagneticTrack")]
    pub aixm_reverse_magnetic_track: AixmReverseMagneticTrack,
    #[serde(rename = "length")]
    pub aixm_length: AixmLength,
    #[serde(rename = "widthLeft")]
    pub aixm_width_left: AixmWidthLeft,
    #[serde(rename = "widthRight")]
    pub aixm_width_right: AixmWidthRight,
    #[serde(rename = "turnDirection")]
    pub aixm_turn_direction: AixmTurnDirection,
    #[serde(rename = "signalGap")]
    pub aixm_signal_gap: AixmSignalGap,
    #[serde(rename = "minimumEnrouteAltitude")]
    pub aixm_minimum_enroute_altitude: AixmMinimumEnrouteAltitude,
    #[serde(rename = "minimumCrossingAtEnd")]
    pub aixm_minimum_crossing_at_end: AixmMinimumCrossingAtEnd,
    #[serde(rename = "minimumCrossingAtEndReference")]
    pub aixm_minimum_crossing_at_end_reference: AixmMinimumCrossingAtEndReference,
    #[serde(rename = "maximumCrossingAtEnd")]
    pub aixm_maximum_crossing_at_end: AixmMaximumCrossingAtEnd,
    #[serde(rename = "maximumCrossingAtEndReference")]
    pub aixm_maximum_crossing_at_end_reference: AixmMaximumCrossingAtEndReference,
    #[serde(rename = "navigationType")]
    pub aixm_navigation_type: String,
    #[serde(rename = "requiredNavigationPerformance")]
    pub aixm_required_navigation_performance: AixmRequiredNavigationPerformance,
    #[serde(rename = "designatorSuffix")]
    pub aixm_designator_suffix: AixmDesignatorSuffix,
    #[serde(rename = "start")]
    pub aixm_start: AixmStart,
    #[serde(rename = "routeFormed")]
    pub aixm_route_formed: AixmRouteFormed,
    #[serde(rename = "curveExtent")]
    pub aixm_curve_extent: AixmCurveExtent,
    #[serde(rename = "end")]
    pub aixm_end: AixmEnd,
    #[serde(rename = "availability")]
    pub aixm_availability: Vec<AixmRouteSegmentAixmAvailability>,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<Vec<AixmAnnotation>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStandardLevelColumn {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmStandardLevelColumnAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStandardLevelColumnAixmTimeSlice {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "StandardLevelColumnTimeSlice")]
    pub aixm_standard_level_column_time_slice: AixmStandardLevelColumnTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStandardLevelColumnTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "series")]
    pub aixm_series: String,
    #[serde(rename = "unitOfMeasurement")]
    pub aixm_unit_of_measurement: AixmUnitOfMeasurement,
    #[serde(rename = "separation")]
    pub aixm_separation: String,
    #[serde(rename = "level")]
    pub aixm_level: Vec<AixmLevel>,
    #[serde(rename = "levelTable")]
    pub aixm_level_table: AixmLevelTable,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStandardLevelTable {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmStandardLevelTableAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStandardLevelTableAixmTimeSlice {
    #[serde(rename = "StandardLevelTableTimeSlice")]
    pub aixm_standard_level_table_time_slice: AixmStandardLevelTableTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmStandardLevelTableTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "name")]
    pub aixm_name: String,
    #[serde(rename = "standardICAO")]
    pub aixm_standard_icao: AixmStandardIcao,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunway {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmRunwayAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayAixmTimeSlice {
    #[serde(rename = "RunwayTimeSlice")]
    pub aixm_runway_time_slice: AixmRunwayTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: String,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: String,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "nominalLength")]
    pub aixm_nominal_length: AixmNominalLength,
    #[serde(rename = "lengthAccuracy")]
    pub aixm_length_accuracy: AixmLengthAccuracy,
    #[serde(rename = "nominalWidth")]
    pub aixm_nominal_width: AixmNominalWidth,
    #[serde(rename = "widthAccuracy")]
    pub aixm_width_accuracy: AixmWidthAccuracy,
    #[serde(rename = "widthShoulder")]
    pub aixm_width_shoulder: AixmWidthShoulder,
    #[serde(rename = "lengthStrip")]
    pub aixm_length_strip: AixmLengthStrip,
    #[serde(rename = "widthStrip")]
    pub aixm_width_strip: AixmWidthStrip,
    #[serde(rename = "lengthOffset")]
    pub aixm_length_offset: AixmLengthOffset,
    #[serde(rename = "widthOffset")]
    pub aixm_width_offset: AixmWidthOffset,
    #[serde(rename = "abandoned")]
    pub aixm_abandoned: AixmAbandoned,
    #[serde(rename = "surfaceProperties")]
    pub aixm_surface_properties: AixmSurfaceProperties,
    #[serde(rename = "associatedAirportHeliport")]
    pub aixm_associated_airport_heliport: AixmAirportHeliportLink,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<Vec<AixmAnnotation>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayCentrelinePoint {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmRunwayCentrelinePointAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayCentrelinePointAixmTimeSlice {
    #[serde(rename = "RunwayCentrelinePointTimeSlice")]
    pub aixm_runway_centreline_point_time_slice: AixmRunwayCentrelinePointTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayCentrelinePointTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: String,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: String,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "role")]
    pub aixm_role: String,
    #[serde(rename = "designator")]
    pub aixm_designator: AixmDesignator,
    #[serde(rename = "location")]
    pub aixm_location: Option<AixmLocation>,
    #[serde(rename = "onRunway")]
    pub aixm_on_runway: AixmOnRunway,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
    #[serde(rename = "associatedDeclaredDistance")]
    pub aixm_associated_declared_distance: Option<Vec<AixmAssociatedDeclaredDistance>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayDirection {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmRunwayDirectionAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayDirectionAixmTimeSlice {
    #[serde(rename = "RunwayDirectionTimeSlice")]
    pub aixm_runway_direction_time_slice: AixmRunwayDirectionTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRunwayDirectionTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: String,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: String,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "trueBearing")]
    pub aixm_true_bearing: AixmTrueBearing,
    #[serde(rename = "trueBearingAccuracy")]
    pub aixm_true_bearing_accuracy: AixmTrueBearingAccuracy,
    #[serde(rename = "magneticBearing")]
    pub aixm_magnetic_bearing: AixmMagneticBearing,
    #[serde(rename = "patternVFR")]
    pub aixm_pattern_vfr: AixmPatternVfr,
    #[serde(rename = "slopeTDZ")]
    pub aixm_slope_tdz: AixmSlopeTdz,
    #[serde(rename = "elevationTDZ")]
    pub aixm_elevation_tdz: AixmElevationTdz,
    #[serde(rename = "elevationTDZAccuracy")]
    pub aixm_elevation_tdzaccuracy: AixmElevationTdzaccuracy,
    #[serde(rename = "approachMarkingType")]
    pub aixm_approach_marking_type: AixmApproachMarkingType,
    #[serde(rename = "approachMarkingCondition")]
    pub aixm_approach_marking_condition: AixmApproachMarkingCondition,
    #[serde(rename = "classLightingJAR")]
    pub aixm_class_lighting_jar: AixmClassLightingJar,
    #[serde(rename = "precisionApproachGuidance")]
    pub aixm_precision_approach_guidance: AixmPrecisionApproachGuidance,
    #[serde(rename = "usedRunway")]
    pub aixm_used_runway: AixmUsedRunway,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<Vec<AixmAnnotation>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVisualGlideSlopeIndicator {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "identifier")]
    pub gml_identifier: GmlIdentifier,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmVisualGlideSlopeIndicatorAixmTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVisualGlideSlopeIndicatorAixmTimeSlice {
    #[serde(rename = "VisualGlideSlopeIndicatorTimeSlice")]
    pub aixm_visual_glide_slope_indicator_time_slice: AixmVisualGlideSlopeIndicatorTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVisualGlideSlopeIndicatorTimeSlice {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "validTime")]
    pub gml_valid_time: GmlValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "sequenceNumber")]
    pub aixm_sequence_number: String,
    #[serde(rename = "correctionNumber")]
    pub aixm_correction_number: String,
    #[serde(rename = "featureLifetime")]
    pub aixm_feature_lifetime: AixmFeatureLifetime,
    #[serde(rename = "emergencyLighting")]
    pub aixm_emergency_lighting: AixmEmergencyLighting,
    #[serde(rename = "intensityLevel")]
    pub aixm_intensity_level: AixmIntensityLevel,
    #[serde(rename = "colour")]
    pub aixm_colour: AixmColour,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "position")]
    pub aixm_position: AixmPosition,
    #[serde(rename = "numberBox")]
    pub aixm_number_box: AixmNumberBox,
    #[serde(rename = "portable")]
    pub aixm_portable: AixmPortable,
    #[serde(rename = "slopeAngle")]
    pub aixm_slope_angle: AixmSlopeAngle,
    #[serde(rename = "minimumEyeHeightOverThreshold")]
    pub aixm_minimum_eye_height_over_threshold: AixmMinimumEyeHeightOverThreshold,
    #[serde(rename = "runwayDirection")]
    pub aixm_runway_direction: AixmRunwayDirectionLink,
}
