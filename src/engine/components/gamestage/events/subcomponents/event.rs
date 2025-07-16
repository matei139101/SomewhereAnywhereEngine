pub trait Event {
    fn execute(&mut self);
    fn get_status(&self) -> &EventStatus;
}

#[derive(PartialEq)]
pub enum EventStatus {
    Pending,
    Processing,
    Done,
    Failed,
}