use std::{collections::HashSet, process::Command};

use anyhow::anyhow;
use anyhow::Result;
use rusb::*;

trait DeviceString {
    fn device_string(&self) -> Result<String>;
}

impl<T: UsbContext> DeviceString for Device<T> {
    fn device_string(&self) -> Result<String> {
        let descr = self.device_descriptor()?;
        let d = format!("{:04x}:{:04x}", descr.vendor_id(), descr.product_id());
        //println!("{}",d);
        Ok(d)
    }
}

pub struct DeviceHandler {
    devices: HashSet<String>,
    numlock_devices: HashSet<String>,
}
impl<T: UsbContext> rusb::Hotplug<T> for DeviceHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        if let Err(e) = self.device_added(device) {
            eprint!("{}", e);
        }
    }

    fn device_left(&mut self, device: Device<T>) {
        if let Err(e) = self.device_removed(device) {
            eprint!("{}", e);
        }
    }
}

impl DeviceHandler {
    fn device_added<T: UsbContext>(&mut self, device: Device<T>) -> Result<()> {
        let descr = device.device_string()?;
        let contains = self.numlock_devices.contains(&descr);
        if self.devices.insert(descr) && contains {
            self.check_devices()?;
        }
        Ok(())
    }

    pub fn new(numlock_devices: HashSet<String>) -> Result<Self> {
        let devices = devices()?
            .iter()
            .map(|d| d.device_string())
            .collect::<Result<_>>()?;

        let mut handler = Self {
            numlock_devices,
            devices,
        };
        handler.check_devices()?;
        Ok(handler)
    }

    fn check_devices(&mut self) -> Result<()> {
        let diff = &self.devices - &self.numlock_devices;
        let external_keyboard = diff.len() < self.devices.len();

        let mut cmd = Command::new("numlockx");
        if external_keyboard {
            cmd.arg("on");
        } else {
            cmd.arg("off");
        }

        let res = cmd.output()?;
        if !res.status.success() {
            return Err(anyhow!("Error running numlockx: {:?}", res));
        }

        Ok(())
    }

    fn device_removed<T: UsbContext>(&mut self, device: Device<T>) -> Result<()> {
        let descr = device.device_string()?;
        if self.devices.remove(&descr) && self.numlock_devices.contains(&descr) {
            self.check_devices()?;
        }
        Ok(())
    }
}
