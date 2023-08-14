use crate::protocol::{Event, TimedEvent};

pub trait EventSource {
    fn get_event(&mut self) -> Option<TimedEvent>;
    fn send_event(&self, event: Event);
}

#[cfg(test)]
pub mod tests {
    use crate::protocol::TimedEvent;

    use super::EventSource;

    pub struct TestEventSource {
        pub(crate) event: Option<TimedEvent>,
    }
    impl EventSource for TestEventSource {
        fn get_event(&mut self) -> Option<TimedEvent> {
            self.event
        }

        fn send_event(&self, _event: crate::protocol::Event) {}
    }
}
