use EventType;
use RawXEvent;

pub struct Event {
    pub typ: EventType,
    pub event: RawXEvent
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{:?}({:?})",self.typ,self.event)
    }
}

impl From<RawXEvent> for Event {
    fn from(f: RawXEvent) -> Event {
        let event_type = f.get_type();
        Event {
            typ: From::from(event_type),
            event: f
        }
    }
}
