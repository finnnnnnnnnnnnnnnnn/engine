use crate::webidl::types::DOMString;
use crate::dom::aborting::abort_signal::AbortSignal;
use super::event::Event;


pub struct EventTarget {}

impl EventTarget {
    pub fn new() -> Self {
        todo!()
    }
    //     undefined addEventListener(DOMString type, EventListener? callback, optional (AddEventListenerOptions or boolean) options = {});
    pub fn add_event_listener<A>(event_type: DOMString, callback: Option<EventListener>, options: Option<AddEventListenerOptions<A>>) {
        todo!()
    }
    //     undefined removeEventListener(DOMString type, EventListener? callback, optional (EventListenerOptions or boolean) options = {});
    pub fn remove_event_listener(event_type: DOMString, callback: Option<EventListener>, options: Option<EventListenerOptions>) {
        todo!()
    }
    //     boolean dispatchEvent(Event event);
    pub fn dispatch_event(event: Event) -> bool {
        todo!()
    }
}

type EventListener = fn(Event);

struct EventListenerOptions {
    capture: bool
}

// this is another case where things are funky and I just don't care at the moment
impl Default for EventListenerOptions {
    fn default() -> Self {
        EventListenerOptions { capture: false }
    }
}

struct AddEventListenerOptions<A> {
    event_listener_options: EventListenerOptions,
    passive: bool,
    signal: AbortSignal<A>,
    optional_add_event_listener_options: OptionalAddEventListenerOptions
}

struct OptionalAddEventListenerOptions {
    once: bool
}

impl Default for OptionalAddEventListenerOptions {
    fn default() -> Self {
        OptionalAddEventListenerOptions { once: false }
    }
}