//! Diagnostic module registry and exports.

mod batt;
mod boot;
mod cpu;
mod disk;
mod fan;
mod gpu;
mod io;
mod mem;
mod mount;
mod net;
mod sleep;
mod temp;
mod usb;

pub use batt::module as batt_module;
pub use boot::module as boot_module;
pub use cpu::module as cpu_module;
pub use disk::module as disk_module;
pub use fan::module as fan_module;
pub use gpu::module as gpu_module;
pub use io::module as io_module;
pub use mem::module as mem_module;
pub use mount::module as mount_module;
pub use net::module as net_module;
pub use sleep::module as sleep_module;
pub use temp::module as temp_module;
pub use usb::module as usb_module;

use crate::core::traits::DiagnosticModule;
use std::sync::Arc;

/// Return the module for the given name, if any.
pub fn get_module(name: &str) -> Option<Arc<dyn DiagnosticModule>> {
    match name {
        "boot" => Some(boot_module()),
        "cpu" => Some(cpu_module()),
        "mem" => Some(mem_module()),
        "disk" => Some(disk_module()),
        "io" => Some(io_module()),
        "net" => Some(net_module()),
        "fan" => Some(fan_module()),
        "temp" => Some(temp_module()),
        "gpu" => Some(gpu_module()),
        "batt" => Some(batt_module()),
        "sleep" => Some(sleep_module()),
        "usb" => Some(usb_module()),
        "mount" => Some(mount_module()),
        _ => None,
    }
}

/// Return all diagnostic modules in a fixed order.
pub fn all_modules() -> Vec<Arc<dyn DiagnosticModule>> {
    vec![
        boot_module(),
        cpu_module(),
        mem_module(),
        disk_module(),
        io_module(),
        net_module(),
        fan_module(),
        temp_module(),
        gpu_module(),
        batt_module(),
        sleep_module(),
        usb_module(),
        mount_module(),
    ]
}
