use crate::{dom::events::event_target::EventTarget, webidl::types::{OptionalDOMString, USVString}};

use super::node_tree::node_list::NodeList;

#[readonly::make]
pub struct Node {
    event_target: EventTarget,
    // readonly attribute unsigned short nodeType;
    #[readonly]
    pub node_type: u16,
    // readonly attribute DOMString nodeName;
    #[readonly]
    pub node_name: DOMString,
    // readonly attribute USVString baseURI;
    #[readonly]
    pub base_URI: USVString,
    // readonly attribute boolean isConnected;
    #[readonly]
    pub is_connected: bool,
    // readonly attribute Document? ownerDocument;
    #[readonly]
    pub owner_document: Option<Document>,
    // readonly attribute Node? parentNode;
    #[readonly]
    pub parent_node: Option<Box<Node>>,
    // readonly attribute Element? parentElement;
    #[readonly]
    pub parent_elemnt: Option<Element>,
    // [SameObject] readonly attribute NodeList childNodes;
    #[readonly]
    pub child_nodes: NodeList,
    // readonly attribute Node? firstChild;
    #[readonly]
    pub first_child: Option<Box<Node>>,
    // readonly attribute Node? lastChild;
    #[readonly]
    pub last_child: Option<Box<Node>>,
    // readonly attribute Node? previousSibling;
    #[readonly]
    pub previous_sibling: Option<Box<Node>>,
    // readonly attribute Node? nextSibling;
    #[readonly]
    pub next_sibling: Option<Box<Node>>,
    // [CEReactions] attribute DOMString? nodeValue;
    pub node_value: OptionalDOMString,
    // [CEReactions] attribute DOMString? textContent;
    pub text_content: OptionalDOMString
}

impl Node {
     // const unsigned short ELEMENT_NODE = 1;
    // const unsigned short ATTRIBUTE_NODE = 2;
    // const unsigned short TEXT_NODE = 3;
    // const unsigned short CDATA_SECTION_NODE = 4;
    // const unsigned short ENTITY_REFERENCE_NODE = 5; // legacy
    // const unsigned short ENTITY_NODE = 6; // legacy
    // const unsigned short PROCESSING_INSTRUCTION_NODE = 7;
    // const unsigned short COMMENT_NODE = 8;
    // const unsigned short DOCUMENT_NODE = 9;
    // const unsigned short DOCUMENT_TYPE_NODE = 10;
    // const unsigned short DOCUMENT_FRAGMENT_NODE = 11;
    // const unsigned short NOTATION_NODE = 12; // legacy

    // Node getRootNode(optional GetRootNodeOptions options = {});
    // boolean hasChildNodes();

    // [CEReactions] undefined normalize();

    // [CEReactions, NewObject] Node cloneNode(optional boolean subtree = false);
    // boolean isEqualNode(Node? otherNode);
    // boolean isSameNode(Node? otherNode); // legacy alias of ===

    // const unsigned short DOCUMENT_POSITION_DISCONNECTED = 0x01;
    // const unsigned short DOCUMENT_POSITION_PRECEDING = 0x02;
    // const unsigned short DOCUMENT_POSITION_FOLLOWING = 0x04;
    // const unsigned short DOCUMENT_POSITION_CONTAINS = 0x08;
    // const unsigned short DOCUMENT_POSITION_CONTAINED_BY = 0x10;
    // const unsigned short DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC = 0x20;
    // unsigned short compareDocumentPosition(Node other);
    // boolean contains(Node? other);

    // DOMString? lookupPrefix(DOMString? namespace);
    // DOMString? lookupNamespaceURI(DOMString? prefix);
    // boolean isDefaultNamespace(DOMString? namespace);

    // [CEReactions] Node insertBefore(Node node, Node? child);
    // [CEReactions] Node appendChild(Node node);
    // [CEReactions] Node replaceChild(Node node, Node child);
    // [CEReactions] Node removeChild(Node child);
}