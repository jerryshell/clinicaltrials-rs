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
    let hits_set: HashSet<clinicaltrials_rs::model::search::Hit> = hits.into_iter().collect();
    println!("hits_set len: {}", hits_set.len());

    Ok(())
}
