use std::{string, collections::HashMap};
use piston_rs;
use reqwest::Response;

slint::include_modules!();
use std::collections::HashMap;


//bussiness logic in this case compiling the code
async fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.compile_code_rs(move |string,string,string| {
        let ui = ui_handle.unwrap();
        let client = piston_rs::Client::new();
        let executor = piston_rs::Executor::new()
            .set_language("rust")
            .set_version("*")
            .add_file(piston_rs::File::default()
                .set_name("main.rs")
                .set_content(ui.get_code)
            );

        match client.execute(&executor)    {
            Ok(response) => {
                print!("Language : Rust {}",response.version);
                if let Some(c) = &response.compile{
                    ui.set_stderr(response.compile)
                }
                ui.set_stdout(response.run.output);
            }
            Err(err) => {
                ui.set_stderr("Something bad happend, its from our side, sorry, {:?}",err);
            }
        }

        

        // let URL = "https://play.rust-lang.org/execute";
        // let mut data : HashMap<&str,&str> = HashMap::new();
        // data.insert("channel", "stable");
        // data.insert("mode", "debug");
        // data.insert("edition", "2024");
        // data.insert("crateType", "bin");
        // data.insert("tests", false);
        // data.insert("code", ui.get_code());
        // data.insert("backtrace", false);

        //compile code here and change the property likewise
        // ui.set_stderr();
        // ui.set_stdout();

    });

    ui.run()
}
