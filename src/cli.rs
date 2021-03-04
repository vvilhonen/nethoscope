use std::io::{stdout, Write};

pub fn print_banner() {
    eprintln!(
        "\n\x1b[1mNethoscope 🩺 {}\x1b[0m\n",
        env!("CARGO_PKG_VERSION")
    );
}

pub fn print_progress() -> anyhow::Result<()> {
    stdout().write(b".")?;
    stdout().flush()?;
    Ok(())
}

pub fn parse_interface_from_args() -> anyhow::Result<Option<String>> {
    let mut args = std::env::args().collect::<Vec<_>>();
    if args
        .iter()
        .any(|arg| arg.starts_with("-h") || arg.starts_with("--help"))
    {
        eprintln!(
            "Usage: {} <\x1b[4minterface name\x1b[0m or omit for the default>\n",
            args[0]
        );

        eprintln!("Available interfaces:");
        for device in super::network::available_interfaces()? {
            eprintln!(
                "name: {}\tdescription: {}",
                device.name,
                device.desc.unwrap_or("<no description>".to_owned())
            );
        }

        std::process::exit(0);
    } else if args.len() == 2 {
        Ok(Some(args.remove(1)))
    } else {
        Ok(None)
    }
}

#[cfg(unix)]
pub fn create_capture_permission_instruction() -> String {
    format!(
        "Couldn't start capture. Run `sudo setcap cap_net_raw,cap_net_admin=eip {}`",
        std::env::current_exe().unwrap().display()
    )
}

#[cfg(not(unix))]
pub fn create_capture_permission_instruction() -> String {
    format!("Couldn't start capture. Run with sudo or other with super user privileges")
}
