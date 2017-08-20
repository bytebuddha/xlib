#![allow(dead_code,unused_variables, unused_mut)]

extern crate x11;
extern crate libc;
extern crate futures;

mod display;
pub use display::Display;
mod error;
pub use error::XError;
mod window;
pub use window::Window;

pub use x11::xlib::{ XMapEvent, XDestroyWindowEvent, XConfigureEvent,
    XUnmapEvent, XPropertyEvent, XButtonEvent, XKeyEvent,
    ButtonPressMask, FocusChangeMask, Mod1Mask, Mod2Mask, Mod3Mask, Mod4Mask,
    EnterWindowMask, LeaveWindowMask, ControlMask, KeyPressMask, KeyReleaseMask,
    ButtonReleaseMask, PropertyChangeMask, SubstructureRedirectMask, SubstructureNotifyMask,
    PointerMotionMask, XWindowChanges, XWMHints
};

pub use x11::xlib::XEvent as RawXEvent;

mod event;
pub use event::Event;

#[derive(Debug)]
pub enum EventType {
    KeyPress = 2,
    ButtonPress = 4,
    ButtonRelease = 5,
    MotionNotify = 6,
    EnterNotify = 7,
    LeaveNotify = 8,
    DestroyNotify = 17,
    UnmapNotify = 18,
    MapRequest = 20,
    ConfigureRequest = 23,
    PropertyNotify = 28,
    ClientMessage = 33
}
macro_rules! impl_from_intiger {
    ($ty:ty) => {
        impl From<$ty> for EventType {
            fn from(f: $ty) -> EventType {
                match f {
                    2 => EventType::KeyPress,
                    4 => EventType::ButtonPress,
                    5 => EventType::ButtonRelease,
                    6 => EventType::MotionNotify,
                    7 => EventType::EnterNotify,
                    8 => EventType::LeaveNotify,
                    17 => EventType::DestroyNotify,
                    18 => EventType::UnmapNotify,
                    20 => EventType::MapRequest,
                    23 => EventType::ConfigureRequest,
                    28 => EventType::PropertyNotify,
                    33 => EventType::ClientMessage,
                    _ => panic!("Unkown Event Type")
                }
            }
        }
    }
}
impl_from_intiger!(i64);
impl_from_intiger!(u64);
impl_from_intiger!(i32);
impl_from_intiger!(u32);
impl_from_intiger!(i16);
impl_from_intiger!(u16);
impl_from_intiger!(i8);
impl_from_intiger!(u8);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
