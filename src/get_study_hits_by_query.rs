use anyhow::Result;
use std::collections::HashSet;

pub async fn get_study_hits_by_query(
    client: &reqwest::Client,
    query: &str,
) -> Result<HashSet<crate::model::search::Hit>> {
    let limit = 10000;

    // cond
    println!("search query.cond");
    let cond_url = url::Url::parse(&format!(
        "https://www.clinicaltrials.gov/api/int/studies?query.cond={}&agg.synonyms=true&aggFilters=&checkSpell=true&from=0&limit={}&fields=OverallStatus%2CHasResults%2CBriefTitle%2CCondition%2CInterventionType%2CInterventionName%2CLocationFacility%2CLocationCity%2CLocationState%2CLocationCountry%2CLocationStatus%2CLocationZip%2CLocationGeoPoint%2CLocationContactName%2CLocationContactRole%2CLocationContactPhone%2CLocationContactPhoneExt%2CLocationContactEMail%2CCentralContactName%2CCentralContactRole%2CCentralContactPhone%2CCentralContactPhoneExt%2CCentralContactEMail%2CGender%2CMinimumAge%2CMaximumAge%2CStdAge%2CNCTId%2CStudyType%2CLeadSponsorName%2CAcronym%2CEnrollmentCount%2CStartDate%2CPrimaryCompletionDate%2CCompletionDate%2CStudyFirstPostDate%2CResultsFirstPostDate%2CLastUpdatePostDate%2COrgStudyId%2CSecondaryId%2CPhase%2CLargeDocLabel%2CLargeDocFilename%2CPrimaryOutcomeMeasure%2CSecondaryOutcomeMeasure%2CDesignAllocation%2CDesignInterventionModel%2CDesignMasking%2CDesignWhoMasked%2CDesignPrimaryPurpose%2CDesignObservationalModel%2CDesignTimePerspective%2CLeadSponsorClass%2CCollaboratorClass&columns=conditions%2Cinterventions%2Ccollaborators&highlight=true",
        query,
        limit,
    ))?.to_string();
    println!("cond_url: {}", cond_url);
    let cond_search_result = client
        .get(cond_url)
        .send()
        .await?
        .json::<crate::model::search::Root>()
        .await?;
    // println!("{:#?}", cond_search_result);
    {
        // println!("save cond.json");
        // let mut file = std::fs::File::create("cond.json")?;
        // serde_json::to_writer_pretty(&mut file, &cond_search_result)?;
    }

    // term
    println!("search query.term");
    let term_url = url::Url::parse(&format!(
        "https://www.clinicaltrials.gov/api/int/studies?query.term={}&agg.synonyms=true&aggFilters=&checkSpell=true&from=0&limit={}&fields=OverallStatus%2CHasResults%2CBriefTitle%2CCondition%2CInterventionType%2CInterventionName%2CLocationFacility%2CLocationCity%2CLocationState%2CLocationCountry%2CLocationStatus%2CLocationZip%2CLocationGeoPoint%2CLocationContactName%2CLocationContactRole%2CLocationContactPhone%2CLocationContactPhoneExt%2CLocationContactEMail%2CCentralContactName%2CCentralContactRole%2CCentralContactPhone%2CCentralContactPhoneExt%2CCentralContactEMail%2CGender%2CMinimumAge%2CMaximumAge%2CStdAge%2CNCTId%2CStudyType%2CLeadSponsorName%2CAcronym%2CEnrollmentCount%2CStartDate%2CPrimaryCompletionDate%2CCompletionDate%2CStudyFirstPostDate%2CResultsFirstPostDate%2CLastUpdatePostDate%2COrgStudyId%2CSecondaryId%2CPhase%2CLargeDocLabel%2CLargeDocFilename%2CPrimaryOutcomeMeasure%2CSecondaryOutcomeMeasure%2CDesignAllocation%2CDesignInterventionModel%2CDesignMasking%2CDesignWhoMasked%2CDesignPrimaryPurpose%2CDesignObservationalModel%2CDesignTimePerspective%2CLeadSponsorClass%2CCollaboratorClass&columns=conditions%2Cinterventions%2Ccollaborators&highlight=true",
        query,
        limit,
    ))?.to_string();
    println!("term_url: {}", term_url);
    let term_search_result = client
        .get(term_url)
        .send()
        .await?
        .json::<crate::model::search::Root>()
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
    let hits_set = hits
        .into_iter()
        .collect::<HashSet<crate::model::search::Hit>>();
    println!("hits_set len: {}", hits_set.len());
    {
        // println!("save combine.json");
        // let mut file = std::fs::File::create("combine.json")?;
        // serde_json::to_writer_pretty(&mut file, &hits_set)?;
    }

    Ok(hits_set)
}
