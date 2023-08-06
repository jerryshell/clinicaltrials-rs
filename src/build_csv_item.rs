use crate::*;
use anyhow::{anyhow, Result};

pub async fn build_csv_item(
    client: &reqwest::Client,
    config: &model::config::Config,
    hit: &model::search::Hit,
) -> Option<model::csv_item::CsvItem> {
    let mut add_to_result = false;

    let id = if let Some(id) = hit.id.as_deref() {
        id.trim().to_string()
    } else {
        return None;
    };
    tracing::info!("id: {}", id);
    if id.is_empty() {
        return None;
    }

    // study
    let study_root = match get_study_root_by_id(client, &id).await {
        Ok(study_root) => study_root,
        Err(e) => {
            tracing::error!("{}", e);
            return None;
        }
    };

    // protocol_section
    let protocol_section = match get_protocol_section_by_study_root(&study_root).await {
        Ok(study_root) => study_root,
        Err(e) => {
            tracing::error!("{}", e);
            return None;
        }
    };

    // eligibility_criteria
    let eligibility_criteria = get_eligibility_criteria_by_protocol_section(protocol_section).await;

    // inclusion_criteria, exclusion_criteria
    let (inclusion_criteria, exclusion_criteria) =
        get_inclusion_criteria_and_exclusion_criteria_by_eligibility_criteria(
            &eligibility_criteria,
        )
        .await;

    // inclusion_criteria filter
    if config.keywords_in_inclusion {
        add_to_result = add_to_result
            || is_some_text_include_keywords(inclusion_criteria, &config.keywords).await;
    }

    // exclusion_criteria filter
    if config.keywords_in_exclusion {
        add_to_result = add_to_result
            || is_some_text_include_keywords(exclusion_criteria, &config.keywords).await;
    }

    // conditions
    let conditions = get_conditions_by_protocol_section(protocol_section).await;

    // conditions filter
    if config.keywords_in_conditions {
        add_to_result =
            add_to_result || is_some_text_include_keywords(&conditions, &config.keywords).await;
    }

    tracing::info!("match: {}", add_to_result);
    if !add_to_result {
        return None;
    }

    let sponsor = get_sponsor_by_protocol_section(protocol_section).await;
    let start_date = get_start_date_by_protocol_section(protocol_section).await;
    let completion_date = get_completion_date_by_protocol_section(protocol_section).await;
    let status = get_status_by_protocol_section(protocol_section).await;
    let phase = get_phase_by_protocol_section(protocol_section).await;
    let drug = get_drug_by_protocol_section(protocol_section).await;

    let csv_item = model::csv_item::CsvItem {
        id,
        sponsor: sponsor.to_string(),
        start_date: start_date.to_string(),
        completion_date: completion_date.to_string(),
        status: status.to_string(),
        phase: phase.to_string(),
        conditions,
        drug,
    };

    Some(csv_item)
}

async fn get_study_root_by_id(client: &reqwest::Client, id: &str) -> Result<model::study::Root> {
    let study_url = format!("https://www.clinicaltrials.gov/api/int/studies/{}", id);
    tracing::info!("study_url: {}", study_url);

    let root = client
        .get(study_url)
        .send()
        .await?
        .json::<model::study::Root>()
        .await?;

    Ok(root)
}

async fn get_protocol_section_by_study_root(
    study_root: &model::study::Root,
) -> Result<&model::study::ProtocolSection> {
    study_root
        .study
        .as_ref()
        .map(|study| study.protocol_section.as_ref())
        .and_then(|protocol_section| protocol_section)
        .ok_or(anyhow!("no protocol_section"))
}

async fn get_eligibility_criteria_by_protocol_section(
    protocol_section: &model::study::ProtocolSection,
) -> String {
    protocol_section
        .eligibility_module
        .as_ref()
        .and_then(|eligibility_module| eligibility_module.eligibility_criteria.as_deref())
        .unwrap_or("")
        .to_lowercase()
}

async fn get_inclusion_criteria_and_exclusion_criteria_by_eligibility_criteria(
    eligibility_criteria: &str,
) -> (&str, &str) {
    let eligibility_criteria_split = eligibility_criteria
        .split("Exclusion Criteria:")
        .collect::<Vec<&str>>();

    if eligibility_criteria_split.len() == 2 {
        (eligibility_criteria_split[0], eligibility_criteria_split[1])
    } else {
        (eligibility_criteria, "")
    }
}

async fn is_some_text_include_keywords(some_text: &str, keywords: &[String]) -> bool {
    keywords.iter().any(|k| some_text.contains(k))
}

async fn get_conditions_by_protocol_section(
    protocol_section: &model::study::ProtocolSection,
) -> String {
    protocol_section
        .conditions_module
        .as_ref()
        .and_then(|x| x.conditions.as_ref())
        .map(|conditions| conditions.join(","))
        .unwrap_or("-".to_string())
}

async fn get_sponsor_by_protocol_section(protocol_section: &model::study::ProtocolSection) -> &str {
    protocol_section
        .sponsor_collaborators_module
        .as_ref()
        .and_then(|sponsor_collaborators_module| sponsor_collaborators_module.lead_sponsor.as_ref())
        .and_then(|lead_sponsor| lead_sponsor.name.as_ref())
        .map(|name| name.as_str())
        .unwrap_or("-")
}

async fn get_start_date_by_protocol_section(
    protocol_section: &model::study::ProtocolSection,
) -> &str {
    protocol_section
        .status_module
        .as_ref()
        .and_then(|status_module| status_module.start_date_struct.as_ref())
        .and_then(|start_date_struct| start_date_struct.date.as_ref())
        .map(|date| date.as_str())
        .unwrap_or("-")
}

async fn get_drug_by_protocol_section(protocol_section: &model::study::ProtocolSection) -> String {
    protocol_section
        .arms_interventions_module
        .as_ref()
        .and_then(|arms_interventions_module| arms_interventions_module.interventions.as_ref())
        .map(|interventions| {
            interventions
                .iter()
                .filter_map(|item| {
                    if item.intervention_type.as_deref() == Some("DRUG") {
                        item.name.as_deref()
                    } else {
                        None
                    }
                })
                .filter(|s| !s.trim().is_empty())
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or("".to_string())
}

async fn get_completion_date_by_protocol_section(
    protocol_section: &model::study::ProtocolSection,
) -> &str {
    protocol_section
        .status_module
        .as_ref()
        .and_then(|status_module| status_module.completion_date_struct.as_ref())
        .and_then(|completion_date_struct| completion_date_struct.date.as_ref())
        .map(|date| date.as_str())
        .unwrap_or("-")
}

async fn get_status_by_protocol_section(protocol_section: &model::study::ProtocolSection) -> &str {
    protocol_section
        .status_module
        .as_ref()
        .and_then(|status_module| status_module.overall_status.as_ref())
        .map(|overall_status| overall_status.as_str())
        .unwrap_or("-")
}

async fn get_phase_by_protocol_section(protocol_section: &model::study::ProtocolSection) -> &str {
    protocol_section
        .design_module
        .as_ref()
        .and_then(|design_module| design_module.phases.as_ref())
        .map(|phases| {
            if phases.is_empty() {
                "-"
            } else {
                phases[0].as_ref()
            }
        })
        .unwrap_or("-")
}
