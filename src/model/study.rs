use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub study: Study,
    pub topics: Topics,
    pub history: History,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Study {
    pub protocol_section: ProtocolSection,
    pub derived_section: DerivedSection,
    pub has_results: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolSection {
    pub identification_module: IdentificationModule,
    pub status_module: StatusModule,
    pub sponsor_collaborators_module: SponsorCollaboratorsModule,
    pub oversight_module: OversightModule,
    pub description_module: DescriptionModule,
    pub conditions_module: ConditionsModule,
    pub design_module: DesignModule,
    pub arms_interventions_module: ArmsInterventionsModule,
    pub outcomes_module: OutcomesModule,
    pub eligibility_module: EligibilityModule,
    pub contacts_locations_module: ContactsLocationsModule,
    pub ipd_sharing_statement_module: IpdSharingStatementModule,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationModule {
    pub nct_id: String,
    pub org_study_id_info: OrgStudyIdInfo,
    pub organization: Organization,
    pub brief_title: String,
    pub official_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgStudyIdInfo {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub full_name: String,
    pub class: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusModule {
    pub status_verified_date: String,
    pub overall_status: String,
    pub expanded_access_info: ExpandedAccessInfo,
    pub start_date_struct: StartDateStruct,
    pub primary_completion_date_struct: PrimaryCompletionDateStruct,
    pub completion_date_struct: CompletionDateStruct,
    pub study_first_submit_date: String,
    pub study_first_submit_qc_date: String,
    pub study_first_post_date_struct: StudyFirstPostDateStruct,
    pub last_update_submit_date: String,
    pub last_update_post_date_struct: LastUpdatePostDateStruct,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandedAccessInfo {
    pub has_expanded_access: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartDateStruct {
    pub date: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryCompletionDateStruct {
    pub date: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionDateStruct {
    pub date: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyFirstPostDateStruct {
    pub date: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastUpdatePostDateStruct {
    pub date: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsorCollaboratorsModule {
    pub responsible_party: ResponsibleParty,
    pub lead_sponsor: LeadSponsor,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsibleParty {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadSponsor {
    pub name: String,
    pub class: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OversightModule {
    pub oversight_has_dmc: bool,
    pub is_fda_regulated_drug: bool,
    pub is_fda_regulated_device: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionModule {
    pub brief_summary: String,
    pub detailed_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionsModule {
    pub conditions: Vec<String>,
    pub keywords: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignModule {
    pub study_type: String,
    pub phases: Vec<String>,
    pub design_info: DesignInfo,
    pub enrollment_info: EnrollmentInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignInfo {
    pub allocation: String,
    pub intervention_model: String,
    pub intervention_model_description: String,
    pub primary_purpose: String,
    pub masking_info: MaskingInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaskingInfo {
    pub masking: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentInfo {
    pub count: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmsInterventionsModule {
    pub arm_groups: Vec<ArmGroup>,
    pub interventions: Vec<Intervention>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmGroup {
    pub label: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
    pub intervention_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Intervention {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub description: String,
    pub arm_group_labels: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutcomesModule {
    pub primary_outcomes: Vec<PrimaryOutcome>,
    pub secondary_outcomes: Vec<SecondaryOutcome>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryOutcome {
    pub measure: String,
    pub description: String,
    pub time_frame: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryOutcome {
    pub measure: String,
    pub description: String,
    pub time_frame: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EligibilityModule {
    pub eligibility_criteria: String,
    pub healthy_volunteers: bool,
    pub sex: String,
    pub gender_based: bool,
    pub minimum_age: String,
    pub std_ages: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactsLocationsModule {
    pub overall_officials: Vec<OverallOfficial>,
    pub locations: Vec<Location>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverallOfficial {
    pub name: String,
    pub affiliation: String,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub facility: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
    pub geo_point: GeoPoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpdSharingStatementModule {
    pub ipd_sharing: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivedSection {
    pub misc_info_module: MiscInfoModule,
    pub condition_browse_module: ConditionBrowseModule,
    pub intervention_browse_module: InterventionBrowseModule,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiscInfoModule {
    pub version_holder: String,
    pub model_predictions: ModelPredictions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelPredictions {
    pub bmi_limits: BmiLimits,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BmiLimits {
    pub min_bmi: f64,
    pub max_bmi: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionBrowseModule {
    pub meshes: Vec<Mesh>,
    pub ancestors: Vec<Ancestor>,
    pub browse_leaves: Vec<BrowseLefe>,
    pub browse_branches: Vec<BrowseBranch>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mesh {
    pub id: String,
    pub term: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ancestor {
    pub id: String,
    pub term: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseLefe {
    pub id: String,
    pub name: String,
    pub as_found: Option<String>,
    pub relevance: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseBranch {
    pub abbrev: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterventionBrowseModule {
    pub browse_leaves: Vec<BrowseLefe2>,
    pub browse_branches: Vec<BrowseBranch2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseLefe2 {
    pub id: String,
    pub name: String,
    pub relevance: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseBranch2 {
    pub abbrev: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topics {
    #[serde(rename = "GHR")]
    pub ghr: Vec<Ghr>,
    #[serde(rename = "MedlinePlus")]
    pub medline_plus: Vec<MedlinePlu>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ghr {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedlinePlu {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub changes: Vec<Change>,
    pub original_data: OriginalData,
    pub last_update_versions: LastUpdateVersions,
    pub outcomes_update_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub version: i64,
    pub date: String,
    pub status: String,
    pub module_labels: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalData {
    pub primary_outcomes: Vec<PrimaryOutcome2>,
    pub secondary_outcomes_same: bool,
    pub enrollment_info: EnrollmentInfo2,
    pub org_full_name_same: bool,
    pub responsible_party_same: bool,
    pub lead_sponsor_same: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryOutcome2 {
    pub measure: String,
    pub description: String,
    pub time_frame: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentInfo2 {
    pub count: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastUpdateVersions {
    pub outcomes: i64,
    pub primary_outcomes: i64,
    pub secondary_outcomes: i64,
    pub enrollment_info: i64,
}
