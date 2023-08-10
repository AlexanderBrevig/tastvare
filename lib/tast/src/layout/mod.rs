use crate::protocol::{Event, EventTime};

pub mod sixbysix;

#[derive(Debug, Clone)]
pub struct LayoutError;

pub trait Layout {
    fn get_event(&mut self) -> Option<(Event, EventTime)>;
}
