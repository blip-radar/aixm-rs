#![allow(clippy::struct_field_names, reason = "XML namespaces")]
use serde::{Deserialize, Serialize};

use crate::gml::{GmlTimeInstant, GmlTimePeriod};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdMetadata {
    #[serde(rename = "@schemaLocation")]
    pub xsi_schema_location: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "fileIdentifier")]
    pub gmd_file_identifier: GmdFileIdentifier,
    #[serde(rename = "language")]
    pub gmd_language: GmdMdMetadataGmdLanguage,
    #[serde(rename = "characterSet")]
    pub gmd_character_set: GmdCharacterSet,
    #[serde(rename = "hierarchyLevel")]
    pub gmd_hierarchy_level: GmdHierarchyLevel,
    #[serde(rename = "contact")]
    pub gmd_contact: Vec<GmdContact>,
    #[serde(rename = "dateStamp")]
    pub gmd_date_stamp: GmdDateStamp,
    #[serde(rename = "metadataStandardName")]
    pub gmd_metadata_standard_name: GmdMetadataStandardName,
    #[serde(rename = "dataSetURI")]
    pub gmd_data_set_uri: GmdDataSetUri,
    #[serde(rename = "referenceSystemInfo")]
    pub gmd_reference_system_info: Vec<GmdReferenceSystemInfo>,
    #[serde(rename = "identificationInfo")]
    pub gmd_identification_info: GmdIdentificationInfo,
    #[serde(rename = "distributionInfo")]
    pub gmd_distribution_info: GmdDistributionInfo,
    #[serde(rename = "dataQualityInfo")]
    pub gmd_data_quality_info: GmdDataQualityInfo,
    #[serde(rename = "metadataConstraints")]
    pub gmd_metadata_constraints: GmdMetadataConstraints,
    #[serde(rename = "applicationSchemaInfo")]
    pub gmd_application_schema_info: GmdApplicationSchemaInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdFileIdentifier {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdMetadataGmdLanguage {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCharacterSet {
    #[serde(rename = "MD_CharacterSetCode")]
    pub gmd_md_character_set_code: GmdMdCharacterSetCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdCharacterSetCode {
    #[serde(rename = "@codeList")]
    pub code_list: String,
    #[serde(rename = "@codeListValue")]
    pub code_list_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdHierarchyLevel {
    #[serde(rename = "MD_ScopeCode")]
    pub gmd_md_scope_code: GmdMdScopeCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdScopeCode {
    #[serde(rename = "@codeList")]
    pub code_list: String,
    #[serde(rename = "@codeListValue")]
    pub code_list_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdContact {
    #[serde(rename = "CI_ResponsibleParty")]
    pub gmd_ci_responsible_party: GmdCiResponsibleParty,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiResponsibleParty {
    #[serde(rename = "organisationName")]
    pub gmd_organisation_name: GmdOrganisationName,
    #[serde(rename = "contactInfo")]
    pub gmd_contact_info: GmdContactInfo,
    #[serde(rename = "role")]
    pub gmd_role: GmdRole,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdOrganisationName {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdContactInfo {
    #[serde(rename = "CI_Contact")]
    pub gmd_ci_contact: GmdCiContact,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiContact {
    #[serde(rename = "phone")]
    pub gmd_phone: GmdPhone,
    #[serde(rename = "address")]
    pub gmd_address: GmdAddress,
    #[serde(rename = "onlineResource")]
    pub gmd_online_resource: GmdOnlineResource,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdPhone {
    #[serde(rename = "CI_Telephone")]
    pub gmd_ci_telephone: GmdCiTelephone,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiTelephone {
    #[serde(rename = "voice")]
    pub gmd_voice: GmdVoice,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdVoice {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdAddress {
    #[serde(rename = "CI_Address")]
    pub gmd_ci_address: GmdCiAddress,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiAddress {
    #[serde(rename = "deliveryPoint")]
    pub gmd_delivery_point: GmdCiAddressGmdDeliveryPoint,
    #[serde(rename = "city")]
    pub gmd_city: GmdCiAddressGmdCity,
    #[serde(rename = "administrativeArea")]
    pub gmd_administrative_area: Option<GmdCiAddressGmdAdministrativeArea>,
    #[serde(rename = "postalCode")]
    pub gmd_postal_code: GmdCiAddressGmdPostalCode,
    #[serde(rename = "country")]
    pub gmd_country: GmdCiAddressGmdCountry,
    #[serde(rename = "electronicMailAddress")]
    pub gmd_electronic_mail_address: GmdCiAddressGmdElectronicMailAddress,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiAddressGmdDeliveryPoint {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiAddressGmdCity {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiAddressGmdAdministrativeArea {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiAddressGmdPostalCode {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiAddressGmdCountry {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiAddressGmdElectronicMailAddress {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdOnlineResource {
    #[serde(rename = "CI_OnlineResource")]
    pub gmd_ci_online_resource: GmdCiOnlineResource,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiOnlineResource {
    #[serde(rename = "linkage")]
    pub gmd_linkage: GmdCiOnlineResourceGmdLinkage,
    #[serde(rename = "protocol")]
    pub gmd_protocol: GmdCiOnlineResourceGmdProtocol,
    #[serde(rename = "name")]
    pub gmd_name: GmdCiOnlineResourceGmdName,
    #[serde(rename = "description")]
    pub gmd_description: GmdCiOnlineResourceGmdDescription,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiOnlineResourceGmdLinkage {
    #[serde(rename = "URL")]
    pub gmd_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiOnlineResourceGmdProtocol {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiOnlineResourceGmdName {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiOnlineResourceGmdDescription {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdRole {
    #[serde(rename = "CI_RoleCode")]
    pub gmd_ci_role_code: GmdCiRoleCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiRoleCode {
    #[serde(rename = "@codeList")]
    pub code_list: String,
    #[serde(rename = "@codeListValue")]
    pub code_list_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDateStamp {
    #[serde(rename = "DateTime")]
    pub gco_date_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMetadataStandardName {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDataSetUri {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdReferenceSystemInfo {
    #[serde(rename = "MD_ReferenceSystem")]
    pub gmd_md_reference_system: GmdMdReferenceSystem,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdReferenceSystem {
    #[serde(rename = "referenceSystemIdentifier")]
    pub gmd_reference_system_identifier: GmdReferenceSystemIdentifier,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdReferenceSystemIdentifier {
    #[serde(rename = "@href")]
    pub xlink_href: String,
    #[serde(rename = "@role")]
    pub xlink_role: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdIdentificationInfo {
    #[serde(rename = "MD_DataIdentification")]
    pub gmd_md_data_identification: GmdMdDataIdentification,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdDataIdentification {
    #[serde(rename = "citation")]
    pub gmd_citation: GmdCitation,
    #[serde(rename = "abstract")]
    pub gmd_abstract: GmdAbstract,
    #[serde(rename = "status")]
    pub gmd_status: GmdStatus,
    #[serde(rename = "pointOfContact")]
    pub gmd_point_of_contact: GmdPointOfContact,
    #[serde(rename = "resourceMaintenance")]
    pub gmd_resource_maintenance: GmdResourceMaintenance,
    #[serde(rename = "descriptiveKeywords")]
    pub gmd_descriptive_keywords: GmdDescriptiveKeywords,
    #[serde(rename = "resourceSpecificUsage")]
    pub gmd_resource_specific_usage: GmdResourceSpecificUsage,
    #[serde(rename = "resourceConstraints")]
    pub gmd_resource_constraints: Vec<GmdResourceConstraints>,
    #[serde(rename = "spatialRepresentationType")]
    pub gmd_spatial_representation_type: GmdSpatialRepresentationType,
    #[serde(rename = "spatialResolution")]
    pub gmd_spatial_resolution: GmdSpatialResolution,
    #[serde(rename = "language")]
    pub gmd_language: GmdMdDataIdentificationGmdLanguage,
    #[serde(rename = "characterSet")]
    pub gmd_character_set: GmdCharacterSet,
    #[serde(rename = "topicCategory")]
    pub gmd_topic_category: GmdTopicCategory,
    #[serde(rename = "extent")]
    pub gmd_extent: Option<GmdMdDataIdentificationGmdExtent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCitation {
    #[serde(rename = "CI_Citation")]
    pub gmd_ci_citation: GmdCiCitation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiCitation {
    #[serde(rename = "title")]
    pub gmd_title: GmdCiCitationGmdTitle,
    #[serde(rename = "date")]
    pub gmd_date: Vec<GmdCiCitationGmdDate>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiCitationGmdTitle {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiCitationGmdDate {
    #[serde(rename = "CI_Date")]
    pub gmd_ci_date: Option<GmdCiDate>,
    #[serde(rename = "@nilReason")]
    pub gco_nil_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiDate {
    #[serde(rename = "date")]
    pub gmd_date: GmdCiDateGmdDate,
    #[serde(rename = "dateType")]
    pub gmd_date_type: GmdCiDateGmdDateType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiDateGmdDate {
    #[serde(rename = "DateTime")]
    pub gco_date_time: Option<String>,
    #[serde(rename = "Date")]
    pub gco_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiDateGmdDateType {
    #[serde(rename = "CI_DateTypeCode")]
    pub gmd_ci_date_type_code: GmdCiDateTypeCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdCiDateTypeCode {
    #[serde(rename = "@codeList")]
    pub code_list: String,
    #[serde(rename = "@codeListValue")]
    pub code_list_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdAbstract {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdStatus {
    #[serde(rename = "MD_ProgressCode")]
    pub gmd_md_progress_code: GmdMdProgressCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdProgressCode {
    #[serde(rename = "@codeList")]
    pub code_list: String,
    #[serde(rename = "@codeListValue")]
    pub code_list_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdPointOfContact {
    #[serde(rename = "CI_ResponsibleParty")]
    pub gmd_ci_responsible_party: GmdCiResponsibleParty,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdResourceMaintenance {
    #[serde(rename = "MD_MaintenanceInformation")]
    pub gmd_md_maintenance_information: GmdMdMaintenanceInformation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdMaintenanceInformation {
    #[serde(rename = "maintenanceAndUpdateFrequency")]
    pub gmd_maintenance_and_update_frequency: GmdMaintenanceAndUpdateFrequency,
    #[serde(rename = "maintenanceNote")]
    pub gmd_maintenance_note: Option<GmdMaintenanceNote>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMaintenanceNote {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMaintenanceAndUpdateFrequency {
    #[serde(rename = "MD_MaintenanceFrequencyCode")]
    pub gmd_md_maintenance_frequency_code: GmdMdMaintenanceFrequencyCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdMaintenanceFrequencyCode {
    #[serde(rename = "@codeList")]
    pub code_list: String,
    #[serde(rename = "@codeListValue")]
    pub code_list_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDescriptiveKeywords {
    #[serde(rename = "MD_Keywords")]
    pub gmd_md_keywords: GmdMdKeywords,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdKeywords {
    #[serde(rename = "keyword")]
    pub gmd_keyword: Vec<GmdKeyword>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdKeyword {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdResourceSpecificUsage {
    #[serde(rename = "MD_Usage")]
    pub gmd_md_usage: GmdMdUsage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdUsage {
    #[serde(rename = "specificUsage")]
    pub gmd_specific_usage: GmdSpecificUsage,
    #[serde(rename = "userContactInfo")]
    pub gmd_user_contact_info: GmdUserContactInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdSpecificUsage {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdUserContactInfo {
    #[serde(rename = "CI_ResponsibleParty")]
    pub gmd_ci_responsible_party: GmdCiResponsibleParty,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdResourceConstraints {
    #[serde(rename = "MD_LegalConstraints")]
    pub gmd_md_legal_constraints: Option<GmdMdLegalConstraints>,
    #[serde(rename = "MD_SecurityConstraints")]
    pub gmd_md_security_constraints: Option<GmdMdSecurityConstraints>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdLegalConstraints {
    #[serde(rename = "accessConstraints")]
    pub gmd_access_constraints: GmdMdLegalConstraintsGmdAccessConstraints,
    #[serde(rename = "otherConstraints")]
    pub gmd_other_constraints: GmdMdLegalConstraintsGmdOtherConstraints,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdLegalConstraintsGmdAccessConstraints {
    #[serde(rename = "MD_RestrictionCode")]
    pub gmd_md_restriction_code: GmdMdRestrictionCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdRestrictionCode {
    #[serde(rename = "@codeList")]
    pub code_list: String,
    #[serde(rename = "@codeListValue")]
    pub code_list_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdLegalConstraintsGmdOtherConstraints {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdSecurityConstraints {
    #[serde(rename = "classification")]
    pub gmd_classification: GmdClassification,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdClassification {
    #[serde(rename = "MD_ClassificationCode")]
    pub gmd_md_classification_code: GmdMdClassificationCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdClassificationCode {
    #[serde(rename = "$text")]
    pub text: String,
    #[serde(rename = "@codeList")]
    pub code_list: String,
    #[serde(rename = "@codeListValue")]
    pub code_list_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdSpatialRepresentationType {
    #[serde(rename = "MD_SpatialRepresentationTypeCode")]
    pub gmd_md_spatial_representation_type_code: GmdMdSpatialRepresentationTypeCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdSpatialRepresentationTypeCode {
    #[serde(rename = "@codeList")]
    pub code_list: String,
    #[serde(rename = "@codeListValue")]
    pub code_list_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdSpatialResolution {
    #[serde(rename = "@nilReason")]
    pub gco_nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdDataIdentificationGmdLanguage {
    #[serde(rename = "LanguageCode")]
    pub gmd_language_code: GmdLanguageCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdLanguageCode {
    #[serde(rename = "@codeList")]
    pub code_list: String,
    #[serde(rename = "@codeListValue")]
    pub code_list_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdTopicCategory {
    #[serde(rename = "MD_TopicCategoryCode")]
    pub gmd_md_topic_category_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdDataIdentificationGmdExtent {
    #[serde(rename = "EX_Extent")]
    pub gmd_ex_extent: GmdExExtent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdExExtent {
    #[serde(rename = "description")]
    pub gmd_description: GmdExExtentGmdDescription,
    #[serde(rename = "geographicElement")]
    pub gmd_geographic_element: Option<GmdGeographicElement>,
    #[serde(rename = "temporalElement")]
    pub gmd_temporal_element: GmdTemporalElement,
    #[serde(rename = "verticalElement")]
    pub gmd_vertical_element: Option<GmdVerticalElement>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdExExtentGmdDescription {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdGeographicElement {
    #[serde(rename = "EX_GeographicBoundingBox")]
    pub gmd_ex_geographic_bounding_box: GmdExGeographicBoundingBox,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdExGeographicBoundingBox {
    #[serde(rename = "westBoundLongitude")]
    pub gmd_west_bound_longitude: GmdWestBoundLongitude,
    #[serde(rename = "eastBoundLongitude")]
    pub gmd_east_bound_longitude: GmdEastBoundLongitude,
    #[serde(rename = "southBoundLatitude")]
    pub gmd_south_bound_latitude: GmdSouthBoundLatitude,
    #[serde(rename = "northBoundLatitude")]
    pub gmd_north_bound_latitude: GmdNorthBoundLatitude,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdWestBoundLongitude {
    #[serde(rename = "Decimal")]
    pub gco_decimal: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdEastBoundLongitude {
    #[serde(rename = "Decimal")]
    pub gco_decimal: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdSouthBoundLatitude {
    #[serde(rename = "Decimal")]
    pub gco_decimal: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdNorthBoundLatitude {
    #[serde(rename = "Decimal")]
    pub gco_decimal: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdTemporalElement {
    #[serde(rename = "EX_TemporalExtent")]
    pub gmd_ex_temporal_extent: GmdExTemporalExtent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdExTemporalExtent {
    #[serde(rename = "extent")]
    pub gmd_extent: GmdExTemporalExtentGmdExtent,
}

// FIXME: enum?
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdExTemporalExtentGmdExtent {
    #[serde(rename = "TimePeriod")]
    pub gml_time_period: Option<GmlTimePeriod>,
    #[serde(rename = "TimeInstant")]
    pub gml_time_instant: Option<GmlTimeInstant>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdVerticalElement {
    #[serde(rename = "@nilReason")]
    pub gco_nil_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDistributionInfo {
    #[serde(rename = "MD_Distribution")]
    pub gmd_md_distribution: GmdMdDistribution,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdDistribution {
    #[serde(rename = "distributionFormat")]
    pub gmd_distribution_format: GmdDistributionFormat,
    #[serde(rename = "transferOptions")]
    pub gmd_transfer_options: GmdTransferOptions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDistributionFormat {
    #[serde(rename = "MD_Format")]
    pub gmd_md_format: GmdMdFormat,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdFormat {
    #[serde(rename = "name")]
pub     gmd_name: MessageAixmbasicMessageAixmMessageMetadataGmdMdMetadataGmdDistributionInfoGmdMdDistributionGmdDistributionFormatGmdMdFormatGmdName,
    #[serde(rename = "version")]
pub     gmd_version: GmdVersion,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageAixmbasicMessageAixmMessageMetadataGmdMdMetadataGmdDistributionInfoGmdMdDistributionGmdDistributionFormatGmdMdFormatGmdName
{
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdVersion {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdTransferOptions {
    #[serde(rename = "MD_DigitalTransferOptions")]
    pub gmd_md_digital_transfer_options: GmdMdDigitalTransferOptions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdDigitalTransferOptions {
    #[serde(rename = "onLine")]
    pub gmd_on_line: GmdOnLine,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdOnLine {
    #[serde(rename = "CI_OnlineResource")]
    pub gmd_ci_online_resource: GmdCiOnlineResource,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDataQualityInfo {
    #[serde(rename = "DQ_DataQuality")]
    pub gmd_dq_data_quality: GmdDqDataQuality,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqDataQuality {
    #[serde(rename = "scope")]
    pub gmd_scope: GmdScope,
    #[serde(rename = "report")]
    pub gmd_report: Vec<GmdReport>,
    #[serde(rename = "lineage")]
    pub gmd_lineage: Option<GmdLineage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdScope {
    #[serde(rename = "DQ_Scope")]
    pub gmd_dq_scope: GmdDqScope,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqScope {
    #[serde(rename = "level")]
    pub gmd_level: GmdLevel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdLevel {
    #[serde(rename = "MD_ScopeCode")]
    pub gmd_md_scope_code: GmdMdScopeCode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdReport {
    #[serde(rename = "@type")]
    pub xlink_type: Option<String>,
    #[serde(rename = "DQ_DomainConsistency")]
    pub gmd_dq_domain_consistency: Option<GmdDqDomainConsistency>,
    #[serde(rename = "DQ_AbsoluteExternalPositionalAccuracy")]
    pub gmd_dq_absolute_external_positional_accuracy:
        Option<GmdDqAbsoluteExternalPositionalAccuracy>,
    #[serde(rename = "DQ_ConceptualConsistency")]
    pub gmd_dq_conceptual_consistency: Option<GmdDqConceptualConsistency>,
    #[serde(rename = "DQ_QuantitativeAttributeAccuracy")]
    pub gmd_dq_quantitative_attribute_accuracy: Option<GmdDqQuantitativeAttributeAccuracy>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqDomainConsistency {
    #[serde(rename = "nameOfMeasure")]
    pub gmd_name_of_measure: GmdNameOfMeasure,
    #[serde(rename = "result")]
    pub gmd_result: GmdDqDomainConsistencyGmdResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdNameOfMeasure {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqDomainConsistencyGmdResult {
    #[serde(rename = "DQ_ConformanceResult")]
    pub gmd_dq_conformance_result: GmdDqConformanceResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqConformanceResult {
    #[serde(rename = "specification")]
    pub gmd_specification: GmdDqConformanceResultGmdSpecification,
    #[serde(rename = "explanation")]
    pub gmd_explanation: GmdDqConformanceResultGmdExplanation,
    #[serde(rename = "pass")]
    pub gmd_pass: GmdDqConformanceResultGmdPass,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqConformanceResultGmdSpecification {
    #[serde(rename = "CI_Citation")]
    pub gmd_ci_citation: Option<GmdCiCitation>,
    #[serde(rename = "@nilReason")]
    pub gco_nil_reason: Option<String>,
    #[serde(rename = "@type")]
    pub xlink_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqConformanceResultGmdExplanation {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqConformanceResultGmdPass {
    #[serde(rename = "Boolean")]
    pub gco_boolean: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqAbsoluteExternalPositionalAccuracy {
    #[serde(rename = "nameOfMeasure")]
    pub gmd_name_of_measure: GmdNameOfMeasure,
    #[serde(rename = "result")]
    pub gmd_result: GmdDqAbsoluteExternalPositionalAccuracyGmdResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqAbsoluteExternalPositionalAccuracyGmdResult {
    #[serde(rename = "DQ_ConformanceResult")]
    pub gmd_dq_conformance_result: GmdDqConformanceResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqConceptualConsistency {
    #[serde(rename = "nameOfMeasure")]
    pub gmd_name_of_measure: GmdNameOfMeasure,
    #[serde(rename = "result")]
    pub gmd_result: GmdDqConceptualConsistencyGmdResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqConceptualConsistencyGmdResult {
    #[serde(rename = "@nilReason")]
    pub gco_nil_reason: Option<String>,
    #[serde(rename = "DQ_ConformanceResult")]
    pub gmd_dq_conformance_result: Option<GmdDqConformanceResult>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqQuantitativeAttributeAccuracy {
    #[serde(rename = "result")]
    pub gmd_result: GmdDqQuantitativeAttributeAccuracyGmdResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdDqQuantitativeAttributeAccuracyGmdResult {
    #[serde(rename = "@type")]
    pub xlink_type: String,
    #[serde(rename = "DQ_ConformanceResult")]
    pub gmd_dq_conformance_result: GmdDqConformanceResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdLineage {
    #[serde(rename = "LI_Lineage")]
    pub gmd_li_lineage: GmdLiLineage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdLiLineage {
    #[serde(rename = "statement")]
    pub gmd_statement: GmdStatement,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdStatement {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMetadataConstraints {
    #[serde(rename = "MD_LegalConstraints")]
    pub gmd_md_legal_constraints: GmdMdLegalConstraints,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdApplicationSchemaInfo {
    #[serde(rename = "MD_ApplicationSchemaInformation")]
    pub gmd_md_application_schema_information: GmdMdApplicationSchemaInformation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdApplicationSchemaInformation {
    #[serde(rename = "name")]
    pub gmd_name: GmdMdApplicationSchemaInformationGmdName,
    #[serde(rename = "schemaLanguage")]
    pub gmd_schema_language: GmdSchemaLanguage,
    #[serde(rename = "constraintLanguage")]
    pub gmd_constraint_language: GmdConstraintLanguage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdMdApplicationSchemaInformationGmdName {
    #[serde(rename = "CI_Citation")]
    pub gmd_ci_citation: GmdCiCitation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdSchemaLanguage {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmdConstraintLanguage {
    #[serde(rename = "CharacterString")]
    pub gco_character_string: String,
}
