use clap::Parser;
use std::fmt;
use std::str::FromStr;
use crate::cli::verify_input_file;
// Csv 面板
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser = parse_format_csv, default_value = "json")]
    pub format: OutputFormatCsv,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormatCsv {
    Json,
    Yaml,
}

fn parse_format_csv(format: &str) -> Result<OutputFormatCsv, anyhow::Error> {
    format.parse::<OutputFormatCsv>()
}

impl From<OutputFormatCsv> for &'static str {
    fn from(format: OutputFormatCsv) -> Self {
        match format {
            OutputFormatCsv::Json => "json",
            OutputFormatCsv::Yaml => "yaml",
        }
    }
}


impl FromStr for OutputFormatCsv {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormatCsv::Json),
            "yaml" => Ok(OutputFormatCsv::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormatCsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
