slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.on_request_calc({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let operation = ui.get_operation();
            let total = ui.get_total();
            let digit = ui.get_digit();
            // Do the actual calculations when the "=" is pressed
            match operation.as_str() {
                "+" => {
                    ui.set_total(total + digit);
                    ui.set_digit(0.0);
                }
                "-" => {
                    ui.set_total(total - digit);
                    ui.set_digit(0.0)
                }
                "*" => {
                    ui.set_total(total * digit);
                    ui.set_digit(0.0)
                }
                "/" => {
                    ui.set_total(total / digit);
                    ui.set_digit(0.0)
                }
                _ => {println!("Empty operation")}
            }
        }
    });
    ui.run()
}