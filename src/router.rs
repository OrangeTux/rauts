use std::collections::HashMap;
use std::hash::Hash;

use crate::handler::Handler;
use crate::ocpp::v16::{call_error::CallError, call_result::CallResult};
use crate::request::{FromRequest, Request};
use crate::response::IntoResponse;

pub struct Router<T> {
    routes: HashMap<T, Box<dyn Handler<Response = Box<dyn IntoResponse>>>>,
}

impl<T> Router<T>
where
    T: Eq + Hash + FromRequest,
{
    pub fn new() -> Self {
        Self {
            routes: Default::default(),
        }
    }

    pub fn register<H: Handler<Response = Box<dyn IntoResponse>> + 'static>(
        mut self,
        handler: H,
        action: T,
    ) -> Self
    where
        H: Handler,
    {
        if self.routes.contains_key(&action) {
            panic!("Route already exists");
        }
        self.routes.insert(action, Box::new(handler));
        self
    }

    pub fn route(&self, req: &Request) -> Result<CallResult, CallError> {
        let action = T::from_request(req);
        let route = self.routes.get(&action);

        match route {
            Some(handler) => handler.call(req).into_response(&req.call),
            None => panic!("No route found for action {:?}.", req.call.action),
        }
    }
}
