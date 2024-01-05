use std::collections::HashMap;
use reqwest;
use serde_json::json;

async fn run_async_code() -> Result<(), reqwest::Error> {
    let code = "//hello world! I'm Krusty the Krab and i like money lol. \n \n//Insert your rusty code over here and i will do the rest\n \n//Also,I tell NO to code that asks input from user \n \n \nfn main(){\n    println!(6969696); //write code here\n}";
    let url = "https://play.rust-lang.org/execute";
    
    let mut data = HashMap::new();
    data.insert("channel", "stable");
    data.insert("mode", "debug");
    data.insert("edition", "2024");
    data.insert("crateType", "bin");
    data.insert("code", code);

    let client = reqwest::Client::new();

    let res = client.post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json!(data).to_string())  // Convert the HashMap to serde_json::Value using json!()
        .send()
        .await?;

    // Handle the response here
    println!("{:?}", res);

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run_async_code().await {
        // Handle the error here
        eprintln!("Error: {:?}", err);
    }
}
