
struct InterfaceMember {

}
struct Object<T, P> {
    object: T,
    parent: Option<P>
}

struct Interface {
    field: String
}

// I need to somehow represent an object oriented structure
// maybe an object struct


fn main() {
    let interface: Object<Interface, Option<Parent>> = Object {
        object: Interface {
            field: "field".to_string()
        },
        parent: None
    };
}