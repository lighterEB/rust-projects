use std::error::Error;

slint::include_modules!();
fn main() -> Result<(), Box<dyn Error>> {
    let app = MainWindow::new()?;
    app.run()?;
    Ok(())
    
}
