use crate::{Result, native};
use serde::Serialize;
use windows_sys::Win32::{Foundation::HANDLE, Graphics::Gdi::HMONITOR};

/// Win32 `MONITORINFOF_PRIMARY` flag (value `1`).  Not exported by the
/// `windows-sys` Gdi feature, so we define it ourselves to avoid a magic
/// number in the primary-monitor check.
const MONITORINFOF_PRIMARY: u32 = 1;

#[derive(Debug, Clone, Serialize)]
pub struct BrightnessInfo {
    pub minimum: u32,
    pub current: u32,
    pub maximum: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContrastInfo {
    pub minimum: u32,
    pub current: u32,
    pub maximum: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct MonitorInfo {
    pub device_name: String,
    pub friendly_name: String,
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
    pub unsafe fn new(hmonitor: HMONITOR, physical_monitor: &native::PHYSICAL_MONITOR) -> Result<Self> {
        let monitor_info = unsafe { native::get_monitor_info(hmonitor)? };

        // Extract the actual device name from MONITORINFOEXW.szDevice (e.g. \\.\DISPLAY1)
        let device_name = {
            let raw = &monitor_info.szDevice;
            let len = raw.iter().position(|&c| c == 0).unwrap_or(raw.len());
            String::from_utf16_lossy(&raw[..len])
        };

        let is_primary = (monitor_info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY) != 0;

        Ok(Self {
            handle: physical_monitor.h_physical_monitor,
            info: MonitorInfo {
                device_name,
                friendly_name: physical_monitor.description(),
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
        let _ = unsafe { native::destroy_physical_monitor(self.handle) };
    }
}

/// Enumerate all physical monitors attached to the system.
///
/// Each returned `PhysicalMonitor` owns its handle and will call
/// `DestroyPhysicalMonitor` when dropped — no separate guard is needed.
///
/// On failure to create an individual `PhysicalMonitor` (e.g. bad monitor
/// info), the corresponding raw handle is explicitly destroyed so it does
/// not leak, and enumeration continues with the remaining monitors.
pub fn enumerate_monitors() -> Result<Vec<PhysicalMonitor>> {
    let enumerator = native::MonitorEnumerator::enumerate()?;
    let mut monitors = Vec::new();

    for hmonitor in enumerator.monitors {
        let physical_monitors = unsafe { native::get_physical_monitors(hmonitor)? };

        for pm in &physical_monitors {
            match unsafe { PhysicalMonitor::new(hmonitor, pm) } {
                Ok(monitor) => monitors.push(monitor),
                Err(e) => {
                    // PhysicalMonitor was not created, so its Drop will not
                    // run. Explicitly destroy the handle to avoid a leak.
                    let _ = unsafe { native::destroy_physical_monitor(pm.h_physical_monitor) };
                    eprintln!("Warning: Failed to create monitor: {e}");
                }
            }
        }
        // Do NOT destroy the handles here — ownership has been moved into
        // the PhysicalMonitor values that were successfully created.
    }

    Ok(monitors)
}

pub fn find_monitor(device_name: &str) -> Result<PhysicalMonitor> {
    let monitors = enumerate_monitors()?;

    let needle = device_name.to_lowercase();
    monitors
        .into_iter()
        .find(|m| {
            let info = m.info();
            info.device_name.to_lowercase() == needle || info.friendly_name.to_lowercase() == needle
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
