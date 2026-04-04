pub mod api {
    #[path = "hello/message.rs"]
    pub mod hello;
}

#[path = "api/hello/handler.rs"]
#[cfg(not(feature = "client"))]
mod hello_handler;
