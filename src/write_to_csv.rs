use anyhow::Result;

pub async fn write_to_csv(data_list: &[crate::model::csv_item::CsvItem]) -> Result<()> {
    let mut csv_writer = csv::Writer::from_path("data.csv")?;

    csv_writer.write_record([
        "id",
        "sponsor",
        "start_date",
        "completion_date",
        "status",
        "phase",
        "conditions",
        "drug",
    ])?;

    // add \t for stupid excel !!!
    for research_report in data_list.iter() {
        csv_writer.write_record(&[
            format!("\t{}", research_report.id),
            format!("\t{}", research_report.sponsor),
            format!("\t{}", research_report.start_date),
            format!("\t{}", research_report.completion_date),
            format!("\t{}", research_report.status),
            format!("\t{}", research_report.phase),
            format!("\t{}", research_report.conditions),
            format!("\t{}", research_report.drug),
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}
