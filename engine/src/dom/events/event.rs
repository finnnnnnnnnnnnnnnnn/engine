
use crate::hr_time::dom::DOMHighResTimeStamp;
use crate::types::DOMString;
use crate::dom::events::event_target::EventTarget;

#[readonly::make]
pub struct Event {
    // readonly attribute DOMString type;
    #[readonly]
    pub event_type: DOMString,
    // readonly attribute EventTarget? target;
    #[readonly]
    pub target: Option<EventTarget>,
    // readonly attribute EventTarget? srcElement; // legacy
    #[readonly]
    pub src_element: Option<EventTarget>,
    // readonly attribute EventTarget? currentTarget;
    #[readonly]
    pub current_target: Option<EventTarget>,

    // readonly attribute unsigned short eventPhase;
    #[readonly]
    pub event_phase: u16,
  
    // readonly attribute boolean bubbles;
    #[readonly]
    pub bubbles: bool,
    // readonly attribute boolean cancelable;
    #[readonly]
    pub cancelable: bool,
    //          attribute boolean returnValue;  // legacy
    pub return_value: bool,
    // readonly attribute boolean defaultPrevented;
    #[readonly]
    pub default_prevented: bool,
    // readonly attribute boolean composed;
    #[readonly]
    pub composed: bool,
  
    // [LegacyUnforgeable] readonly attribute boolean isTrusted;
    #[readonly]
    is_trusted: bool,
    // readonly attribute DOMHighResTimeStamp timeStamp;
    #[readonly]
    time_state: DOMHighResTimeStamp
}

impl Event{
    // constructor(DOMString type, optional EventInit eventInitDict = {});
    // not sure this is a correct interpretation of constructor
    fn new(event_type: DOMString, event_init_dict: Option<EventInit>) -> Self {
        todo!()
    }

    // sequence<EventTarget> composedPath();
    fn composed_path() -> Vec<EventTarget> {
        todo!();
    }

    // const unsigned short NONE = 0;
    const NONE: u16 = 0;
    // const unsigned short CAPTURING_PHASE = 1;
    const CAPTURING_PHASE: u16 = 1;
    // const unsigned short AT_TARGET = 2;
    const AT_TARGET: u16 = 2;
    // const unsigned short BUBBLING_PHASE = 3;
    const BUBBLING_PHASE: u16 = 3;

        // undefined stopPropagation();
    fn stop_propagation() {
        todo!();
    }
    //          attribute boolean cancelBubble; // legacy alias of .stopPropagation()
    fn cancel_bubble() {
        Self::stop_propagation()
    }
    // undefined stopImmediatePropagation();
    fn stop_immediate_propagation() {
        todo!();
    }
    // undefined preventDefault();
    fn prevent_default() {
        todo!();
    }
    // undefined initEvent(DOMString type, optional boolean bubbles = false, optional boolean cancelable = false); // legacy
    fn init_event(event_type: DOMString, bubbles: DefaultTrueBool, cancelable: DefaultTrueBool) {
        todo!();
    }
    
    
}

struct EventInit {
    //     boolean bubbles = false;
    pub bubbles: bool,
    //     boolean cancelable = false;
    pub cancelable: bool,
    //     boolean composed = false;
    pub composed: bool
}

impl Default for EventInit{
    fn default() -> Self {
        Self {
            bubbles: false,
            cancelable: false,
            composed: false
        }
    }
}

// I am yet to find a satisying way to have optional params with defaults
struct DefaultTrueBool(bool);

impl Default for DefaultTrueBool {
    fn default() -> Self {
        DefaultTrueBool(true)
    }
}