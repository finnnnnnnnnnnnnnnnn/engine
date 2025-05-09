use webidl_rust::{a_getter, TraitWithField};

fn do_something_with_type_with_field(has_a: impl TraitWithField) {
    let a = has_a.get_a();
    println!("{:#?}", a);
}

trait TraitWithField {
    a_getter!();
}

#[derive(TraitWithField)]
struct HasA {
    a: String
}

fn main() {
    let has_a = HasA {
        a: "a".to_string()
    };
    do_something_with_type_with_field(has_a);
}

// I think this sort of solves the problem???
// I can write traits that require a struct have a certain field

// It's still not really inheritance though
