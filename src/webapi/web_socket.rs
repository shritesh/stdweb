use webapi::event_target::{IEventTarget, EventTarget};
use webcore::value::Reference;
use webcore::try_from::TryInto;

/// A `Websocket` Object
/// TODO: More docs
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
pub struct WebSocket( Reference );

reference_boilerplate!{
    WebSocket,
    instanceof WebSocket
    convertible to EventTarget
}

impl IEventTarget for WebSocket {}

impl WebSocket {
    /// Creates a new `WebSocket` with the given url.
    pub fn new(url: &str) -> Self {
        js!( return new WebSocket( @{url} ); ).try_into().unwrap()
    }

    /// Send string to the `WebSocket`
    pub fn send_string(&self, string: &str) {
        js!( @(no_return)
            @{self}.send( @{string} );
        );
    }
}