use anyhow::Result;
use regex::Regex;
use std::{io::Cursor, str::FromStr};

pub async fn fetch_url_to_file(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

pub async fn fetch_url(url: String) -> Result<Vec<u8>> {
    let response = reqwest::get(url).await?;
    Ok(response.bytes().await.unwrap().to_vec())
}

pub async fn iframe_link(race_url: &str) -> Result<String, &str> {
    let re = Regex::new(r#"(?i)src="([^"]*)""#).unwrap();
    let response = reqwest::get(race_url).await.unwrap().text().await.unwrap();
    for line in response.lines() {
        if line.contains("</iframe>") || line.contains("</IFRAME>") {
            if let Some(captures) = re.captures(line) {
                if let Some(src) = captures.get(1) {
                    return Ok(String::from_str(src.as_str()).unwrap());
                }
            }
        }
    }

    Err("No src attribute found")
}

pub async fn m3u8_link(iframe_url: &str) -> Result<String, &str>{
    let response = reqwest::get(iframe_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let re = Regex::new("'([^']*)'").unwrap();
    let re2 = Regex::new(r#""([^']*)""#).unwrap();
    for line in response.lines() {
        if line.contains("source: ") && !line.contains("//") {
            if let Some(captures) = re.captures(line) {
                if let Some(src) = captures.get(1) {
                    return Ok(String::from_str(src.as_str()).unwrap());
                }
            }

            if let Some(captures) = re2.captures(line) {
                if let Some(src) = captures.get(1) {
                    return Ok(String::from_str(src.as_str()).unwrap());
                }
            }
        }
    }

    Err("No m3u8 found")
}
