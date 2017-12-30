extern crate stdweb;

use stdweb::web::{alert, IEventTarget, WebSocket};
use stdweb::web::event::{OpenEvent, MessageEvent};

use std::sync::Arc;

fn main() {
    stdweb::initialize();

    let ws = Arc::new(WebSocket::new("ws://echo.websocket.org"));
    let sender = Arc::clone(&ws);

    ws.add_event_listener(move |_: OpenEvent| {
        sender.send_string("Hello World!");
    });

    ws.add_event_listener(move |event: MessageEvent| {
        alert(&event.data());
    });

    stdweb::event_loop();
}

