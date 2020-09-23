use std::collections::HashSet;

use anyhow::anyhow;
use anyhow::Result;
use rusb::*;

mod handler;

fn main() -> Result<()> {
    println!("All your numlock belongs to us!");
    let mut numlock_devices = HashSet::new();
    numlock_devices.insert("04d9:a232".to_string());

    if rusb::has_hotplug() {
        let context = Context::new()?;
        context.register_callback(
            None,
            None,
            None,
            Box::new(handler::DeviceHandler::new(numlock_devices)?),
        )?;

        loop {
            context.handle_events(None).unwrap();
        }
    } else {
        eprint!("libusb hotplug api unsupported");
        return Err(anyhow!("libusb hotplug api unsupported"));
    }
}
