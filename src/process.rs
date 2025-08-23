use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename="DOB")]
    dob: String,
    nationality: String,
    #[serde(rename="Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()>{
    let mut rdr = csv::Reader::from_path(input)?;
    let records = rdr.deserialize().
        map(|record| { record.unwrap() }).
        collect::<Vec<Player>>();

    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output,json)?;

    println!("{:?}",records);
    Ok(())
}