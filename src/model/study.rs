use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub study: Option<Study>,
    // pub topics: Option<Topics>,
    // pub history: Option<History>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub changes: Option<Vec<Change>>,
    pub original_data: Option<OriginalData>,
    pub last_update_versions: Option<LastUpdateVersions>,
    pub outcomes_update_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub version: Option<i64>,
    pub date: Option<String>,
    pub status: Option<String>,
    pub module_labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastUpdateVersions {
    pub outcomes: Option<i64>,
    pub primary_outcomes: Option<i64>,
    pub secondary_outcomes: Option<i64>,
    pub enrollment_info: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalData {
    pub primary_outcomes: Option<Vec<AryOutcome>>,
    pub secondary_outcomes_same: Option<bool>,
    pub enrollment_info: Option<EnrollmentInfo>,
    pub org_full_name_same: Option<bool>,
    pub responsible_party_same: Option<bool>,
    pub lead_sponsor_same: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentInfo {
    pub count: Option<i64>,
    #[serde(rename = "type")]
    pub enrollment_info_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AryOutcome {
    pub measure: Option<String>,
    pub description: Option<String>,
    pub time_frame: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Study {
    pub protocol_section: Option<ProtocolSection>,
    // pub derived_section: Option<DerivedSection>,
    // pub has_results: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivedSection {
    pub misc_info_module: Option<MiscInfoModule>,
    pub condition_browse_module: Option<ConditionBrowseModule>,
    pub intervention_browse_module: Option<InterventionBrowseModule>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionBrowseModule {
    pub meshes: Option<Vec<Ancestor>>,
    pub ancestors: Option<Vec<Ancestor>>,
    pub browse_leaves: Option<Vec<BrowseLeaf>>,
    pub browse_branches: Option<Vec<BrowseBranch>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ancestor {
    pub id: Option<String>,
    pub term: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrowseBranch {
    pub abbrev: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseLeaf {
    pub id: Option<String>,
    pub name: Option<String>,
    pub as_found: Option<String>,
    pub relevance: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterventionBrowseModule {
    pub browse_leaves: Option<Vec<BrowseLeaf>>,
    pub browse_branches: Option<Vec<BrowseBranch>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiscInfoModule {
    pub version_holder: Option<String>,
    pub model_predictions: Option<ModelPredictions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelPredictions {
    // pub bmi_limits: Option<HashMap<String, i64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolSection {
    // pub identification_module: Option<IdentificationModule>,
    pub status_module: Option<StatusModule>,
    pub sponsor_collaborators_module: Option<SponsorCollaboratorsModule>,
    // pub oversight_module: Option<OversightModule>,
    // pub description_module: Option<DescriptionModule>,
    pub conditions_module: Option<ConditionsModule>,
    pub design_module: Option<DesignModule>,
    pub arms_interventions_module: Option<ArmsInterventionsModule>,
    // pub outcomes_module: Option<OutcomesModule>,
    pub eligibility_module: Option<EligibilityModule>,
    // pub contacts_locations_module: Option<ContactsLocationsModule>,
    // pub ipd_sharing_statement_module: Option<IpdSharingStatementModule>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmsInterventionsModule {
    pub arm_groups: Option<Vec<ArmGroup>>,
    pub interventions: Option<Vec<Intervention>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmGroup {
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub arm_group_type: Option<String>,
    pub description: Option<String>,
    pub intervention_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Intervention {
    #[serde(rename = "type")]
    pub intervention_type: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub arm_group_labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionsModule {
    pub conditions: Option<Vec<String>>,
    // pub keywords: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactsLocationsModule {
    pub overall_officials: Option<Vec<OverallOfficial>>,
    pub locations: Option<Vec<Location>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub facility: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
    pub geo_point: Option<GeoPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoPoint {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverallOfficial {
    pub name: Option<String>,
    pub affiliation: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionModule {
    pub brief_summary: Option<String>,
    pub detailed_description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignModule {
    // pub study_type: Option<String>,
    pub phases: Option<Vec<String>>,
    // pub design_info: Option<DesignInfo>,
    // pub enrollment_info: Option<EnrollmentInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignInfo {
    pub allocation: Option<String>,
    pub intervention_model: Option<String>,
    pub intervention_model_description: Option<String>,
    pub primary_purpose: Option<String>,
    pub masking_info: Option<MaskingInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaskingInfo {
    pub masking: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EligibilityModule {
    pub eligibility_criteria: Option<String>,
    pub healthy_volunteers: Option<bool>,
    pub sex: Option<String>,
    pub gender_based: Option<bool>,
    pub minimum_age: Option<String>,
    pub std_ages: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationModule {
    pub nct_id: Option<String>,
    pub org_study_id_info: Option<OrgStudyIdInfo>,
    pub organization: Option<Organization>,
    pub brief_title: Option<String>,
    pub official_title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgStudyIdInfo {
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub full_name: Option<String>,
    pub class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpdSharingStatementModule {
    pub ipd_sharing: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutcomesModule {
    pub primary_outcomes: Option<Vec<AryOutcome>>,
    pub secondary_outcomes: Option<Vec<AryOutcome>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OversightModule {
    pub oversight_has_dmc: Option<bool>,
    pub is_fda_regulated_drug: Option<bool>,
    pub is_fda_regulated_device: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsorCollaboratorsModule {
    pub responsible_party: Option<ResponsibleParty>,
    pub lead_sponsor: Option<LeadSponsor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeadSponsor {
    pub name: Option<String>,
    pub class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsibleParty {
    #[serde(rename = "type")]
    pub responsible_party_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusModule {
    pub status_verified_date: Option<String>,
    pub overall_status: Option<String>,
    pub expanded_access_info: Option<ExpandedAccessInfo>,
    pub start_date_struct: Option<DateStruct>,
    pub primary_completion_date_struct: Option<DateStruct>,
    pub completion_date_struct: Option<DateStruct>,
    pub study_first_submit_date: Option<String>,
    pub study_first_submit_qc_date: Option<String>,
    pub study_first_post_date_struct: Option<DateStruct>,
    pub last_update_submit_date: Option<String>,
    pub last_update_post_date_struct: Option<DateStruct>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateStruct {
    pub date: Option<String>,
    #[serde(rename = "type")]
    pub date_struct_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandedAccessInfo {
    pub has_expanded_access: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Topics {
    #[serde(rename = "GHR")]
    pub ghr: Option<Vec<Ghr>>,
    pub medline_plus: Option<Vec<Ghr>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ghr {
    pub name: Option<String>,
    pub url: Option<String>,
}
