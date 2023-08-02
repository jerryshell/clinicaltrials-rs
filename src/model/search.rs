use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub from: Option<i64>,
    pub limit: Option<i64>,
    pub total: Option<i64>,
    pub hits: Option<Vec<Hit>>,
    pub aggs: Option<Aggs>,
    pub agg_filters: Option<Vec<AggFilter>>,
    pub signals: Option<Signals>,
    pub profile_results: Option<ProfileResults>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggFilter {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub agg_filter_type: Option<String>,
    pub options: Option<Vec<OptionItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionItem {
    pub key: Option<String>,
    pub count: Option<i64>,
    pub checked: Option<bool>,
    pub parent_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Aggs {
    pub synonyms: Option<Synonyms>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Synonyms {
    pub condition_search: Option<ConditionSearch>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionSearch {
    pub castration: Option<Vec<Cancer>>,
    #[serde(rename = "castration resistant prostate cancer")]
    pub castration_resistant_prostate_cancer: Option<Vec<Cancer>>,
    pub resistant: Option<Vec<Cancer>>,
    pub prostate: Option<Vec<Cancer>>,
    #[serde(rename = "prostate cancer")]
    pub prostate_cancer: Option<Vec<Cancer>>,
    pub cancer: Option<Vec<Cancer>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cancer {
    pub id: Option<i64>,
    pub norm: Option<String>,
    pub orig: Option<String>,
    pub count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub id: Option<String>,
    pub study: Option<Study>,
    pub columns: Option<Columns>,
    pub is_new: Option<bool>,
}

impl std::cmp::Eq for Hit {}

impl std::hash::Hash for Hit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Columns {
    pub interventions: Option<Interventions>,
    pub collaborators: Option<Collaborators>,
    pub conditions: Option<Collaborators>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collaborators {
    pub total: Option<i64>,
    pub items: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interventions {
    pub total: Option<i64>,
    pub items: Option<Vec<Item>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Study {
    pub protocol_section: Option<ProtocolSection>,
    pub has_results: Option<bool>,
    pub document_section: Option<DocumentSection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSection {
    pub large_document_module: Option<LargeDocumentModule>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargeDocumentModule {
    pub large_docs: Option<Vec<LargeDoc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LargeDoc {
    pub label: Option<String>,
    pub filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolSection {
    pub identification_module: Option<IdentificationModule>,
    pub status_module: Option<StatusModule>,
    pub sponsor_collaborators_module: Option<SponsorCollaboratorsModule>,
    pub conditions_module: Option<ConditionsModule>,
    pub design_module: Option<DesignModule>,
    pub arms_interventions_module: Option<ArmsInterventionsModule>,
    pub outcomes_module: Option<OutcomesModule>,
    pub eligibility_module: Option<EligibilityModule>,
    pub contacts_locations_module: Option<ContactsLocationsModule>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArmsInterventionsModule {
    pub interventions: Option<Vec<Item>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionsModule {
    pub conditions: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactsLocationsModule {
    pub central_contacts: Option<Vec<Contact>>,
    pub locations: Option<Vec<Location>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub name: Option<String>,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub phone_ext: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub facility: Option<String>,
    pub status: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
    pub contacts: Option<Vec<Contact>>,
    pub geo_point: Option<GeoPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoPoint {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignModule {
    pub study_type: Option<String>,
    pub phases: Option<Vec<String>>,
    pub design_info: Option<DesignInfo>,
    pub enrollment_info: Option<EnrollmentInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignInfo {
    pub allocation: Option<String>,
    pub intervention_model: Option<String>,
    pub primary_purpose: Option<String>,
    pub masking_info: Option<MaskingInfo>,
    pub observational_model: Option<String>,
    pub time_perspective: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaskingInfo {
    pub masking: Option<String>,
    pub who_masked: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentInfo {
    pub count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EligibilityModule {
    pub sex: Option<String>,
    pub minimum_age: Option<String>,
    pub std_ages: Option<Vec<String>>,
    pub maximum_age: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationModule {
    pub nct_id: Option<String>,
    pub org_study_id_info: Option<YIdInfo>,
    pub secondary_id_infos: Option<Vec<YIdInfo>>,
    pub brief_title: Option<String>,
    pub acronym: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YIdInfo {
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutcomesModule {
    pub primary_outcomes: Option<Vec<AryOutcome>>,
    pub secondary_outcomes: Option<Vec<AryOutcome>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AryOutcome {
    pub measure: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsorCollaboratorsModule {
    pub lead_sponsor: Option<LeadSponsor>,
    pub collaborators: Option<Vec<Collaborator>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collaborator {
    pub class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeadSponsor {
    pub name: Option<String>,
    pub class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusModule {
    pub overall_status: Option<String>,
    pub start_date_struct: Option<DateStruct>,
    pub primary_completion_date_struct: Option<DateStruct>,
    pub completion_date_struct: Option<DateStruct>,
    pub study_first_submit_date: Option<String>,
    pub study_first_post_date_struct: Option<DateStruct>,
    pub last_update_post_date_struct: Option<DateStruct>,
    pub results_first_post_date_struct: Option<DateStruct>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateStruct {
    pub date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResults {
    pub pre_process_time: Option<String>,
    pub elastic_request_time: Option<String>,
    pub elastic_took_time: Option<String>,
    pub post_process_time: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signals {}
