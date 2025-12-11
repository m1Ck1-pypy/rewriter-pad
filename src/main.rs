//#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    rewriter_pad::main()?;
    Ok(())
}
