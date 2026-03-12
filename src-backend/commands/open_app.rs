use std::process::Command;

pub fn open_app(target: &str) -> std::io::Result<()> {
    // Use Windows "start" through cmd to resolve .lnk and .url shortcuts
    Command::new("cmd")
        .args(["/C", "start", "", target])
        .spawn()?;
    Ok(())
}

