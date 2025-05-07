use crate::dom::events::event::Event;



// [LegacyTreatNonObjectAsNull]
// callback EventHandlerNonNull = any (Event event);
type EventHanderNonNull<T> = fn(Event) -> T;
// typedef EventHandlerNonNull? EventHandler;
pub type EventHandler<T> = Option<EventHanderNonNull<T>>;