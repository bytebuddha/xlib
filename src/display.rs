use x11::xlib;
use error::XError;
use window::Window;
use std::ptr::{ null };
use std::ffi::{ CStr, CString} ;
use libc::{ c_char, c_ulong };
use std::mem::transmute;
use std::str::from_utf8;
use std::os::unix::io::{ RawFd, AsRawFd };
use Event;
pub struct Display(pub *mut xlib::Display);

impl Display {
    pub fn default() -> Result<Display, XError> {
        let display = unsafe { xlib::XOpenDisplay(null()) };
        if display.is_null() {
            Err(XError::OpenDisplayError)
        } else {
            Ok(Display(display))
        }
    }

    pub fn screen_count(&self) -> i32 {
        unsafe {
            xlib::XScreenCount(self.0)
        }
    }

    pub fn default_screen(&self) -> *mut xlib::Screen {
        unsafe {
            xlib::XDefaultScreenOfDisplay(self.0)
        }
    }

    pub fn number(&self) -> *mut i8 {
        unsafe {
            xlib::XDisplayName(self.0 as *const i8)
        }
    }

    pub fn send_event(&self, event: &mut Event, window: Window) {
        unsafe {
            xlib::XSendEvent(self.0, window.0 as c_ulong, 0, 0, &mut event.event);
        }
    }

    pub fn get_screen(&self, num: i32) -> *mut xlib::Screen {
        unsafe {
            xlib::XScreenOfDisplay(self.0, num)
        }
    }

    pub fn root_window(&self, screen: *mut xlib::Screen) -> Window {
        Window(unsafe {
            xlib::XRootWindowOfScreen(screen)
        })
    }

    fn get_keycode_from_string(key: &str) -> Result<u64, XError> {
        unsafe {
            match CString::new(key.as_bytes()) {
                Ok(b) => Ok(xlib::XStringToKeysym(b.as_ptr()) as u64),
                _ => Err(XError::BadKeyString),
            }
        }
    }

    fn get_string_from_keycode(&self, key: u32) -> Result<String, XError> {
        unsafe {
            let keysym = xlib::XKeycodeToKeysym(self.0, key as u8, 0);
            let keyname: *mut c_char = xlib::XKeysymToString(keysym);

            match from_utf8(CStr::from_ptr(transmute(keyname)).to_bytes()) {
                Ok(x) => Ok(x.to_owned()),
                _ => Err(XError::BadKeyCode),
            }
        }
    }


    pub fn intern_atom(&self, s: &str) -> Result<u64, XError> {
        match CString::new(s) {
            Ok(b) => Ok(unsafe { xlib::XInternAtom(self.0, b.as_ptr() as *const c_char, 0)  as u64}),
            _ => Err(XError::BadAtom)
        }
    }

    pub fn flush(&self) {
        unsafe {
            xlib::XFlush(self.0);
        }
    }

    pub fn events_pending(&self) -> bool {
        unsafe { xlib::XPending(self.0) != 0 }
    }

    pub fn get_event(&self) -> Result<Event, XError> {
        let mut event = xlib::XEvent { pad: [0; 24] };
        unsafe {
            xlib::XNextEvent(self.0, &mut event);
        }
        Ok(From::from(event))
    }
}

impl AsRawFd for Display {
    fn as_raw_fd(&self) -> RawFd {
        unsafe {
            xlib::XConnectionNumber(self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open() {
        let d = Display::default().unwrap();
        println!("ScreenCount: {}",d.screen_count());
    }
    #[ignore]
    #[test]
    fn window_props() {
        let d = Display::default().unwrap();
        let win = d.root_window(d.get_screen(0));
        for window in win.query_tree(&d) {
            println!("WINDOW: {}\nWM_NAME: {:#?}", window.0, window.get_wm_name(&d));
            println!("Role: {:#?}", window.get_wm_role(&d));
        }
    }
    #[test]
    fn current_desktop() {
        let d = Display::default().unwrap();
        let win = d.root_window(d.get_screen(0));
        let atom = d.intern_atom("_NET_CURRENT_DESKTOP").unwrap();
        println!("{:?}",win.get_property(&d, atom));
        win.change_property(&d, atom, atom, 0, &mut [2]);
        assert!(Some(vec![2]) == win.get_property(&d, atom));
    }
}
