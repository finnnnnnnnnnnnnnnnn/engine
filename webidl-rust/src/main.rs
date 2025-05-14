// use inherit_derive::inherit;

use webidl_rust::inherit;
use webidl_rust::tlborm_fn_macro;

#[derive(Debug)]
struct InterfaceParent {
    field: &'static str
}

#[inherit(InterfaceParent)]
#[derive(Debug)]
struct Interface {
    other_field: &'static str
}

tlborm_fn_macro!(
    interface EventTarget {
        undefined addEventListener(DOMString type, EventListener? callback, optional (AddEventListenerOptions or boolean) options = {});
    };
);

impl EventTargetTrait for EventTarget {
}

fn main() {
    let test_interface = Interface {
        field: "test",
        other_field: "test"
    };
    println!("{:#?}", test_interface);
    println!("{:#?}", EventTarget {})
}

// planning
// I'm getting focused on the parser again.
// Let me think about how I'm going to do this.
// How is this going to actually be used?
// macro will be written with input widl
// this expands out to a struct and a trait
//   -> since the macro can't write the implementation obviously
// need some way to gurantee the trait is implemented
// what does this look like

// from_widl!(
//     interface EventTarget {
//         undefined addEventListener(DOMString type, EventListener? callback, optional (AddEventListenerOptions or boolean) options = {});
//     };
// );

// ->

//https://docs.rs/static_assertions/latest/static_assertions/macro.assert_impl_all.html

// struct EventTarget {}
// trait EventTargetImpl {
//      fn add_event_listener(...) -> ...
// }
// assert_impl_all!(EventTarget: EventTargetImpl);

