use codegen::Impl;




struct ParentInterface {
    field1: String
}
struct TestInterface {
    field2: String
}

// what's the ideal way to itneract with this?

// let test_interface = TestInterface { field1: ..., field2: .... }
// test_interface.field1;
// can't do this though

// let test_interface = Object { parent: ParentInterface { field1: ... }, child: TestInterface {field2: ...} }
//  test_interface.parent.field1;

// this is a pretty jank though particularly becuase if the child ovverrides the parent then you have to check if it's none 
//              (also all fields would have to be optioanl I think? Maybe you could od this with a macro)

// I guess everything could just live in a gigantic macro

// struct Person {
//     name: String,
//     surname: String,
//     kind: Kind,
// }

// https://users.rust-lang.org/t/recommended-approach-for-attribute-inheritance-in-rust/111750
// enum Kind {
//     Employee {
//         company_id: String,
//     },
//     Client {
//         client_id: String,
//     }
// }

// I'm going to pretend these are strings not functions
// interface Animal {
//   attribute String name;
// };

// interface Human : Animal {
//   attribute String pet_name;
// };

// interface Pet : Animal {
//   attribute String owner_name;
// };

struct Animal {
    name: String,
    kind: AnimalKind
}

enum AnimalKind {
    Human {
        pet_name: String
    },
    Pet {
        owner_name: String
    }
}

// I guess this works. I need a real world example with fields and stuff though

// interface Node {
//   undefined addEventListener(DOMString type, EventListener listener);
// };

// struct Node {}
// impl Node {
//     fn add_event_listener(listener: impl EventListener) {

//     }
// }

// callback interface EventListener {
//   undefined handleEvent(Event event);
// };

// trait EventListener {
//     fn handle_event(event: String);
// }

// this example works fine because it's just a trait

// added nonexistent field
// callback interface EventListener {
//   attribute String dummy;
//   undefined handleEvent(Event event);
// };

struct Node {}
impl Node {
    fn add_event_listener(&self, listener: impl EventListener) {
        listener.test("test".to_string());
    }
}


// maybe the trait requires getter and setter?

use webidl_rust::tlborm_fn_macro;

trait EventListener {
    fn handle_event(event: String);
    fn get_dummy() -> String;
    fn set_dummy(dummy: String);
    tlborm_fn_macro!(test);
}

struct Test {}
impl EventListener for Test {
    fn handle_event(event: String) {
        todo!()
    }

    fn get_dummy() -> String {
        todo!()
    }

    fn set_dummy(dummy: String) {
        todo!()
    }

    fn test(&self, dummy: String) {
        println!("something is borked")
    }
}

// and these functions could be created with a macroo???

// so one macro to add the functions to the trait
// and another macro to implement the trait

// this works as poc, however rust-analizer is mad incorrectly
// also I'm going to make fields of all structs modifed with getters and setters.

fn main() {
    // let human = Animal {
    //     name: "joe".to_string(),
    //     kind: AnimalKind::Human { pet_name: "pickles".to_string() }
    // };
    let el = Test{};
    el.test("test".to_string());
    let nod = Node{};
    nod.add_event_listener(el);
}

// An interface can be defined to inherit from another interface. If the identifier of the interface is followed by a U+003A (:) and an identifier, then that identifier identifies the inherited interface. 
// An object that implements an interface that inherits from another also implements that inherited interface. 
//The object therefore will also have members that correspond to the interface members from the inherited interface.

// Interfaces may specify an interface member that has the same name as one from an inherited interface.
//   ^not sure how to handle overring stuff like this

// I just found out proc macros don't work how I thought they did yayyyyyy
// I guess everything could just be defined inside pro macros