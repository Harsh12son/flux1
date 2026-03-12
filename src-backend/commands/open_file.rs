use std::process::Command;

pub fn open_file(path: &str) -> std::io::Result<()> {
    // Delegate to Windows ShellExecute via "start"
    Command::new("cmd")
        .args(["/C", "start", "", path])
        .spawn()?;
    Ok(())
}

