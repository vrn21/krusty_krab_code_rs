use std::collections::HashMap;
use reqwest;
use serde_json::json;

async fn compile_rust() -> Result<(), reqwest::Error> {
    let code = "//hello world! I'm Krusty the Krab and i like money lol. \n \n//Insert your rusty code over here and i will do the rest\n \n//Also,I tell NO to code that asks input from user \n \n \nfn main(){\n    println!(6969696); //write code here\n}";
    let url = "https://play.rust-lang.org/compile";
    
    let mut data = HashMap::new();
    data.insert("channel", "stable");
    data.insert("mode", "debug");
    data.insert("edition", "2021");
    data.insert("crateType", "bin");
    data.insert("code", code);

    let client = reqwest::Client::new();

    let res = client.post(url)
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .json(&json!(data))  // Convert the HashMap to serde_json::Value using json!()
    .send()
    .await?;

println!("{:?}", res);



    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = compile_rust().await {
        // Handle the error here
        eprintln!("Error: {:?}", err);
    }
}
