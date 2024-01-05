use std::string;

slint::include_modules!();

//bussiness logic in this case compiling the code
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.compile_code_rs(move |string,string,string| {
        let ui = ui_handle.unwrap();

        //compile code here and change the property likewise
        ui.set_counter(ui.get_counter() + 1);
    });

    ui.run()
}
