pub trait Event {}

pub trait Eventable {
    fn handle(event: Event);
}
