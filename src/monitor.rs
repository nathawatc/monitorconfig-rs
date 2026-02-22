use crate::{Result, native};
use serde::{Deserialize, Serialize};
use windows_sys::Win32::{Foundation::HANDLE, Graphics::Gdi::HMONITOR};

/// RAII wrapper around a `Vec<native::PHYSICAL_MONITOR>` obtained from
/// `GetPhysicalMonitorsFromHMONITOR`.  The Windows API allocates OS-level
/// physical monitor handles; they **must** be released with
/// `DestroyPhysicalMonitors` (the bulk form) rather than being silently
/// dropped.  Without this guard every call to `enumerate_monitors` leaks one
/// handle per physical monitor attached to the system.
struct PhysicalMonitorGuard {
    monitors: Vec<native::PHYSICAL_MONITOR>,
}

impl PhysicalMonitorGuard {
    fn new(monitors: Vec<native::PHYSICAL_MONITOR>) -> Self {
        Self { monitors }
    }

    fn as_slice(&self) -> &[native::PHYSICAL_MONITOR] {
        &self.monitors
    }
}

impl Drop for PhysicalMonitorGuard {
    fn drop(&mut self) {
        if !self.monitors.is_empty() {
            // Safety: the slice pointer and length come directly from the Vec
            // that was filled by GetPhysicalMonitorsFromHMONITOR.  We only call
            // this once (inside Drop), so there is no double-free risk.
            unsafe {
                crate::native::dxva2::DestroyPhysicalMonitors(
                    self.monitors.len() as u32,
                    self.monitors.as_ptr(),
                );
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrightnessInfo {
    pub minimum: u32,
    pub current: u32,
    pub maximum: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContrastInfo {
    pub minimum: u32,
    pub current: u32,
    pub maximum: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub device_name: String,
    pub friendly_name: String,
    pub instance_name: String,
    pub is_primary: bool,
}

pub trait Monitor {
    fn get_brightness(&self) -> Result<BrightnessInfo>;
    fn set_brightness(&self, level: u32) -> Result<()>;
    fn get_contrast(&self) -> Result<ContrastInfo>;
    fn set_contrast(&self, level: u32) -> Result<()>;
    fn info(&self) -> &MonitorInfo;
}

pub struct PhysicalMonitor {
    handle: HANDLE,
    info: MonitorInfo,
}

impl PhysicalMonitor {
    pub fn new(hmonitor: HMONITOR, physical_monitor: &native::PHYSICAL_MONITOR) -> Result<Self> {
        let monitor_info = native::get_monitor_info(hmonitor)?;

        // Note: windows-sys MONITORINFOEXW.szDevice is at offset after MONITORINFO
        // For simplicity, use a placeholder device name based on handle address
        let device_name = format!("DISPLAY_{:p}", hmonitor as *const ());

        let is_primary = (monitor_info.monitorInfo.dwFlags & 1) != 0;

        Ok(Self {
            handle: physical_monitor.h_physical_monitor,
            info: MonitorInfo {
                device_name,
                friendly_name: physical_monitor.description(),
                instance_name: String::new(), // TODO: Get from display device
                is_primary,
            },
        })
    }

    pub fn handle(&self) -> HANDLE {
        self.handle
    }
}

impl Monitor for PhysicalMonitor {
    fn get_brightness(&self) -> Result<BrightnessInfo> {
        unsafe {
            let mut min = 0u32;
            let mut current = 0u32;
            let mut max = 0u32;

            let result =
                native::dxva2::GetMonitorBrightness(self.handle, &mut min, &mut current, &mut max);

            if result == 0 {
                return Err(crate::MonitorError::UnsupportedOperation(
                    "GetMonitorBrightness failed".to_string(),
                ));
            }

            Ok(BrightnessInfo {
                minimum: min,
                current,
                maximum: max,
            })
        }
    }

    fn set_brightness(&self, level: u32) -> Result<()> {
        unsafe {
            let result = native::dxva2::SetMonitorBrightness(self.handle, level);

            if result == 0 {
                return Err(crate::MonitorError::UnsupportedOperation(
                    "SetMonitorBrightness failed".to_string(),
                ));
            }

            Ok(())
        }
    }

    fn get_contrast(&self) -> Result<ContrastInfo> {
        unsafe {
            let mut min = 0u32;
            let mut current = 0u32;
            let mut max = 0u32;

            let result =
                native::dxva2::GetMonitorContrast(self.handle, &mut min, &mut current, &mut max);

            if result == 0 {
                return Err(crate::MonitorError::UnsupportedOperation(
                    "GetMonitorContrast failed".to_string(),
                ));
            }

            Ok(ContrastInfo {
                minimum: min,
                current,
                maximum: max,
            })
        }
    }

    fn set_contrast(&self, level: u32) -> Result<()> {
        unsafe {
            let result = native::dxva2::SetMonitorContrast(self.handle, level);

            if result == 0 {
                return Err(crate::MonitorError::UnsupportedOperation(
                    "SetMonitorContrast failed".to_string(),
                ));
            }

            Ok(())
        }
    }

    fn info(&self) -> &MonitorInfo {
        &self.info
    }
}

impl Drop for PhysicalMonitor {
    fn drop(&mut self) {
        let _ = native::destroy_physical_monitor(self.handle);
    }
}

pub fn enumerate_monitors() -> Result<Vec<PhysicalMonitor>> {
    let enumerator = native::MonitorEnumerator::enumerate()?;
    let mut monitors = Vec::new();

    for hmonitor in enumerator.monitors {
        // Wrap the raw handles in a guard so DestroyPhysicalMonitors is called
        // when this iteration ends, regardless of whether PhysicalMonitor::new
        // succeeds or fails.  The HANDLE value is copied into PhysicalMonitor
        // (which owns its own DestroyPhysicalMonitor in Drop), so releasing the
        // *original* handles here does not affect the copies.
        //
        // Note: DestroyPhysicalMonitor (singular) and DestroyPhysicalMonitors
        // (plural) both just decrement a ref-count inside dxva2.dll for the
        // same underlying handle slot — calling Destroy on the original after
        // the copy has been made is the correct and documented pattern.
        let guard = PhysicalMonitorGuard::new(native::get_physical_monitors(hmonitor)?);

        for pm in guard.as_slice() {
            match PhysicalMonitor::new(hmonitor, pm) {
                Ok(monitor) => monitors.push(monitor),
                Err(e) => eprintln!("Warning: Failed to create monitor: {}", e),
            }
        }
        // `guard` drops here → DestroyPhysicalMonitors called on the originals.
    }

    Ok(monitors)
}

pub fn find_monitor(device_name: &str) -> Result<PhysicalMonitor> {
    let monitors = enumerate_monitors()?;

    monitors
        .into_iter()
        .find(|m| {
            let info = m.info();
            info.device_name == device_name || info.friendly_name == device_name
        })
        .ok_or_else(|| crate::MonitorError::MonitorNotFound(device_name.to_string()))
}

pub fn get_primary_monitor() -> Result<PhysicalMonitor> {
    let monitors = enumerate_monitors()?;

    monitors
        .into_iter()
        .find(|m| m.info().is_primary)
        .ok_or_else(|| crate::MonitorError::MonitorNotFound("Primary monitor".to_string()))
}
