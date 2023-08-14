use crate::protocol::TimedEvent;

pub mod sixbysix;

#[derive(Debug, Clone)]
pub struct LayoutError;

pub trait Layout {
    fn get_event(&mut self) -> Option<TimedEvent>;
}

#[cfg(test)]
pub mod tests {
    use crate::protocol::TimedEvent;

    use super::Layout;

    pub struct TestLayout {
        pub(crate) event: Option<TimedEvent>,
    }
    impl Layout for TestLayout {
        fn get_event(&mut self) -> Option<TimedEvent> {
            self.event
        }
    }
}
