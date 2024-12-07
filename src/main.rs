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
    html.push_str("<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>YouTube Comments</title>
    <style>
        body {
            font-family: 'Comic Sans MS', cursive;
            background-color: #ff69b4;
            background-image: linear-gradient(to bottom, #ff69b4, #ffe6cc);
            background-size: 100% 100%;
            animation: pulse 5s infinite;
        }
        .comment {
            padding: 20px;
            border-bottom: 1px solid #ccc;
            border-radius: 20px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
            background-color: #ffffff;
        }
        .comment-author {
            font-weight: bold;
            font-size: 24px;
            color: #ff0000;
            text-shadow: 0 0 5px rgba(255, 0, 0, 0.5);
        }
        .comment-text {
            font-size: 18px;
            color: #0000ff;
            text-shadow: 0 0 5px rgba(0, 0, 255, 0.5);
        }
        @keyframes pulse {
            0% {
                background-color: #ff69b4;
            }
            50% {
                background-color: #ffe6cc;
            }
            100% {
                background-color: #ff69b4;
            }
        }
    </style>
</head>
<body>
    <h1>YouTube Comments</h1>
    <div id=\"comments\">");

    for item in json["items"].as_array().unwrap() {
        let comment = item["snippet"]["topLevelComment"]["snippet"]["textDisplay"].as_str().unwrap();
        let author = item["snippet"]["topLevelComment"]["snippet"]["authorDisplayName"].as_str().unwrap();

        html.push_str(&format!("<div class=\"comment\">
            <span class=\"comment-author\">@{}</span>
            <span class=\"comment-text\">{}</span>
        </div>", author, comment));
    }

    html.push_str("</div>
</body>
</html>");

    std::fs::write("comments.html", html).unwrap();
}
