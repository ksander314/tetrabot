use std::env;

use futures::StreamExt;
use telegram_bot::*;

async fn translate(a: &str, api_url: &str) -> String {
    let mut translation = String::from("NO TRANSLATION");
    let req = format!("{}{}", api_url, a);
    let resp = reqwest::get(&req).await;
    println!("here");
    if let Ok(r) = resp {
        let res: std::result::Result<serde_json::Value, _> = r.json().await;
        if let Ok(j) = res {
            if let Some(tr) = j["def"].as_array() {
                if tr.len() == 0 {
                    translation = "NO TRANSLATION".to_string();
                } else {
                    translation = tr[0]["tr"].as_array().unwrap()[0]["text"].to_string();
                }
            } else {
                translation = "NO TRANSLATION".to_string();
            }
            print!("\x1B[31m{}\x1B[m\x1B[90m({})\x1B[m ", a, translation);
        } else {
            println!("Can't parse translation response");
            println!("{:#?}", res);
        }
    }
    translation
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let args: Vec<String> = env::args().collect();
    let api_key = &args[1];
    let api_url = format!("https://dictionary.yandex.net/api/v1/dicservice.json/lookup?key={}&lang=en-ru&text=", api_key);
    // Fetch new updates via long poll method
    let mut stream = api.stream();
    let mut translation = String::from("NO TRANSLATION");
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                translation = translate(data, &api_url).await;

                // Answer message with "Hi".
                api.send(message.text_reply(format!(
                    "{}", translation
                )))
                .await?;
            }
        }
    }
    Ok(())
}
