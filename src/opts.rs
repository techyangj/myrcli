use clap::Parser;
use core::fmt;
use image::ImageFormat;
use std::{ffi::OsStr, path::Path, str::FromStr};
use url::Url;

#[derive(Debug, Parser)]
#[clap(name = "myrcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(name = "web2images", about = "Generate a web to image")]
    WebImage(WebImageOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormatCsv {
    Json,
    Yaml,
}

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

// 密码修改面板
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

#[derive(Debug, Parser)]
pub struct WebImageOpts {
    #[arg(long, value_parser = verify_url)]
    url: String,
    #[arg(long, value_parser = verify_file_name, default_value = "./output.jpg")]
    output: String,
    #[arg(long, value_parser = parse_format_image, default_value = "jpg")]
    pub format: OutputFormatImage,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormatImage {
    Jpg,
    Png,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

fn parse_format_csv(format: &str) -> Result<OutputFormatCsv, anyhow::Error> {
    format.parse::<OutputFormatCsv>()
}

fn parse_format_image(format: &str) -> Result<OutputFormatImage, anyhow::Error> {
    format.parse::<OutputFormatImage>()
}

fn verify_url(url: &str) -> Result<String, String> {
    match Url::parse(url) {
        Ok(u) => Ok(u.into()),
        Err(_) => Err("You must provide a valid url".into()),
    }
}

fn get_image_format(path: &Path) -> Option<ImageFormat> {
    path.extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|ext| {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "png" => Some(ImageFormat::Png),
                "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
                _ => None,
            }
        })
}

fn verify_file_name(name: &str) -> Result<String, String> {
    let path = Path::new(name);
    let parent = path.parent().and_then(|p| p.is_dir().then(|| p));
    let ext = get_image_format(path);

    if parent.is_none() || ext.is_none() {
        return Err("File path must be exists and file must be jpg or png".into());
    }
    Ok(name.into())
}

impl From<OutputFormatImage> for &'static str {
    fn from(format: OutputFormatImage) -> Self {
        match format {
            OutputFormatImage::Jpg => "jpg",
            OutputFormatImage::Png => "png",
        }
    }
}

impl From<OutputFormatCsv> for &'static str {
    fn from(format: OutputFormatCsv) -> Self {
        match format {
            OutputFormatCsv::Json => "json",
            OutputFormatCsv::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormatImage {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jpg" => Ok(OutputFormatImage::Jpg),
            "png" => Ok(OutputFormatImage::Png),
            _ => Err(anyhow::anyhow!("Invalid format")),
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
