use core_graphics::display::{CGDirectDisplayID, CGGetActiveDisplayList};
use std::ffi::CString;
use std::os::raw::{c_char, c_float, c_int, c_void};

const RTLD_LAZY: c_int = 0x1;
const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

type DisplayServicesGetBrightnessFn =
    unsafe extern "C" fn(display: CGDirectDisplayID, brightness: *mut c_float) -> c_int;

type DisplayServicesSetBrightnessFn =
    unsafe extern "C" fn(display: CGDirectDisplayID, brightness: c_float) -> c_int;

pub struct BrightnessController {
    display_id: CGDirectDisplayID,
    handle: *mut c_void,
    get_brightness_fn: DisplayServicesGetBrightnessFn,
    set_brightness_fn: DisplayServicesSetBrightnessFn,
}

impl BrightnessController {
    pub fn new() -> Result<Self, String> {
        let mut display_count: u32 = 0;
        let mut displays: [CGDirectDisplayID; 16] = [0; 16];

        unsafe {
            let result =
                CGGetActiveDisplayList(16, displays.as_mut_ptr(), &mut display_count);

            if result != 0 {
                return Err("Failed to get active displays".to_string());
            }

            if display_count == 0 {
                return Err("No active displays found".to_string());
            }

            let framework_paths = vec![
                "/System/Library/PrivateFrameworks/DisplayServices.framework/DisplayServices",
                "/System/Library/PrivateFrameworks/SkyLight.framework/SkyLight",
            ];

            let mut handle = std::ptr::null_mut();
            for path in &framework_paths {
                let framework_path = CString::new(*path).unwrap();
                handle = dlopen(framework_path.as_ptr(), RTLD_LAZY);
                if !handle.is_null() {
                    break;
                }
            }

            let search_handle = if handle.is_null() {
                RTLD_DEFAULT
            } else {
                handle
            };

            let get_brightness_name = CString::new("DisplayServicesGetBrightness").unwrap();
            let set_brightness_name = CString::new("DisplayServicesSetBrightness").unwrap();

            let get_fn_ptr = dlsym(search_handle, get_brightness_name.as_ptr());
            let set_fn_ptr = dlsym(search_handle, set_brightness_name.as_ptr());

            if get_fn_ptr.is_null() || set_fn_ptr.is_null() {
                if !handle.is_null() {
                    dlclose(handle);
                }
                return Err("DisplayServices functions not available on this system.".to_string());
            }

            let get_brightness_fn: DisplayServicesGetBrightnessFn =
                std::mem::transmute(get_fn_ptr);
            let set_brightness_fn: DisplayServicesSetBrightnessFn =
                std::mem::transmute(set_fn_ptr);

            Ok(BrightnessController {
                display_id: displays[0],
                handle,
                get_brightness_fn,
                set_brightness_fn,
            })
        }
    }

    pub fn get(&self) -> Result<f32, String> {
        let mut brightness: c_float = 0.0;

        unsafe {
            let result = (self.get_brightness_fn)(self.display_id, &mut brightness);
            if result != 0 {
                return Err(format!("Failed to get brightness: error code {}", result));
            }
        }

        Ok(brightness)
    }

    pub fn set(&self, brightness: f32) -> Result<(), String> {
        if !(0.0..=1.0).contains(&brightness) {
            return Err("Brightness must be between 0.0 and 1.0".to_string());
        }

        unsafe {
            let result = (self.set_brightness_fn)(self.display_id, brightness);
            if result != 0 {
                return Err(format!("Failed to set brightness: error code {}", result));
            }
        }

        Ok(())
    }
}

impl Drop for BrightnessController {
    fn drop(&mut self) {
        unsafe {
            if !self.handle.is_null() {
                dlclose(self.handle);
            }
        }
    }
}
