mod ui;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    ui::app_ui()?;
    Ok(())
}
