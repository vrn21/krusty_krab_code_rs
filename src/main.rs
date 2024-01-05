use std::{string, collections::HashMap};

slint::include_modules!();
use std::collections::HashMap;


//bussiness logic in this case compiling the code
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.compile_code_rs(move |string,string,string| {
        let ui = ui_handle.unwrap();

        let URL = "https://play.rust-lang.org/execute";
        let mut data : HashMap<&str,&str> = HashMap::new();
        data.insert("channel", "stable");
        data.insert("mode", "debug");
        data.insert("edition", "2024");
        data.insert("crateType", "bin");
        data.insert("tests", false);
        data.insert("code", ui.get_code());
        data.insert("backtrace", false);

        //compile code here and change the property likewise
        ui.set_stderr();
        ui.set_stdout();

    });

    ui.run()
}
