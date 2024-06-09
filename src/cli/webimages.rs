use clap::Parser;
use image::ImageFormat;
use std::{ffi::OsStr, path::Path, str::FromStr};
use url::Url;

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
