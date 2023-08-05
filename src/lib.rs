pub mod get_study_hits_by_query;
pub mod load_config;
pub mod model;
pub mod write_to_csv;

use anyhow::anyhow;
use anyhow::Result;
use get_study_hits_by_query::*;
use load_config::*;
use write_to_csv::*;

pub async fn run() -> Result<()> {
    // config
    let mut config = load_config().await?;
    println!("query: {:#?}", config);
    if config.query.is_empty() {
        return Err(anyhow!("query cannot be empty!"));
    }
    if config.keywords.is_empty() {
        return Err(anyhow!("keywords cannot be empty!"));
    }

    // config.keywords to_lowercase
    config
        .keywords
        .iter_mut()
        .for_each(|k| *k = k.to_lowercase());

    let client = reqwest::Client::builder()
        .user_agent("Chrome/96.0.4664.110")
        .build()
        .unwrap();

    let hits_set = get_study_hits_by_query(&client, &config.query).await?;

    let mut result = vec![];
    for hit in hits_set {
        let mut add_to_result = false;

        let id = if let Some(id) = hit.id {
            id.trim().to_string()
        } else {
            continue;
        };
        println!("id: {}", id);
        if id.is_empty() {
            continue;
        }

        // study
        let study_url = format!("https://www.clinicaltrials.gov/api/int/studies/{}", id);
        println!("study_url: {}", study_url);
        let send_result = match client.get(study_url).send().await {
            Ok(response) => response,
            Err(e) => {
                println!("{:#?}", e);
                continue;
            }
        };
        let study = match send_result.json::<model::study::Root>().await {
            Ok(json) => json,
            Err(e) => {
                println!("{:#?}", e);
                continue;
            }
        };

        // protocol_section
        let protocol_section = if let Some(study) = &study.study {
            if let Some(protocol_section) = &study.protocol_section {
                protocol_section
            } else {
                continue;
            }
        } else {
            continue;
        };

        // eligibility_criteria
        let eligibility_criteria = protocol_section
            .eligibility_module
            .as_ref()
            .and_then(|eligibility_module| eligibility_module.eligibility_criteria.as_deref())
            .unwrap_or("")
            .to_lowercase();
        let eligibility_criteria_split = eligibility_criteria
            .split("Exclusion Criteria:")
            .collect::<Vec<&str>>();
        let (inclusion_criteria, exclusion_criteria) = if eligibility_criteria_split.len() == 2 {
            (eligibility_criteria_split[0], eligibility_criteria_split[1])
        } else {
            (eligibility_criteria.as_str(), "")
        };

        // inclusion_criteria filter
        if config.keywords_in_inclusion {
            add_to_result = config
                .keywords
                .iter()
                .any(|k| inclusion_criteria.contains(k))
                || add_to_result;
        }

        // exclusion_criteria filter
        if config.keywords_in_exclusion {
            add_to_result = config
                .keywords
                .iter()
                .any(|k| exclusion_criteria.contains(k))
                || add_to_result;
        }

        // sponsor
        let sponsor = match &protocol_section.sponsor_collaborators_module {
            Some(sponsor_collaborators_module) => {
                match &sponsor_collaborators_module.lead_sponsor {
                    Some(lead_sponsor) => match &lead_sponsor.name {
                        Some(name) => name,
                        None => "-",
                    },
                    None => "-",
                }
            }
            None => "-",
        };

        // start_date
        let start_date = match &protocol_section.status_module {
            Some(status_module) => match &status_module.start_date_struct {
                Some(start_date_struct) => match &start_date_struct.date {
                    Some(date) => date,
                    None => "-",
                },
                None => "-",
            },
            None => "-",
        };

        // completion_date
        let completion_date = match &protocol_section.status_module {
            Some(status_module) => match &status_module.completion_date_struct {
                Some(completion_date_struct) => match &completion_date_struct.date {
                    Some(date) => date,
                    None => "-",
                },
                None => "-",
            },
            None => "-",
        };

        // status
        let status = match &protocol_section.status_module {
            Some(status_module) => match &status_module.overall_status {
                Some(overall_status) => overall_status,
                None => "-",
            },
            None => "-",
        };

        // phase
        let phase = match &protocol_section.design_module {
            Some(design_module) => match &design_module.phases {
                Some(phases) => {
                    if phases.is_empty() {
                        "-"
                    } else {
                        &phases[0]
                    }
                }
                None => "-",
            },
            None => "-",
        };

        // conditions
        let conditions = match &protocol_section.conditions_module {
            Some(conditions_module) => match &conditions_module.conditions {
                Some(conditions) => conditions.join(","),
                None => "-".to_string(),
            },
            None => "-".to_string(),
        };

        // conditions filter
        if config.keywords_in_conditions {
            add_to_result = config.keywords.iter().any(|k| conditions.contains(k)) || add_to_result;
        }

        // drug
        let drug = protocol_section
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
            .unwrap_or("".to_string());

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

        println!("match: {}", add_to_result);
        if add_to_result {
            result.push(csv_item);
        }
    }

    result.sort_by(|a, b| a.id.cmp(&b.id));
    println!("write to csv ...");
    write_to_csv(&result).await?;

    Ok(())
}
