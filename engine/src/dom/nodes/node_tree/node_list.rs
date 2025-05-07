
use crate::dom::nodes::node::Node;

#[readonly::make]
pub struct NodeList {
    //   readonly attribute unsigned long length;
    #[readonly]
    pub length: u32,
}

impl NodeList {
    //   getter Node? item(unsigned long index);
    fn node(index: u32) -> Option<Node> {
        todo!()
    }
}

impl Iterator for NodeList{
    //   iterable<Node>;
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}