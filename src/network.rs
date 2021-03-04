use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use anyhow::Context;

pub fn start_capture(
    interface_name: Option<String>,
    counter: Arc<AtomicUsize>,
) -> anyhow::Result<()> {
    let device = match interface_name {
        Some(interface_name) => {
            let devices = pcap::Device::list().context("Listing network devices")?;

            devices
                .into_iter()
                .find(|x| x.name == interface_name)
                .with_context(|| format!("No device named {}", interface_name))
        }
        _ => pcap::Device::lookup().context("Couldn't look up default capture device"),
    }?;

    eprintln!(
        "Capturing using {} ({})",
        device.desc.clone().unwrap_or("N/A".to_owned()),
        device.name
    );
    eprintln!("Run with \"-h\" to see how to capture with another network device\n");

    let mut cap = pcap::Capture::from_device(device)
        .and_then(|device| device.immediate_mode(true).open())
        .with_context(crate::cli::create_capture_permission_instruction)?;

    loop {
        match cap.next() {
            Ok(_pck) => {
                counter.fetch_add(1, Ordering::SeqCst);
            }
            Err(pcap::Error::TimeoutExpired) => (),
            Err(e) => {
                eprintln!("Unknown capture error {}", e);
                std::process::exit(1);
            }
        };
    }
}

pub fn available_interfaces() -> anyhow::Result<Vec<pcap::Device>> {
    Ok(pcap::Device::list()?)
}
