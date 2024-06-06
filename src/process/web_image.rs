// use anyhow::{anyhow, Result};
// use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};
// use image::imageops::overlay;
// use image::Luma;
// use image::{load_from_memory, DynamicImage, GenericImageView, ImageFormat};
// use std::{fmt::Display, time::Instant};

// fn url2image(url: &str) -> Result<DynamicImage> {
//     let start = Instant::now();

//     fn to_anyhow(e: impl Display) -> anyhow::Error {
//         anyhow!(e.to_string())
//     }

//     let browser = Browser::new(
//         LaunchOptionsBuilder::default()
//             .window_size(Some((1200, 1600)))
//             .build()
//             .unwrap(),
//     )
//     .map_err(to_anyhow)?;
//     let tab = browser.wait_for_initial_tab().map_err(to_anyhow)?;
//     let viewport = tab
//         .navigate_to(url)
//         .map_err(to_anyhow)?
//         .wait_for_element("body")
//         .map_err(to_anyhow)?
//         .get_box_model()
//         .map_err(to_anyhow)?
//         .margin_viewport();

//     dbg!(&viewport);
//     let data = tab
//         .capture_screenshot(ScreenshotFormat::PNG, Some(viewport), true)
//         .map_err(to_anyhow)?;

//     println!("time spend on url2image: {}ms", start.elapsed().as_millis());

//     Ok(load_from_memory(&data)?)
// }

// pub fn web2image(url: &str, output: &str, format: &str) -> Result<()> {
//     // 整理format
//     let mut bottom = url2image(url)?;

//     bottom.save_with_format(output, format)?;

//     Ok(())
// }
