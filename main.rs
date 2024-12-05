use reqwest;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let video_url = "https://www.youtube.com/watch?v=XeWZIzndlY4";
    let api_key = "AIzaSyBGn5-1nK6WT5G6iwWXIMbLTQYjIuWbhnc";
    let video_id = video_url.split("v=").last().unwrap();

    let url = format!("https://www.googleapis.com/youtube/v3/commentThreads?part=snippet&videoId={}&key={}", video_id, api_key);

    let response = reqwest::get(url).await.unwrap();
    let json: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    let mut html = String::new();
    html.push_str("<html><body>");

    for item in json["items"].as_array().unwrap() {
        let comment = item["snippet"]["topLevelComment"]["snippet"]["textDisplay"].as_str().unwrap();
        let author = item["snippet"]["topLevelComment"]["snippet"]["authorDisplayName"].as_str().unwrap();

        html.push_str(&format!("<p>{}: {}</p>", author, comment));
    }

    html.push_str("</body></html>");

    std::fs::write("comments.html", html).unwrap();
}
