use reqwest;
use serde_json::Value;
use std::process::Command;

#[tokio::main]
async fn main() {
    let video_url = "https://www.youtube.com/watch?v=XeWZIzndlY4";
    let api_key = "AIzaSyBGn5-1nK6WT5G6iwWXIMbLTQYjIuWbhnc";
    let video_id = video_url.split("v=").last().unwrap();

    let url = format!("https://www.googleapis.com/youtube/v3/commentThreads?part=snippet&videoId={}&key={}", video_id, api_key);

    let response = reqwest::get(url).await.unwrap();
    let json: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    let mut html = String::new();
    html.push_str("<html><head><style>
        body {
            font-family: Arial, sans-serif;
            background-color: #FFBF00;
            text-align: center;
        }
        p {
            margin-bottom: 10px;
            font-size: 48px;
            color: #00BFFF;
            text-shadow: 2px 2px #ff0000;
            animation: blink 1s 3;
        }
        .author {
            font-weight: bold;
            color: #00BFFF;
            font-size: 64px;
            text-shadow: 2px 2px #0000ff;
        }
        @keyframes blink {
            0% {
                opacity: 1;
            }
            50% {
                opacity: 0;
            }
            100% {
                opacity: 1;
            }
        }
    </style></head><body>");

    for item in json["items"].as_array().unwrap() {
        let comment = item["snippet"]["topLevelComment"]["snippet"]["textDisplay"].as_str().unwrap();
        let author = item["snippet"]["topLevelComment"]["snippet"]["authorDisplayName"].as_str().unwrap();

        html.push_str(&format!("<p><span class='author'>{}</span>: {}</p>", author, comment));
    }

    html.push_str("</body></html>");

    std::fs::write("comments.html", html).unwrap();

    let mut child = Command::new("google-chrome")
        .arg("comments.html")
        .spawn()
        .unwrap();

    child.wait().unwrap();

    std::process::exit(0);
}
