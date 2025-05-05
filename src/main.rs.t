use std::fs;

fn main() {
    let contents = fs::read_to_string("test.html").unwrap();


    let dom = tl::parse(&contents, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let element = dom.get_element_by_id("text")
    .expect("Failed to find element")
    .get(parser)
    .unwrap();

    assert_eq!(element.inner_text(parser), "Hello");
}
