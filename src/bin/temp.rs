use std::collections::HashMap;
use reqwest;
use serde::{Serialize, Deserialize};
use piston_rs;

enum Args {
    Text(String),
    Bool(bool),
}

impl Serialize for Args {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Args::Text(text) => serializer.serialize_str(text),
            Args::Bool(boolean) => serializer.serialize_bool(*boolean),
        }
    }
}

async fn compile_rust() -> Result<(), reqwest::Error> {
    let code = "//hello world! I'm Krusty the Krab and i like money lol. \n \n//Insert your rusty code over here and i will do the rest\n \n//Also,I tell NO to code that asks input from user \n \n \nfn main(){\n    println!(6969696); //write code here\n}";
    
    //let mut data:HashMap<&str, Args> = HashMap::new();
    // rust playground api
    // let url = "https://play.rust-lang.org/execute";
    // data.insert("channel", Args::Text(String::from("stable")));
    // data.insert("mode", Args::Text(String::from("debug")));
    // data.insert("edition", Args::Text(String::from("2021")));
    // data.insert("crateType", Args::Text(String::from("bin")));
    // data.insert("tests", Args::Bool(false));
    // data.insert("code", Args::Text(String::from(code)));
    // data.insert("backtrace", Args::Bool(false));
    
    
    //piston api
    let mut data:HashMap<&str, &str> = HashMap::new();
    let url = "https://emkc.org/api/v2/piston/execute";

    data.insert("files", code);
    data.insert("version", "1.67.1");
    data.insert("language", "rust");


    let client = reqwest::Client::new();

    let res = client.post(url)
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .json(&data)  // Convert the HashMap to serde_json::Value using json!()
    .send()
    .await?;

    // let response_json: serde_json::Value = res.json().await?;

    println!("{:#?}", data);
    println!("{:#?}", res);

    Ok(())
}

#[tokio::main]
async fn main() {
    // if let Err(err) = compile_rust().await {
    //     // Handle the error here
    //     eprintln!("Error: {:?}", err);
    // }
    let code = "//hello world! I'm Krusty the Krab and i like money lol. \n \n//Insert your rusty code over here and i will do the rest\n \n//Also,I tell NO to code that asks input from user \n \n \nfn main(){\n    println!(); //write code here\n}";
    let client = piston_rs::Client::new();
    let executor = piston_rs::Executor::new()
        .set_language("rust")
        .set_version("*")
        .add_file(piston_rs::File::default()
            .set_name("main.rs")
            .set_content(code));

    match client.execute(&executor).await {
        Ok(response) => {
            println!("Ok response, ");
            println!("Language: {} \n \n",response.language);
            println!("version: {} \n \n",response.version);

            if let Some(c) = &response.compile{
                println!("Compiled: {:#?} \n \n",response.compile);
            }
            println!("StdOut :  {:#?} \n \n",response.run.output);
        }
        Err(err) =>{
            println!("Some error happend {:?}",err);
        }
    }    
}


//curl

// {
//     "channel": "stable",
//     "mode": "debug",
//     "edition": "2021",
//     "crateType": "bin",
//     "tests": False,
//     "code": code,
//     "backtrace": False
// }


// curl -X POST -H "Content-Type: application/json" \
//     -d '{
//         "channel": "stable",
//         "mode": "debug",
//         "edition": "2021",
//         "crateType": "bin",
//         "tests": false,
//         "code": "//hello world! I'm Krusty the Krab and i like money lol. \n \n//Insert your rusty code over here and i will do the rest\n \n//Also,I tell NO to code that asks input from user \n \n \nfn main(){\n    println!(6969696); //write code here\n}",
//         "backtrace": false
//     }' \
//     https://play.rust-lang.org/execute
    
//piston api


// "files": "//hello world! I'm Krusty the Krab and i like money lol. \n \n//Insert your rusty code over here and i will do the rest\n \n//Also,I tell NO to code that asks input from user \n \n \nfn main(){\n    println!(6969696); //write code here\n}",
// "version": "1.67.1",
// "language": "rust",
// }

// curl -X POST -H "Content-Type: application/json" \
//     -d '    "files": "//hello world! Im Krusty the Krab and i like money lol. \n \n//Insert your rusty code over here and i will do the rest\n \n//Also,I tell NO to code that asks input from user \n \n \nfn main(){\n    println!(6969696); //write code here\n}",
//     "version": "1.67.1",
//     "language": "rust",
// }' \
// https://emkc.org/api/v2/piston/execute

// let client = piston_rs::Client::new();
// let executor = piston_rs::Executor::new()
//     .set_language("rust")
//     .set_version("*")
//     .add_file(
//         piston_rs::File::default()
//             .set_name("main.rs")
//             .set_content("fn main() { println!(\"42\"); }")
//     );

// match client.execute(&executor).await {
//     Ok(response) => {
//         println!("Language: {}", response.language);
//         println!("Version: {}", response.version);

//         if let Some(c) = response.compile {
//             println!("Compilation: {}", c.output);
//         }

//         println!("Output: {}", response.run.output);
//     }
//     Err(e) => {
//         println!("Something went wrong contacting Piston.");
//         println!("{}", e);
//     }
// }