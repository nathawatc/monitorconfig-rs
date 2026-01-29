use crate::Result;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;

pub mod dxva2 {
    use super::*;

    #[link(name = "dxva2")]
    unsafe extern "system" {
        pub fn GetNumberOfPhysicalMonitorsFromHMONITOR(
            hmonitor: HMONITOR,
            pdwnumberofphysicalmonitors: *mut u32,
        ) -> i32;

        pub fn GetPhysicalMonitorsFromHMONITOR(
            hmonitor: HMONITOR,
            dwphysicalmonitorarraysize: u32,
            pphysicalmonitorarray: *mut PHYSICAL_MONITOR,
        ) -> i32;

        pub fn DestroyPhysicalMonitor(hmonitor: HANDLE) -> i32;

        pub fn DestroyPhysicalMonitors(
            dwphysicalmonitorarraysize: u32,
            pphysicalmonitorarray: *const PHYSICAL_MONITOR,
        ) -> i32;

        pub fn GetMonitorBrightness(
            hmonitor: HANDLE,
            pdwminimumbrightness: *mut u32,
            pdwcurrentbrightness: *mut u32,
            pdwmaximumbrightness: *mut u32,
        ) -> i32;

        pub fn SetMonitorBrightness(hmonitor: HANDLE, dwnewbrightness: u32) -> i32;

        pub fn GetMonitorContrast(
            hmonitor: HANDLE,
            pdwminimumcontrast: *mut u32,
            pdwcurrentcontrast: *mut u32,
            pdwmaximumcontrast: *mut u32,
        ) -> i32;

        pub fn SetMonitorContrast(hmonitor: HANDLE, dwnewcontrast: u32) -> i32;

        pub fn GetVCPFeatureAndVCPFeatureReply(
            hmonitor: HANDLE,
            bvcpcode: u8,
            pvct: *mut u32,
            pdwcurrentvalue: *mut u32,
            pdwmaximumvalue: *mut u32,
        ) -> i32;

        pub fn SetVCPFeature(hmonitor: HANDLE, bvcpcode: u8, dwnewvalue: u32) -> i32;

        pub fn SaveCurrentMonitorSettings(hmonitor: HANDLE) -> i32;

        pub fn RestoreMonitorFactoryDefaults(hmonitor: HANDLE) -> i32;

        pub fn RestoreMonitorFactoryColorDefaults(hmonitor: HANDLE) -> i32;

        pub fn GetCapabilitiesStringLength(
            hmonitor: HANDLE,
            pdwcapabilitiesstringlenghtincharacters: *mut u32,
        ) -> i32;

        pub fn CapabilitiesRequestAndCapabilitiesReply(
            hmonitor: HANDLE,
            pszasciicapabilitiesstring: *mut u8,
            dwcapabilitiesstringlenghtincharacters: u32,
        ) -> i32;
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct PHYSICAL_MONITOR {
    pub h_physical_monitor: HANDLE,
    pub sz_physical_monitor_description: [u16; 128],
}

impl PHYSICAL_MONITOR {
    pub fn description(&self) -> String {
        let len = self
            .sz_physical_monitor_description
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.sz_physical_monitor_description.len());
        String::from_utf16_lossy(&self.sz_physical_monitor_description[..len])
    }
}

pub struct MonitorEnumerator {
    pub monitors: Vec<HMONITOR>,
}

impl MonitorEnumerator {
    pub fn enumerate() -> Result<Self> {
        let mut monitors = Vec::new();

        unsafe {
            let result = EnumDisplayMonitors(
                std::ptr::null_mut(),
                std::ptr::null(),
                Some(enum_monitor_callback),
                &mut monitors as *mut _ as isize,
            );

            if result == 0 {
                return Err(crate::MonitorError::EnumerationFailed);
            }
        }

        Ok(Self { monitors })
    }
}

unsafe extern "system" fn enum_monitor_callback(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _lprect: *mut RECT,
    lparam: LPARAM,
) -> i32 {
    let monitors = &mut *(lparam as *mut Vec<HMONITOR>);
    monitors.push(hmonitor);
    1
}

pub fn get_monitor_info(hmonitor: HMONITOR) -> Result<MONITORINFOEXW> {
    unsafe {
        let mut info: MONITORINFOEXW = std::mem::zeroed();
        info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

        let result = GetMonitorInfoW(hmonitor, &mut info.monitorInfo as *mut _ as *mut _);
        if result == 0 {
            return Err(crate::MonitorError::EnumerationFailed);
        }

        Ok(info)
    }
}

pub fn get_physical_monitors(hmonitor: HMONITOR) -> Result<Vec<PHYSICAL_MONITOR>> {
    unsafe {
        let mut count = 0u32;
        let result = dxva2::GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut count);

        if result == 0 || count == 0 {
            return Err(crate::MonitorError::PhysicalMonitorHandleFailed);
        }

        let mut physical_monitors = vec![
            PHYSICAL_MONITOR {
                h_physical_monitor: std::ptr::null_mut(),
                sz_physical_monitor_description: [0; 128],
            };
            count as usize
        ];

        let result =
            dxva2::GetPhysicalMonitorsFromHMONITOR(hmonitor, count, physical_monitors.as_mut_ptr());

        if result == 0 {
            return Err(crate::MonitorError::PhysicalMonitorHandleFailed);
        }

        Ok(physical_monitors)
    }
}

pub fn destroy_physical_monitor(handle: HANDLE) -> Result<()> {
    unsafe {
        let result = dxva2::DestroyPhysicalMonitor(handle);
        if result == 0 {
            return Err(crate::MonitorError::EnumerationFailed);
        }
        Ok(())
    }
}
