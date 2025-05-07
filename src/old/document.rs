
use webidl::types::DOMString;

enum DocumentReadyState {
    LOADING,
    INTERACTIVE,
    COMPLETE
}

enum DocumentVisibilityState {
    VISIBLE,
    HIDDEN
}

enum HTMLOrSVGScriptElement {
    HTMLScriptElement(HTMLScriptElement),
    SVGScriptElement(SVGScriptElement)
}
struct Document {
    // resource metadata management
    location: Option<Location>, // [PutForwards=href, LegacyUnforgeable]
    pub domain: USVString,
    #[readonly]
    pub referrer: USVString,
    pub cookie: USVString,
    #[readonly]
    pub last_modified: DOMString,
    #[readonly]
    pub ready_state: DocumentReadyState,

    // DOM tree accessors
    pub title: DOMString, //  [CEReactions]
    pub dir: DOMString, //  [CEReactions]
    pub body: Option<HTMLElement>, //  [CEReactions]
    #[readonly]
    pub head: Option<HTMLHeadElement>, //  [SameObject]
    #[readonly]
    pub images: HTMLCollection, //  [SameObject]
    #[readonly]
    pub embeds: HTMLCollection, //  [SameObject]
    #[readonly]
    pub plugins: HTMLCollection, //  [SameObject]
    #[readonly]
    pub links: HTMLCollection, //  [SameObject]
    #[readonly]
    pub forms: HTMLCollection, //  [SameObject]
    #[readonly]
    pub scripts: HTMLCollection, //  [SameObject]
    #[readonly]
    pub current_script: Option<HTMLOrSVGScriptElement>,

    // user interaction
    #[readonly]
    pub default_view: Option<WindowProxy>,
    pub design_mode: DOMString, //  [CEReactions]
    #[readonly]
    pub hidden: bool,
    #[readonly]
    pub visibility_state: DocumentVisibilityState,

    // special event handler IDL attributes that only apply to Document objects
    pub on_ready_state_change: EventHandler, //   [LegacyLenientThis]
    pub on_visibility_change: EventHandler,
}

impl Document {
    fn parse_html_unsafe<S: DOMString>(html: S) -> Document {
        todo!();
    }

    fn get_elements_by_name(element_name: DOMString) -> Document {
        todo!();
    }
}



struct HTMLScriptElement {
}

struct SVGScriptElement {

}

struct HTMLElement {

}

struct HTMLHeadElement {

}

struct EventHandler {

}