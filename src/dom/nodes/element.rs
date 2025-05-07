use crate::webidl::types::{DOMString, OptionalDOMString};



#[readonly::make]
pub struct Element {
    // readonly attribute DOMString? namespaceURI;
    #[readonly]
    pub namespace_uri: OptionalDOMString,
    // readonly attribute DOMString? prefix;
    #[readonly]
    pub prefix: OptionalDOMString,
    // readonly attribute DOMString localName;
    #[readonly]
    pub local_name: DOMString,
    // readonly attribute DOMString tagName;
    #[readonly]
    pub tag_name: DOMString,
    // [CEReactions] attribute DOMString id;
    pub id: DOMString,
    // [CEReactions] attribute DOMString className;
    pub class_name: DOMString,
    // [SameObject, PutForwards=value] readonly attribute DOMTokenList classList;
    #[readonly]
    pub class_list: DOMtokenList,
    // [CEReactions, Unscopable] attribute DOMString slot;
    pub slot: DOMString,
    // [SameObject] readonly attribute NamedNodeMap attributes;
    pub attributes: NamedNodeMap,
    // readonly attribute ShadowRoot? shadowRoot;
    #[readonly]
    pub shadow_root: Option<ShadowRoot>,
    // readonly attribute CustomElementRegistry? customElementRegistry;
    #[readonly]
    pub custom_element_registry: Option<CustomElemntRegistry>
}

impl Element {
    // boolean hasAttributes();
    fn has_attributes() -> bool {
        todo!();
    }
    // sequence<DOMString> getAttributeNames();
    fn get_attribute_names() -> Vec<DOMString> {
        todo!();
    }
    // DOMString? getAttribute(DOMString qualifiedName);
    fn get_attribute
    // DOMString? getAttributeNS(DOMString? namespace, DOMString localName);
    // [CEReactions] undefined setAttribute(DOMString qualifiedName, DOMString value);
    // [CEReactions] undefined setAttributeNS(DOMString? namespace, DOMString qualifiedName, DOMString value);
    // [CEReactions] undefined removeAttribute(DOMString qualifiedName);
    // [CEReactions] undefined removeAttributeNS(DOMString? namespace, DOMString localName);
    // [CEReactions] boolean toggleAttribute(DOMString qualifiedName, optional boolean force);
    // boolean hasAttribute(DOMString qualifiedName);
    // boolean hasAttributeNS(DOMString? namespace, DOMString localName);
  
    // Attr? getAttributeNode(DOMString qualifiedName);
    // Attr? getAttributeNodeNS(DOMString? namespace, DOMString localName);
    // [CEReactions] Attr? setAttributeNode(Attr attr);
    // [CEReactions] Attr? setAttributeNodeNS(Attr attr);
    // [CEReactions] Attr removeAttributeNode(Attr attr);
  
    // ShadowRoot attachShadow(ShadowRootInit init);

    // Element? closest(DOMString selectors);
    // boolean matches(DOMString selectors);
    // boolean webkitMatchesSelector(DOMString selectors); // legacy alias of .matches
  
    // HTMLCollection getElementsByTagName(DOMString qualifiedName);
    // HTMLCollection getElementsByTagNameNS(DOMString? namespace, DOMString localName);
    // HTMLCollection getElementsByClassName(DOMString classNames);
  
    // [CEReactions] Element? insertAdjacentElement(DOMString where, Element element); // legacy
    // undefined insertAdjacentText(DOMString where, DOMString data); // legacy


}