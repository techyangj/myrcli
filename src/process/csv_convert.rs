use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormatCsv;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    //Name,Position,DOB,Nationality,Kit Number
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub number: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormatCsv) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormatCsv::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormatCsv::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;

    Ok(())
}
