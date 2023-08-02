use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub from: i64,
    pub limit: i64,
    pub total: i64,
    pub hits: Vec<Hit>,
    pub aggs: Aggs,
    pub agg_filters: Vec<AggFilter>,
    pub signals: Signals,
    pub profile_results: ProfileResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub id: String,
    // pub study: Study,
    // pub columns: Columns,
    // pub is_new: bool,
}

impl std::cmp::Eq for Hit {}

impl std::hash::Hash for Hit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Study {
    pub protocol_section: ProtocolSection,
    pub has_results: bool,
    pub document_section: Option<DocumentSection>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolSection {
    // pub identification_module: IdentificationModule,
    // pub status_module: StatusModule,
    // pub sponsor_collaborators_module: SponsorCollaboratorsModule,
    // pub conditions_module: ConditionsModule,
    // pub design_module: DesignModule,
    // pub arms_interventions_module: Option<ArmsInterventionsModule>,
    // pub outcomes_module: Option<OutcomesModule>,
    // pub eligibility_module: EligibilityModule,
    // pub contacts_locations_module: Option<ContactsLocationsModule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationModule {
    pub nct_id: String,
    pub org_study_id_info: OrgStudyIdInfo,
    pub brief_title: String,
    #[serde(default)]
    pub secondary_id_infos: Vec<SecondaryIdInfo>,
    pub acronym: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgStudyIdInfo {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryIdInfo {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusModule {
    pub overall_status: String,
    pub start_date_struct: Option<StartDateStruct>,
    pub primary_completion_date_struct: Option<PrimaryCompletionDateStruct>,
    pub completion_date_struct: Option<CompletionDateStruct>,
    pub study_first_submit_date: String,
    pub study_first_post_date_struct: StudyFirstPostDateStruct,
    pub last_update_post_date_struct: LastUpdatePostDateStruct,
    pub results_first_post_date_struct: Option<ResultsFirstPostDateStruct>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartDateStruct {
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryCompletionDateStruct {
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionDateStruct {
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyFirstPostDateStruct {
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastUpdatePostDateStruct {
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsFirstPostDateStruct {
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsorCollaboratorsModule {
    pub lead_sponsor: LeadSponsor,
    #[serde(default)]
    pub collaborators: Vec<Collaborator>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadSponsor {
    pub name: Option<String>,
    pub class: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collaborator {
    pub class: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionsModule {
    pub conditions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignModule {
    pub study_type: String,
    #[serde(default)]
    pub phases: Vec<String>,
    pub design_info: Option<DesignInfo>,
    pub enrollment_info: Option<EnrollmentInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignInfo {
    pub allocation: Option<String>,
    pub intervention_model: Option<String>,
    pub primary_purpose: Option<String>,
    pub masking_info: Option<MaskingInfo>,
    pub observational_model: Option<String>,
    pub time_perspective: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaskingInfo {
    pub masking: String,
    #[serde(default)]
    pub who_masked: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentInfo {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmsInterventionsModule {
    #[serde(default)]
    pub interventions: Vec<Intervention>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Intervention {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutcomesModule {
    pub primary_outcomes: Vec<PrimaryOutcome>,
    #[serde(default)]
    pub secondary_outcomes: Vec<SecondaryOutcome>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryOutcome {
    pub measure: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryOutcome {
    pub measure: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EligibilityModule {
    pub sex: String,
    pub minimum_age: Option<String>,
    pub std_ages: Vec<String>,
    pub maximum_age: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactsLocationsModule {
    #[serde(default)]
    pub locations: Vec<Location>,
    #[serde(default)]
    pub central_contacts: Vec<CentralContact>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub facility: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: String,
    #[serde(default)]
    pub contacts: Vec<Contact>,
    pub geo_point: Option<GeoPoint>,
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub name: Option<String>,
    pub role: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub phone_ext: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CentralContact {
    pub name: Option<String>,
    pub role: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub phone_ext: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSection {
    pub large_document_module: LargeDocumentModule,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargeDocumentModule {
    pub large_docs: Vec<LargeDoc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargeDoc {
    pub label: String,
    pub filename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Columns {
    pub interventions: Option<Interventions>,
    pub conditions: Conditions,
    pub collaborators: Option<Collaborators>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interventions {
    pub total: i64,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conditions {
    pub total: i64,
    pub items: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collaborators {
    pub total: i64,
    pub items: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggs {
    pub synonyms: Synonyms,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Synonyms {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggFilter {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub options: Vec<OptionItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionItem {
    pub key: String,
    pub count: i64,
    pub checked: bool,
    pub parent_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signals {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResults {
    pub pre_process_time: String,
    pub elastic_request_time: String,
    pub elastic_took_time: String,
    pub post_process_time: String,
}
