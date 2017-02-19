#[derive(Debug)]
pub enum XError {
    OpenDisplayError,
    BadAtom,
    BadProperty,
    UnknownEventType,
    BadKeyCode,
    BadKeyString
}
