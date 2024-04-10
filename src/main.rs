use reqwest;
use serde_json::Value;
use std::env;

async fn get_author_data(author_url: &str) -> Result<String, reqwest::Error> {
    let request_url = format!("https://openlibrary.org/{}.json", author_url);
    let response = reqwest::get(&request_url).await?.json::<Value>().await?;

    Ok(response["name"].as_str().unwrap_or("").to_string())
}

async fn get_data(isbn: &str) -> Result<Value, reqwest::Error> {
    let mut data_dictionary = serde_json::json!({});

    let request_url = format!("https://openlibrary.org/isbn/{}.json", isbn);
    let response = reqwest::get(&request_url).await?.json::<Value>().await?;

    data_dictionary["metadata"] = serde_json::json!({ "title": response["title"].as_str() });

    let authors = response["authors"].as_array().cloned().unwrap_or_else(Vec::new);
    let mut author_names = vec![];

    for author in &authors {
        let author_name = get_author_data(author["key"].as_str().unwrap_or("")).await?;
        author_names.push(author_name);
    }

    data_dictionary["authors"] = serde_json::json!(author_names);
    data_dictionary["publish_date"] = serde_json::json!(response["publish_date"].as_str());

    Ok(data_dictionary)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let isbn = &args[1];

    let data = get_data(isbn).await?;
    println!("{}", data);

    Ok(())
}