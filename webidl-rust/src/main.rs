// use inherit_derive::inherit;

use webidl_rust::inherit;
struct InterfaceParent {
    field: &'static str
}

#[inherit(InterfaceParent)]
struct Interface {
    other_field: &'static str
}


// ok so the thing is there are sort of basically the same things.
// when you interact with an interface with a parent you can do pretty much what you do with a normal interface there's just an extra field
// maybe instead of this extra type layer, parent just becomes a reserved field of an interface?
// I may need to write some weird trait stuff later but I think this should work for now



fn main() {
    let test_interface = Interface {
        // field: "test",
        other_field: "test"
    };
}

// planning

// Despite now being able to inheritance everything probaly still needs to be a generic from of a some kind of object type because we also need to store a reference to a parent I think