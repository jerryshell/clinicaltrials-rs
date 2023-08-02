use std::collections::HashSet;

pub mod model;

pub async fn run() -> anyhow::Result<()> {
    let config = get_config();
    if config.cond_query.is_empty() && config.term_query.is_empty() {
        return Err(anyhow::anyhow!(
            "cond_query and term_query cannot be empty at the same time!"
        ));
    }

    let mut cond_query = config.cond_query;
    if cond_query.is_empty() {
        cond_query = config.term_query.clone();
    }
    println!("cond_query: {:?}", cond_query);

    let mut term_query = config.term_query;
    if term_query.is_empty() {
        term_query = cond_query.clone();
    }
    println!("term_query: {:?}", term_query);

    let keywords = config.keywords;
    println!("keywords: {:?}", keywords);
    let limit = 5000;

    let client = reqwest::Client::builder()
        .user_agent("Chrome/96.0.4664.110")
        .build()
        .unwrap();

    // download cond.json
    println!("search query.cond");
    let cond_url = url::Url::parse(&format!(
        "https://www.clinicaltrials.gov/api/int/studies?query.cond={}&agg.synonyms=true&aggFilters=&checkSpell=true&from=0&limit={}&fields=OverallStatus%2CHasResults%2CBriefTitle%2CCondition%2CInterventionType%2CInterventionName%2CLocationFacility%2CLocationCity%2CLocationState%2CLocationCountry%2CLocationStatus%2CLocationZip%2CLocationGeoPoint%2CLocationContactName%2CLocationContactRole%2CLocationContactPhone%2CLocationContactPhoneExt%2CLocationContactEMail%2CCentralContactName%2CCentralContactRole%2CCentralContactPhone%2CCentralContactPhoneExt%2CCentralContactEMail%2CGender%2CMinimumAge%2CMaximumAge%2CStdAge%2CNCTId%2CStudyType%2CLeadSponsorName%2CAcronym%2CEnrollmentCount%2CStartDate%2CPrimaryCompletionDate%2CCompletionDate%2CStudyFirstPostDate%2CResultsFirstPostDate%2CLastUpdatePostDate%2COrgStudyId%2CSecondaryId%2CPhase%2CLargeDocLabel%2CLargeDocFilename%2CPrimaryOutcomeMeasure%2CSecondaryOutcomeMeasure%2CDesignAllocation%2CDesignInterventionModel%2CDesignMasking%2CDesignWhoMasked%2CDesignPrimaryPurpose%2CDesignObservationalModel%2CDesignTimePerspective%2CLeadSponsorClass%2CCollaboratorClass&columns=conditions%2Cinterventions%2Ccollaborators&highlight=true",
        cond_query,
        limit,
    ))?.to_string();
    println!("cond_url: {}", cond_url);
    let cond_search_result = client
        .get(cond_url)
        .send()
        .await?
        .json::<model::search::Root>()
        .await?;
    // println!("{:#?}", cond_search_result);
    {
        // println!("save cond.json");
        // let mut file = std::fs::File::create("cond.json")?;
        // serde_json::to_writer_pretty(&mut file, &cond_search_result)?;
    }

    // download term.json
    println!("search query.term");
    let term_url = url::Url::parse(&format!(
        "https://www.clinicaltrials.gov/api/int/studies?query.term={}&agg.synonyms=true&aggFilters=&checkSpell=true&from=0&limit={}&fields=OverallStatus%2CHasResults%2CBriefTitle%2CCondition%2CInterventionType%2CInterventionName%2CLocationFacility%2CLocationCity%2CLocationState%2CLocationCountry%2CLocationStatus%2CLocationZip%2CLocationGeoPoint%2CLocationContactName%2CLocationContactRole%2CLocationContactPhone%2CLocationContactPhoneExt%2CLocationContactEMail%2CCentralContactName%2CCentralContactRole%2CCentralContactPhone%2CCentralContactPhoneExt%2CCentralContactEMail%2CGender%2CMinimumAge%2CMaximumAge%2CStdAge%2CNCTId%2CStudyType%2CLeadSponsorName%2CAcronym%2CEnrollmentCount%2CStartDate%2CPrimaryCompletionDate%2CCompletionDate%2CStudyFirstPostDate%2CResultsFirstPostDate%2CLastUpdatePostDate%2COrgStudyId%2CSecondaryId%2CPhase%2CLargeDocLabel%2CLargeDocFilename%2CPrimaryOutcomeMeasure%2CSecondaryOutcomeMeasure%2CDesignAllocation%2CDesignInterventionModel%2CDesignMasking%2CDesignWhoMasked%2CDesignPrimaryPurpose%2CDesignObservationalModel%2CDesignTimePerspective%2CLeadSponsorClass%2CCollaboratorClass&columns=conditions%2Cinterventions%2Ccollaborators&highlight=true",
        term_query,
        limit,
    ))?.to_string();
    println!("term_url: {}", term_url);
    let term_search_result = client
        .get(term_url)
        .send()
        .await?
        .json::<model::search::Root>()
        .await?;
    // println!("{:#?}", term_search_result);
    {
        // println!("save term.json");
        // let mut file = std::fs::File::create("term.json")?;
        // serde_json::to_writer_pretty(&mut file, &term_search_result)?;
    }

    // combine hits
    let mut cond_hits = cond_search_result.hits.unwrap_or(vec![]);
    println!("cond_hits: {}", cond_hits.len());
    let mut term_hits = term_search_result.hits.unwrap_or(vec![]);
    println!("term_hits: {}", term_hits.len());
    let mut hits = vec![];
    hits.append(&mut cond_hits);
    hits.append(&mut term_hits);
    println!("hints len: {}", hits.len());
    let hits_set = hits.into_iter().collect::<HashSet<model::search::Hit>>();
    println!("hits_set len: {}", hits_set.len());
    {
        // println!("save combine.json");
        // let mut file = std::fs::File::create("combine.json")?;
        // serde_json::to_writer_pretty(&mut file, &hits_set)?;
    }

    let mut result = vec![];
    for item in hits_set {
        let id = item.id.unwrap_or("".to_string());
        println!("id: {}", id);
        if id.is_empty() {
            continue;
        }

        // get study by id
        let study_url = format!("https://www.clinicaltrials.gov/api/int/studies/{}", id);
        println!("study_url: {}", study_url);
        let send_result = client.get(study_url).send().await;
        if send_result.is_err() {
            println!("{:#?}", send_result.unwrap_err());
            continue;
        }
        let json_result = send_result.unwrap().json::<model::study::Root>().await;
        if json_result.is_err() {
            println!("{:#?}", json_result.unwrap_err());
            continue;
        }
        let study = json_result.unwrap();

        // study
        if study.study.is_none() {
            continue;
        }

        // get protocol_section
        let protocol_section = study.study.unwrap().protocol_section;
        if protocol_section.is_none() {
            continue;
        }
        let protocol_section = protocol_section.unwrap();

        // eligibility_module
        if protocol_section.eligibility_module.is_none() {
            continue;
        }

        // get eligibility_criteria
        let eligibility_criteria = protocol_section
            .eligibility_module
            .unwrap()
            .eligibility_criteria;
        if eligibility_criteria.is_none() {
            continue;
        }
        let mut eligibility_criteria = eligibility_criteria.unwrap();

        // Exclusion Criteria filter
        let split = eligibility_criteria
            .split("Exclusion Criteria:")
            .collect::<Vec<&str>>();
        if split.len() >= 2 {
            eligibility_criteria = split.first().copied().unwrap_or("").to_string();
        }

        // keywords filter
        let contains_any_keyword_flag = keywords.iter().any(|k| eligibility_criteria.contains(k));
        println!("contains_any_keyword_flag: {}", contains_any_keyword_flag);
        if !contains_any_keyword_flag {
            continue;
        }

        // sponsor
        let sponsor = match protocol_section.sponsor_collaborators_module {
            Some(sponsor_collaborators_module) => match sponsor_collaborators_module.lead_sponsor {
                Some(lead_sponsor) => match lead_sponsor.name {
                    Some(name) => name,
                    None => "-".to_string(),
                },
                None => "-".to_string(),
            },
            None => "-".to_string(),
        };

        // start_date
        let start_date = match &protocol_section.status_module {
            Some(status_module) => match &status_module.start_date_struct {
                Some(start_date_struct) => match &start_date_struct.date {
                    Some(date) => date.to_string(),
                    None => "-".to_string(),
                },
                None => "-".to_string(),
            },
            None => "-".to_string(),
        };

        // completion_date
        let completion_date = match &protocol_section.status_module {
            Some(status_module) => match &status_module.completion_date_struct {
                Some(completion_date_struct) => match &completion_date_struct.date {
                    Some(date) => date.to_string(),
                    None => "-".to_string(),
                },
                None => "-".to_string(),
            },
            None => "-".to_string(),
        };

        // drug
        let mut drug_list = vec![];
        if let Some(arms_interventions_module) = protocol_section.arms_interventions_module {
            if let Some(interventions) = arms_interventions_module.interventions {
                for item in interventions {
                    if let Some(intervention_type) = item.intervention_type {
                        if "DRUG" == intervention_type {
                            if let Some(name) = item.name {
                                drug_list.push(name);
                            }
                        }
                    }
                }
            }
        }
        let drug = drug_list.join(",");

        let csv_item = model::csv_item::CsvItem {
            id,
            sponsor,
            start_date,
            completion_date,
            drug,
        };

        result.push(csv_item);
    }

    write_to_csv(&result).await?;

    Ok(())
}

pub fn get_config() -> model::config::Config {
    let config_file = std::fs::File::open("config.json").expect("Failed to open config.json");
    let reader = std::io::BufReader::new(config_file);
    serde_json::from_reader(reader).expect("config.json serialization failed")
}

pub async fn write_to_csv(data_list: &[model::csv_item::CsvItem]) -> anyhow::Result<()> {
    println!("write to csv...");

    let mut csv_writer = csv::Writer::from_path("data.csv")?;

    csv_writer.write_record(["id", "sponsor", "start_date", "completion_date", "drug"])?;

    for research_report in data_list.iter() {
        csv_writer.write_record(&[
            research_report.id.to_string(),
            research_report.sponsor.to_string(),
            research_report.start_date.to_string(),
            research_report.completion_date.to_string(),
            research_report.drug.to_string(),
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}
