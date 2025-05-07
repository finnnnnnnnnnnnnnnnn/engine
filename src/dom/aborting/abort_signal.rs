use crate::html::scripting::events::event_handler::EventHandler;


// there are defineitly really incorrect
// honestly I have no idea what is going on with the generics at the moment and this will defintily break
#[readonly::make]
pub struct AbortSignal<A> {
    //   readonly attribute boolean aborted;
    #[readonly]
    pub aborted: bool,
    //   readonly attribute any reason;
    #[readonly]
    pub reason: A,
    //   attribute EventHandler onabort;
    pub onabort: EventHandler<A>
}

impl<A> AbortSignal<A> {
    //   [NewObject] static AbortSignal abort(optional any reason);
    fn abort<T>(reason: T) -> Self {
        todo!();
    }
    //   [Exposed=(Window,Worker), NewObject] static AbortSignal timeout([EnforceRange] unsigned long long milliseconds);
    fn timeout(milliseconds: u64) -> Self {
        todo!();
    }
    //   [NewObject] static AbortSignal _any(sequence<AbortSignal> signals);
    fn _any<T>(signals: Vec<AbortSignal<T>>) -> Self {
        todo!();
    }
    //   undefined throwIfAborted();
    fn throw_if_aborted() {
        todo!();
    }

}