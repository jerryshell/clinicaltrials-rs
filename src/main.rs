use std::collections::HashSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cond_query = "CPRC";
    let term_query = "CRPC";
    let limit = 5000;

    let client = reqwest::Client::builder()
        .user_agent("Chrome/96.0.4664.110")
        .build()
        .unwrap();

    // download cond.json
    println!("search query.cond");
    let cond_url = format!(
        "https://www.clinicaltrials.gov/api/int/studies?query.cond={}&agg.synonyms=true&aggFilters=&checkSpell=true&from=0&limit={}&fields=OverallStatus%2CHasResults%2CBriefTitle%2CCondition%2CInterventionType%2CInterventionName%2CLocationFacility%2CLocationCity%2CLocationState%2CLocationCountry%2CLocationStatus%2CLocationZip%2CLocationGeoPoint%2CLocationContactName%2CLocationContactRole%2CLocationContactPhone%2CLocationContactPhoneExt%2CLocationContactEMail%2CCentralContactName%2CCentralContactRole%2CCentralContactPhone%2CCentralContactPhoneExt%2CCentralContactEMail%2CGender%2CMinimumAge%2CMaximumAge%2CStdAge%2CNCTId%2CStudyType%2CLeadSponsorName%2CAcronym%2CEnrollmentCount%2CStartDate%2CPrimaryCompletionDate%2CCompletionDate%2CStudyFirstPostDate%2CResultsFirstPostDate%2CLastUpdatePostDate%2COrgStudyId%2CSecondaryId%2CPhase%2CLargeDocLabel%2CLargeDocFilename%2CPrimaryOutcomeMeasure%2CSecondaryOutcomeMeasure%2CDesignAllocation%2CDesignInterventionModel%2CDesignMasking%2CDesignWhoMasked%2CDesignPrimaryPurpose%2CDesignObservationalModel%2CDesignTimePerspective%2CLeadSponsorClass%2CCollaboratorClass&columns=conditions%2Cinterventions%2Ccollaborators&highlight=true",
        cond_query,
        limit,
    );
    println!("cond_url: {}", cond_url);
    let mut cond_search_result = client
        .get(cond_url)
        .send()
        .await?
        .json::<clinicaltrials_rs::model::search::Root>()
        .await?;
    // println!("{:#?}", cond_search_result);
    {
        println!("save cond.json");
        let mut file = std::fs::File::create("cond.json")?;
        serde_json::to_writer_pretty(&mut file, &cond_search_result)?;
    }

    // download term.json
    println!("search query.term");
    let term_url = format!(
        "https://www.clinicaltrials.gov/api/int/studies?query.term={}&agg.synonyms=true&aggFilters=&checkSpell=true&from=0&limit={}&fields=OverallStatus%2CHasResults%2CBriefTitle%2CCondition%2CInterventionType%2CInterventionName%2CLocationFacility%2CLocationCity%2CLocationState%2CLocationCountry%2CLocationStatus%2CLocationZip%2CLocationGeoPoint%2CLocationContactName%2CLocationContactRole%2CLocationContactPhone%2CLocationContactPhoneExt%2CLocationContactEMail%2CCentralContactName%2CCentralContactRole%2CCentralContactPhone%2CCentralContactPhoneExt%2CCentralContactEMail%2CGender%2CMinimumAge%2CMaximumAge%2CStdAge%2CNCTId%2CStudyType%2CLeadSponsorName%2CAcronym%2CEnrollmentCount%2CStartDate%2CPrimaryCompletionDate%2CCompletionDate%2CStudyFirstPostDate%2CResultsFirstPostDate%2CLastUpdatePostDate%2COrgStudyId%2CSecondaryId%2CPhase%2CLargeDocLabel%2CLargeDocFilename%2CPrimaryOutcomeMeasure%2CSecondaryOutcomeMeasure%2CDesignAllocation%2CDesignInterventionModel%2CDesignMasking%2CDesignWhoMasked%2CDesignPrimaryPurpose%2CDesignObservationalModel%2CDesignTimePerspective%2CLeadSponsorClass%2CCollaboratorClass&columns=conditions%2Cinterventions%2Ccollaborators&highlight=true",
        term_query,
        limit,
    );
    println!("term_url: {}", term_url);
    let mut term_search_result = client
        .get(term_url)
        .send()
        .await?
        .json::<clinicaltrials_rs::model::search::Root>()
        .await?;
    // println!("{:#?}", term_search_result);
    {
        println!("save term.json");
        let mut file = std::fs::File::create("term.json")?;
        serde_json::to_writer_pretty(&mut file, &term_search_result)?;
    }

    // combine hits
    let mut hits = vec![];
    println!(
        "cond_search_result.hits.len: {}",
        cond_search_result.hits.len()
    );
    println!(
        "term_search_result.hits.len: {}",
        term_search_result.hits.len()
    );
    hits.append(&mut cond_search_result.hits);
    hits.append(&mut term_search_result.hits);
    println!("hints len: {}", hits.len());
    let hits_set = hits
        .into_iter()
        .collect::<HashSet<clinicaltrials_rs::model::search::Hit>>();
    println!("hits_set len: {}", hits_set.len());
    {
        println!("save combine.json");
        let mut file = std::fs::File::create("combine.json")?;
        serde_json::to_writer_pretty(&mut file, &hits_set)?;
    }

    let keywords = vec!["enzalutamide", "abiraterone"];
    println!("keywords: {:?}", keywords);

    let mut result = vec![];
    for item in hits_set {
        let id = item.id;
        println!("id: {}", id);

        // get study by id
        let study_url = format!("https://www.clinicaltrials.gov/api/int/studies/{}", id);
        println!("study_url: {}", study_url);
        let study = client
            .get(study_url)
            .send()
            .await?
            .json::<clinicaltrials_rs::model::study::Root>()
            .await?;

        // get eligibility_criteria
        let eligibility_criteria = study
            .study
            .protocol_section
            .eligibility_module
            .eligibility_criteria;

        // keywords filter
        let contains_any_keyword_flag = keywords.iter().any(|&k| eligibility_criteria.contains(k));
        println!("contains_any_keyword_flag: {}", contains_any_keyword_flag);
        if !contains_any_keyword_flag {
            continue;
        }

        // sponsor
        let sponsor = study
            .study
            .protocol_section
            .sponsor_collaborators_module
            .lead_sponsor
            .name;

        // start_date
        let start_date = match study.study.protocol_section.status_module.start_date_struct {
            Some(d) => d.date,
            None => "-".to_string(),
        };

        // completion_date
        let completion_date = match study
            .study
            .protocol_section
            .status_module
            .completion_date_struct
        {
            Some(d) => d.date,
            None => "-".to_string(),
        };

        // drug
        let mut drug_list = vec![];
        if let Some(arms_interventions_module) =
            study.study.protocol_section.arms_interventions_module
        {
            if let Some(interventions) = arms_interventions_module.interventions {
                for item in interventions {
                    if "DRUG" == item.type_field {
                        drug_list.push(item.name);
                    }
                }
            }
        }
        // match study.study.protocol_section.arms_interventions_module {
        //     Some(arms_interventions_module) => match arms_interventions_module.interventions {
        //         Some(interventions) => {
        //             for item in interventions {
        //                 if "DRUG" == item.type_field {
        //                     drug_list.push(item.name);
        //                 }
        //             }
        //         }
        //         None => {}
        //     },
        //     None => {}
        // }
        let drug = drug_list.join(",");

        let csv_item = clinicaltrials_rs::model::csv_item::CsvItem {
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

pub async fn write_to_csv(
    data_list: &[clinicaltrials_rs::model::csv_item::CsvItem],
) -> anyhow::Result<()> {
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
