use display::Display;
use error::XError;
use x11::xlib;
use std::str;
use libc::{ c_uint, c_int, c_uchar, c_char, c_ulong };
use std::ffi::{ CStr };
use std::mem::{ transmute, uninitialized };
use { XWMHints, window, XWindowChanges };
use std::slice::from_raw_parts;

#[derive(Debug)]
pub struct Window(pub u64);

impl Window {

    pub fn query_tree(&self, display: &Display) -> Vec<window::Window> {
        unsafe {
            let mut unused: c_ulong = 0;
            let mut children: *mut c_ulong = uninitialized();
            let children_ptr: *mut *mut c_ulong = &mut children;
            let mut num_children: c_uint = 0;
            xlib::XQueryTree(display.0,
                             self.0 as c_ulong,
                             &mut unused,
                             &mut unused,
                             children_ptr,
                             &mut num_children);
            let const_children: *const u64 = children as *const u64;

            from_raw_parts(const_children, num_children as usize)
                .iter()
                .filter(|&&c| c != self.0)
                .map(|c| window::Window(*c))
                .collect()
        }
    }

    pub fn grab_key(&self, display: &Display, key: i32, mask: u32) {
        unsafe { xlib::XGrabKey(display.0,
               key,
               mask,
               self.0 as c_ulong,
               1,
               1,
               1);
        }
    }

    pub fn get_wm_name(&self, display: &Display) -> Result<String, XError> {
        unsafe {
            let mut name: *mut c_char = uninitialized();
            let result = xlib::XFetchName(display.0, self.0 as c_ulong, &mut name);
            if result == 0 {
                Err(XError::BadProperty)
            } else {
                Ok(str::from_utf8_unchecked(CStr::from_ptr(name as *const c_char).to_bytes()).to_owned())
            }
        }
    }

    pub fn configure(&self, display: &Display, mask: u32, changes: &mut XWindowChanges) {
        unsafe {
            xlib::XConfigureWindow(display.0, self.0, mask, changes);
        }
    }

    pub fn get_wm_class(&self, display: &Display) -> Result<String, XError> {
        unsafe {
            let mut class_hint: xlib::XClassHint = uninitialized();
            let rs = xlib::XGetClassHint(display.0, self.0 as c_ulong, &mut class_hint);
            if rs == 0 || class_hint.res_class.is_null() {
                Err(XError::BadProperty)
            } else {
                Ok(CStr::from_ptr(class_hint.res_class).to_string_lossy().into_owned())
            }
        }
    }

    pub fn hints(&self, display: &Display) -> *mut XWMHints {
        unsafe {
            xlib::XGetWMHints(display.0, self.0)
        }
    }

    pub fn get_wm_role(&self, display: &Display) -> Result<String, XError> {
            let mut class_hint: xlib::XClassHint = unsafe { uninitialized() };
            let rs = unsafe { xlib::XGetClassHint(display.0, self.0 as c_ulong, &mut class_hint) };
            if rs == 0 || class_hint.res_class.is_null() {
                Err(XError::BadProperty)
            } else {
                Ok(unsafe { CStr::from_ptr(class_hint.res_name) }.to_string_lossy().into_owned())
            }
    }


    pub fn select_input(&self, display: &Display, mask: i64) {
        unsafe {
            xlib::XSelectInput(display.0, self.0, mask);
        }
    }

    pub fn list_properties(&self, display: &Display) -> Vec<u64> {
        let mut x = 0;
        let s = unsafe {
            xlib::XListProperties(display.0, self.0, &mut x)
        };
        let ss = unsafe { from_raw_parts(s as *const u64, x as usize) }.to_owned();
        unsafe { xlib::XFree(s as *mut ::std::os::raw::c_void) };
        ss

    }

    pub fn get_property(&self, display: &Display, atom: u64) -> Option<Vec<u8>> {
        unsafe {
            let mut actual_type_return: c_ulong = 0;
            let mut actual_format_return: c_int = 0;
            let mut nitems_return: c_ulong = 0;
            let mut bytes_after_return: c_ulong = 0;
            let mut prop_return: *mut c_uchar = uninitialized();

            let r = xlib::XGetWindowProperty(display.0,
                self.0 as c_ulong,
                atom as c_ulong,
                0,
                0xFFFFFFFF,
                0,
                0,
                &mut actual_type_return,
                &mut actual_format_return,
                &mut nitems_return,
                &mut bytes_after_return,
                &mut prop_return);

                if r != 0 {
                    None
                } else {
                    if actual_format_return == 0 {
                        None
                    } else {
                        Some(from_raw_parts(prop_return as *const c_ulong, nitems_return as usize)
                        .iter()
                        .map(|&c| c as u8)
                        .collect())
                    }
                }
        }
    }

    pub fn change_property(&self, display: &Display, property: u64,
                    typ: u64,
                    mode: c_int,
                    dat: &mut [c_ulong]) {
                        unsafe {
                            let ptr: *mut u8 = transmute(dat.as_mut_ptr());
                            xlib::XChangeProperty(display.0,
                                                  self.0 as c_ulong,
                                                  property as c_ulong,
                                                  typ as c_ulong,
                                                  32,
                                                  mode,
                                                  ptr,
                                                  1);
                        }
    }


    pub fn kill(self, display: &Display) {
            unsafe {
                xlib::XKillClient(display.0, self.0 as c_ulong);

            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Display;
    #[test]
    fn query_tree() {
        let mut d = Display::default().unwrap();
        let scr = d.default_screen();
        let rt = d.root_window(scr);
        assert!(rt.query_tree(&d).len() > 1);
    }
    #[test]
    fn list_properties() {
        let mut d = Display::default().unwrap();
        let scr = d.default_screen();
        let rt = d.root_window(scr);
        assert!(0 < rt.list_properties(&d).len());
    }


}
