#![allow(clippy::struct_field_names, reason = "XML namespaces")]
use annotation::AixmAnnotation;
use gmd::GmdMdMetadata;
use gml::{GmlIdentifier, GmlSegments, GmlTimePeriod, GmlValidTime};
use serde::{Deserialize, Serialize};
use time_interval::AixmTimeInterval;

mod annotation;
mod gmd;
mod gml;
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
pub struct AixmReferenceTemperature {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTransitionAltitude {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmTransitionLevel {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLowestTemperature {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: i32,
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
    pub location: LocationType,
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
    pub aixm_horizontal_accuracy: Option<AixmHorizontalAccuracy>,
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
    pub aixm_horizontal_accuracy: Option<AixmHorizontalAccuracy>,
    #[serde(rename = "elevation")]
    pub aixm_elevation: Option<AixmElevation>,
    #[serde(rename = "geoidUndulation")]
    pub aixm_geoid_undulation: Option<AixmGeoidUndulation>,
    #[serde(rename = "verticalDatum")]
    pub aixm_vertical_datum: Option<String>,
    #[serde(rename = "verticalAccuracy")]
    pub aixm_vertical_accuracy: Option<AixmVerticalAccuracy>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmHorizontalAccuracy {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmElevation {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmGeoidUndulation {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVerticalAccuracy {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmResponsibleOrganisation {
    #[serde(rename = "AirportHeliportResponsibilityOrganisation")]
    pub aixm_airport_heliport_responsibility_organisation:
        Option<AixmAirportHeliportResponsibilityOrganisation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportResponsibilityOrganisation {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "role")]
    pub aixm_role: Option<String>,
    #[serde(rename = "theOrganisationAuthority")]
    pub aixm_the_organisation_authority: AixmTheOrganisationAuthority,
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
    pub aixm_name: Option<String>,
    #[serde(rename = "address", default)]
    pub aixm_address: Vec<AixmAddress>,
    #[serde(rename = "phoneFax")]
    pub aixm_phone_fax: Option<AixmPhoneFax>,
    #[serde(rename = "networkNode", default)]
    pub aixm_network_node: Vec<AixmNetworkNode>,
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
    pub aixm_city: Option<String>,
    #[serde(rename = "postalCode")]
    pub aixm_postal_code: Option<String>,
    #[serde(rename = "country")]
    pub aixm_country: Option<String>,
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
    pub aixm_linkage: Option<String>,
    #[serde(rename = "eMail")]
    pub aixm_e_mail: Option<String>,
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
    pub aixm_operational_status: Option<String>,
    #[serde(rename = "warning")]
    pub aixm_warning: Option<String>,
    #[serde(rename = "annotation", default)]
    pub aixm_annotation: Vec<AixmAnnotation>,
    #[serde(rename = "usage", default)]
    pub aixm_usage: Vec<AixmUsage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNavaidOperationalStatus {
    #[serde(rename = "@id")]
    pub gml_id: String,
    #[serde(rename = "timeInterval")]
    pub aixm_time_interval: Option<AixmTimeInterval>,
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
    pub aixm_origin: Option<String>,
    #[serde(rename = "purpose")]
    pub aixm_purpose: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmGhostFrequency {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmFrequency {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmRdh {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: u32,
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
    pub aixm_collocation_group: Option<String>,
    #[serde(rename = "providesNavigableLocation")]
    pub aixm_provides_navigable_location: Option<String>,
    #[serde(rename = "theNavaidEquipment")]
    pub aixm_the_navaid_equipment: AixmTheNavaidEquipment,
    #[serde(rename = "markerPosition")]
    pub aixm_marker_position: Option<String>,
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
    pub aixm_arc_direction: Option<String>,
    #[serde(rename = "fromAngle")]
    pub aixm_from_angle: String,
    #[serde(rename = "toAngle")]
    pub aixm_to_angle: String,
    #[serde(rename = "angleType")]
    pub aixm_angle_type: Option<String>,
    #[serde(rename = "angleDirectionReference")]
    pub aixm_angle_direction_reference: Option<String>,
    #[serde(rename = "innerDistance")]
    pub aixm_inner_distance: Option<AixmInnerDistance>,
    #[serde(rename = "outerDistance")]
    pub aixm_outer_distance: Option<AixmOuterDistance>,
    #[serde(rename = "upperLimit")]
    pub aixm_upper_limit: Option<AixmUpperLimit>,
    #[serde(rename = "upperLimitReference")]
    pub aixm_upper_limit_reference: Option<String>,
    #[serde(rename = "lowerLimit")]
    pub aixm_lower_limit: Option<AixmLowerLimit>,
    #[serde(rename = "lowerLimitReference")]
    pub aixm_lower_limit_reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmInnerDistance {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmOuterDistance {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmUpperLimit {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLowerLimit {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmFix {
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmPointChoiceNavaidSystem {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmAirportHeliportLink {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@title")]
    pub xlink_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDistance {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmFieldElevation {
    // DFS includes data with xsi:nil="false"
    #[serde(rename = "@nil")]
    pub xsi_nil: Option<String>,
    #[serde(rename = "@nilReason")]
    pub nil_reason: Option<String>,
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: i32,
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
pub struct AixmLength {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidth {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
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
    pub aixm_cardinal_direction: Option<String>,
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
    pub aixm_upper_limit: Option<AixmUpperLimit>,
    #[serde(rename = "upperLimitReference")]
    pub aixm_upper_limit_reference: Option<String>,
    #[serde(rename = "lowerLimit")]
    pub aixm_lower_limit: Option<AixmLowerLimit>,
    #[serde(rename = "lowerLimitReference")]
    pub aixm_lower_limit_reference: Option<String>,
    #[serde(rename = "altitudeInterpretation")]
    pub aixm_altitude_interpretation: Option<String>,
    #[serde(rename = "discreteLevelSeries")]
    pub aixm_discrete_level_series: Option<AixmDiscreteLevelSeries>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmDiscreteLevelSeries {
    #[serde(rename = "@href")]
    pub xlink_href: String,
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
pub struct AixmNominalLength {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmNominalWidth {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmLengthStrip {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWidthStrip {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: u32,
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
    pub aixm_preparation: Option<String>,
    #[serde(rename = "surfaceCondition")]
    pub aixm_surface_condition: Option<String>,
    #[serde(rename = "classPCN")]
    pub aixm_class_pcn: Option<String>,
    #[serde(rename = "pavementTypePCN")]
    pub aixm_pavement_type_pcn: Option<String>,
    #[serde(rename = "pavementSubgradePCN")]
    pub aixm_pavement_subgrade_pcn: Option<String>,
    #[serde(rename = "maxTyrePressurePCN")]
    pub aixm_max_tyre_pressure_pcn: Option<String>,
    #[serde(rename = "evaluationMethodPCN")]
    pub aixm_evaluation_method_pcn: Option<String>,
    #[serde(rename = "classLCN")]
    pub aixm_class_lcn: Option<String>,
    #[serde(rename = "weightSIWL")]
    pub aixm_weight_siwl: Option<String>,
    #[serde(rename = "tyrePressureSIWL")]
    pub aixm_tyre_pressure_siwl: Option<String>,
    #[serde(rename = "weightAUW")]
    pub aixm_weight_auw: Option<AixmWeightAuw>,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmWeightAuw {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
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
    pub aixm_distance_accuracy: Option<String>,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmElevationTdz {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: u32,
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
    pub aixm_width: Option<AixmWidth>,
    #[serde(rename = "length")]
    pub aixm_length: Option<AixmLength>,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "status")]
    pub aixm_status: Option<String>,
    #[serde(rename = "protectedRunwayDirection")]
    pub aixm_protected_runway_direction: AixmProtectedRunwayDirection,
    #[serde(rename = "surfaceProperties")]
    pub aixm_surface_properties: Option<AixmSurfaceProperties>,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
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
pub struct AixmMinimumEyeHeightOverThreshold {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub value: f64,
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
    pub aixm_emission_class: Option<String>,
    #[serde(rename = "mobile")]
    pub aixm_mobile: Option<String>,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: Option<f64>,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: Option<f64>,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: Option<u32>,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: Option<String>,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "authority")]
    pub aixm_authority: Option<AixmAuthority>,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmNavaidAvailability,
    #[serde(rename = "type")]
    pub aixm_type: Option<String>,
    #[serde(rename = "channel")]
    pub aixm_channel: String,
    #[serde(rename = "ghostFrequency")]
    pub aixm_ghost_frequency: Option<AixmGhostFrequency>,
    #[serde(rename = "displace")]
    pub aixm_displace: Option<String>,
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
    pub aixm_name: Option<String>,
    #[serde(rename = "emissionClass")]
    pub aixm_emission_class: Option<String>,
    #[serde(rename = "mobile")]
    pub aixm_mobile: Option<String>,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: Option<f64>,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: Option<f64>,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: Option<u32>,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: Option<String>,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmNavaidAvailability,
    #[serde(rename = "frequency")]
    pub aixm_frequency: AixmFrequency,
    #[serde(rename = "slope")]
    pub aixm_slope: String,
    #[serde(rename = "angleAccuracy")]
    pub aixm_angle_accuracy: Option<String>,
    #[serde(rename = "rdh")]
    pub aixm_rdh: Option<AixmRdh>,
    #[serde(rename = "rdhAccuracy")]
    pub aixm_rdh_accuracy: Option<String>,
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
    pub aixm_name: Option<String>,
    #[serde(rename = "emissionClass")]
    pub aixm_emission_class: Option<String>,
    #[serde(rename = "mobile")]
    pub aixm_mobile: Option<String>,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: Option<f64>,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: Option<f64>,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: String,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: Option<String>,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmNavaidAvailability,
    #[serde(rename = "frequency")]
    pub aixm_frequency: AixmFrequency,
    #[serde(rename = "magneticBearing")]
    pub aixm_magnetic_bearing: String,
    #[serde(rename = "magneticBearingAccuracy")]
    pub aixm_magnetic_bearing_accuracy: Option<String>,
    #[serde(rename = "trueBearing")]
    pub aixm_true_bearing: String,
    #[serde(rename = "trueBearingAccuracy")]
    pub aixm_true_bearing_accuracy: Option<String>,
    #[serde(rename = "declination")]
    pub aixm_declination: Option<String>,
    #[serde(rename = "widthCourse")]
    pub aixm_width_course: Option<u32>,
    #[serde(rename = "widthCourseAccuracy")]
    pub aixm_width_course_accuracy: Option<String>,
    #[serde(rename = "backCourseUsable")]
    pub aixm_back_course_usable: Option<String>,
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
    pub aixm_emission_class: Option<String>,
    #[serde(rename = "mobile")]
    pub aixm_mobile: Option<String>,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: Option<f64>,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: Option<f64>,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: Option<u32>,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: Option<String>,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "authority")]
    pub aixm_authority: Option<AixmAuthority>,
    #[serde(rename = "annotation", default)]
    pub aixm_annotation: Vec<AixmAnnotation>,
    #[serde(rename = "class")]
    pub aixm_class: Option<String>,
    #[serde(rename = "frequency")]
    pub aixm_frequency: AixmFrequency,
    #[serde(rename = "axisBearing")]
    pub aixm_axis_bearing: Option<f64>,
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
    pub aixm_emission_class: Option<String>,
    #[serde(rename = "mobile")]
    pub aixm_mobile: Option<String>,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: Option<f64>,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: Option<f64>,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: Option<u32>,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: Option<String>,
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
    pub aixm_emission_band: Option<String>,
    #[serde(rename = "annotation", default)]
    pub aixm_annotation: Vec<AixmAnnotation>,
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
    pub aixm_name: Option<String>,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: Option<String>,
    #[serde(rename = "purpose")]
    pub aixm_purpose: Option<String>,
    #[serde(rename = "signalPerformance")]
    pub aixm_signal_performance: Option<String>,
    #[serde(rename = "courseQuality")]
    pub aixm_course_quality: Option<String>,
    #[serde(rename = "integrityLevel")]
    pub aixm_integrity_level: Option<String>,
    #[serde(rename = "navaidEquipment")]
    pub aixm_navaid_equipment: Vec<AixmNavaidEquipment>,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "runwayDirection")]
    pub aixm_runway_direction: Option<AixmRunwayDirectionLink>,
    #[serde(rename = "annotation", default)]
    pub aixm_annotation: Vec<AixmAnnotation>,
    #[serde(rename = "servedAirport", default)]
    pub aixm_served_airport: Vec<AixmServedAirport>,
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
    pub aixm_angle_scallop: Option<String>,
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
    pub aixm_emission_class: Option<String>,
    #[serde(rename = "mobile")]
    pub aixm_mobile: Option<String>,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: Option<f64>,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: Option<f64>,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: String,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: Option<String>,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "authority")]
    pub aixm_authority: Option<AixmAuthority>,
    #[serde(rename = "availability")]
    pub aixm_availability: AixmNavaidAvailability,
    #[serde(rename = "channel")]
    pub aixm_channel: String,
    #[serde(rename = "declination")]
    pub aixm_declination: Option<String>,
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
    pub aixm_vortime_slice: AixmVorTimeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AixmVorTimeSlice {
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
    pub aixm_emission_class: Option<String>,
    #[serde(rename = "mobile")]
    pub aixm_mobile: Option<String>,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: Option<f64>,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: Option<f64>,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: String,
    #[serde(rename = "flightChecked")]
    pub aixm_flight_checked: Option<String>,
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
    pub aixm_declination: Option<String>,
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
    pub aixm_angle_type: Option<String>,
    #[serde(rename = "indicationDirection")]
    pub aixm_indication_direction: Option<String>,
    #[serde(rename = "trueAngle")]
    pub aixm_true_angle: Option<f64>,
    #[serde(rename = "cardinalDirection")]
    pub aixm_cardinal_direction: Option<String>,
    #[serde(rename = "minimumReceptionAltitude")]
    pub aixm_minimum_reception_altitude: Option<String>,
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
    pub aixm_name: Option<String>,
    #[serde(rename = "location")]
    pub aixm_location: AixmLocation,
    #[serde(rename = "annotation", default)]
    pub aixm_annotation: Vec<AixmAnnotation>,
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
    pub aixm_minimum_reception_altitude: Option<String>,
    #[serde(rename = "type")]
    pub aixm_type: Option<String>,
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
    pub aixm_location_indicator_icao: Option<String>,
    #[serde(rename = "designatorIATA")]
    pub aixm_designator_iata: Option<String>,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "controlType")]
    pub aixm_control_type: Option<String>,
    #[serde(rename = "fieldElevation")]
    pub aixm_field_elevation: Option<AixmFieldElevation>,
    #[serde(rename = "verticalDatum")]
    pub aixm_vertical_datum: Option<String>,
    #[serde(rename = "magneticVariation")]
    pub aixm_magnetic_variation: Option<f64>,
    #[serde(rename = "magneticVariationAccuracy")]
    pub aixm_magnetic_variation_accuracy: Option<f64>,
    #[serde(rename = "dateMagneticVariation")]
    pub aixm_date_magnetic_variation: Option<u32>,
    #[serde(rename = "magneticVariationChange")]
    pub aixm_magnetic_variation_change: Option<f64>,
    #[serde(rename = "referenceTemperature")]
    pub aixm_reference_temperature: Option<AixmReferenceTemperature>,
    #[serde(rename = "transitionAltitude")]
    pub aixm_transition_altitude: Option<AixmTransitionAltitude>,
    #[serde(rename = "transitionLevel")]
    pub aixm_transition_level: Option<AixmTransitionLevel>,
    #[serde(rename = "lowestTemperature")]
    pub aixm_lowest_temperature: Option<AixmLowestTemperature>,
    #[serde(rename = "abandoned")]
    pub aixm_abandoned: String,
    #[serde(rename = "certificationDate")]
    pub aixm_certification_date: Option<String>,
    #[serde(rename = "certificationExpirationDate")]
    pub aixm_certification_expiration_date: Option<String>,
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
    pub aixm_annotation: Option<AixmAnnotation>,
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
    pub aixm_designator_prefix: Option<String>,
    #[serde(rename = "designatorSecondLetter")]
    pub aixm_designator_second_letter: String,
    #[serde(rename = "designatorNumber")]
    pub aixm_designator_number: String,
    #[serde(rename = "multipleIdentifier")]
    pub aixm_multiple_identifier: Option<String>,
    #[serde(rename = "locationDesignator")]
    pub aixm_location_designator: String,
    #[serde(rename = "name")]
    pub aixm_name: Option<String>,
    #[serde(rename = "type")]
    pub aixm_type: Option<String>,
    #[serde(rename = "flightRule")]
    pub aixm_flight_rule: Option<String>,
    #[serde(rename = "internationalUse")]
    pub aixm_international_use: Option<String>,
    #[serde(rename = "militaryUse")]
    pub aixm_military_use: Option<String>,
    #[serde(rename = "militaryTrainingType")]
    pub aixm_military_training_type: Option<String>,
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
    pub aixm_minimum_obstacle_clearance_altitude: Option<String>,
    #[serde(rename = "pathType")]
    pub aixm_path_type: Option<String>,
    #[serde(rename = "trueTrack")]
    pub aixm_true_track: Option<f64>,
    #[serde(rename = "magneticTrack")]
    pub aixm_magnetic_track: Option<f64>,
    #[serde(rename = "reverseTrueTrack")]
    pub aixm_reverse_true_track: Option<f64>,
    #[serde(rename = "reverseMagneticTrack")]
    pub aixm_reverse_magnetic_track: Option<f64>,
    #[serde(rename = "length")]
    pub aixm_length: AixmLength,
    #[serde(rename = "widthLeft")]
    pub aixm_width_left: Option<String>,
    #[serde(rename = "widthRight")]
    pub aixm_width_right: Option<String>,
    #[serde(rename = "turnDirection")]
    pub aixm_turn_direction: Option<String>,
    #[serde(rename = "signalGap")]
    pub aixm_signal_gap: Option<String>,
    #[serde(rename = "minimumEnrouteAltitude")]
    pub aixm_minimum_enroute_altitude: Option<String>,
    #[serde(rename = "minimumCrossingAtEnd")]
    pub aixm_minimum_crossing_at_end: Option<String>,
    #[serde(rename = "minimumCrossingAtEndReference")]
    pub aixm_minimum_crossing_at_end_reference: Option<String>,
    #[serde(rename = "maximumCrossingAtEnd")]
    pub aixm_maximum_crossing_at_end: Option<String>,
    #[serde(rename = "maximumCrossingAtEndReference")]
    pub aixm_maximum_crossing_at_end_reference: Option<String>,
    #[serde(rename = "navigationType")]
    pub aixm_navigation_type: String,
    #[serde(rename = "requiredNavigationPerformance")]
    pub aixm_required_navigation_performance: Option<String>,
    #[serde(rename = "designatorSuffix")]
    pub aixm_designator_suffix: Option<String>,
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
    #[serde(rename = "annotation", default)]
    pub aixm_annotation: Vec<AixmAnnotation>,
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
    pub aixm_unit_of_measurement: Option<String>,
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
    pub aixm_standard_icao: Option<String>,
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
    pub aixm_nominal_length: Option<AixmNominalLength>,
    #[serde(rename = "lengthAccuracy")]
    pub aixm_length_accuracy: Option<String>,
    #[serde(rename = "nominalWidth")]
    pub aixm_nominal_width: Option<AixmNominalWidth>,
    #[serde(rename = "widthAccuracy")]
    pub aixm_width_accuracy: Option<String>,
    #[serde(rename = "widthShoulder")]
    pub aixm_width_shoulder: Option<String>,
    #[serde(rename = "lengthStrip")]
    pub aixm_length_strip: Option<AixmLengthStrip>,
    #[serde(rename = "widthStrip")]
    pub aixm_width_strip: Option<AixmWidthStrip>,
    #[serde(rename = "lengthOffset")]
    pub aixm_length_offset: Option<String>,
    #[serde(rename = "widthOffset")]
    pub aixm_width_offset: Option<String>,
    #[serde(rename = "abandoned")]
    pub aixm_abandoned: Option<String>,
    #[serde(rename = "surfaceProperties")]
    pub aixm_surface_properties: AixmSurfaceProperties,
    #[serde(rename = "associatedAirportHeliport")]
    pub aixm_associated_airport_heliport: AixmAirportHeliportLink,
    #[serde(rename = "annotation", default)]
    pub aixm_annotation: Vec<AixmAnnotation>,
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
    pub aixm_designator: Option<String>,
    #[serde(rename = "location")]
    pub aixm_location: Option<AixmLocation>,
    #[serde(rename = "onRunway")]
    pub aixm_on_runway: AixmOnRunway,
    #[serde(rename = "annotation")]
    pub aixm_annotation: Option<AixmAnnotation>,
    #[serde(rename = "associatedDeclaredDistance", default)]
    pub aixm_associated_declared_distance: Vec<AixmAssociatedDeclaredDistance>,
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
    pub aixm_true_bearing: Option<f64>,
    #[serde(rename = "trueBearingAccuracy")]
    pub aixm_true_bearing_accuracy: Option<String>,
    #[serde(rename = "magneticBearing")]
    pub aixm_magnetic_bearing: Option<f64>,
    #[serde(rename = "patternVFR")]
    pub aixm_pattern_vfr: Option<String>,
    #[serde(rename = "slopeTDZ")]
    pub aixm_slope_tdz: Option<String>,
    #[serde(rename = "elevationTDZ")]
    pub aixm_elevation_tdz: Option<AixmElevationTdz>,
    #[serde(rename = "elevationTDZAccuracy")]
    pub aixm_elevation_tdzaccuracy: Option<String>,
    #[serde(rename = "approachMarkingType")]
    pub aixm_approach_marking_type: Option<String>,
    #[serde(rename = "approachMarkingCondition")]
    pub aixm_approach_marking_condition: Option<String>,
    #[serde(rename = "classLightingJAR")]
    pub aixm_class_lighting_jar: Option<String>,
    #[serde(rename = "precisionApproachGuidance")]
    pub aixm_precision_approach_guidance: Option<String>,
    #[serde(rename = "usedRunway")]
    pub aixm_used_runway: AixmUsedRunway,
    #[serde(rename = "annotation", default)]
    pub aixm_annotation: Vec<AixmAnnotation>,
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
    pub aixm_emergency_lighting: Option<String>,
    #[serde(rename = "intensityLevel")]
    pub aixm_intensity_level: Option<String>,
    #[serde(rename = "colour")]
    pub aixm_colour: Option<String>,
    #[serde(rename = "type")]
    pub aixm_type: String,
    #[serde(rename = "position")]
    pub aixm_position: Option<String>,
    #[serde(rename = "numberBox")]
    pub aixm_number_box: Option<String>,
    #[serde(rename = "portable")]
    pub aixm_portable: Option<String>,
    #[serde(rename = "slopeAngle")]
    pub aixm_slope_angle: Option<f64>,
    #[serde(rename = "minimumEyeHeightOverThreshold")]
    pub aixm_minimum_eye_height_over_threshold: Option<AixmMinimumEyeHeightOverThreshold>,
    #[serde(rename = "runwayDirection")]
    pub aixm_runway_direction: AixmRunwayDirectionLink,
}
