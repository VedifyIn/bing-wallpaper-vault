use serde::Deserialize;
use std::error::Error;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct BingResponse {
    images: Vec<BingImage>,
}

#[derive(Deserialize, Debug)]
struct BingImage {
    urlbase: String,
    title: String,
    copyright: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let parsed: BingResponse = serde_json::from_str(&body)?;
    let img = parsed.images.first().ok_or("No images in response")?;

    // Build the UHD image URL from urlbase
    let image_url = format!(
        "https://www.bing.com{urlbase}_UHD.jpg",
        urlbase = img.urlbase
    );
    println!("Downloading from: {}", image_url);

    // Download the image bytes
    let image_resp = reqwest::get(&image_url).await?;
    let image_bytes = image_resp.bytes().await?;

    // Save to disk
    tokio::fs::write("./test.jpg", &image_bytes).await?;
    println!("Saved {} bytes to ./test.jpg", image_bytes.len());

    // After saving ./test.jpg
    let abs_path = std::env::current_dir()?.join("test.jpg");
    set_wallpaper(abs_path.to_str().ok_or("invalid path")?)?;
    println!("Wallpaper set!");
    Ok(())
}

fn set_wallpaper(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let script = format!(
        r#"tell application "System Events" to set picture of every desktop to "{}""#,
        path
    );
    let output = Command::new("osascript").arg("-e").arg(&script).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("osascript failed: {}", stderr).into());
    }
    Ok(())
}
