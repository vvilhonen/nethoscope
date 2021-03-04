use std::sync::Arc;

mod audio;
mod cli;
mod network;

fn main() -> anyhow::Result<()> {
    cli::print_banner();

    let (counter, _stream) = audio::start()?;

    let interface = cli::parse_interface_from_args()?;
    network::start_capture(interface, Arc::clone(&counter))?;

    Ok(())
}
