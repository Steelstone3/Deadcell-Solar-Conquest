use bevy::ecs::event::{Event, EventReader};

/* pub fn resource_available<T: Resource>(resource: Option<Res<T>>) -> bool {
    resource.is_some()
} */

pub fn event_called<T: Event>(mut event_reader: EventReader<T>) -> bool {
    if event_reader.read().next().is_some() {
        return true;
    }
    false
}
