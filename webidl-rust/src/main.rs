use inherit_derive::inherit;
struct Parent {
    field: String
}

#[inherit(Parent)]
struct Child {
    different_field: String
}

fn main() {
    let child = Child {
        field: "test".to_string(),
        different_field: "test".to_string()
    };
    println!("{:#?}", child);
}