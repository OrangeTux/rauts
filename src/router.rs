use std::collections::HashMap;
use std::hash::Hash;

use crate::extract::{FromRequest, Request};
use crate::handler::Handler;
use crate::ocpp::v16::{call_error::CallError, call_result::CallResult};
use crate::response::IntoResponse;

pub struct Router {
    routes: HashMap<String, Box<dyn Handler<Response = Box<dyn IntoResponse>>>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Default::default(),
        }
    }

    pub fn register<H: Handler<Response = Box<dyn IntoResponse>> + 'static, T: Into<String>>(
        mut self,
        handler: H,
        action: T,
    ) -> Self
    where
        H: Handler,
    {
        let action: String = action.into();
        if self.routes.contains_key(&action) {
            panic!("Route already exists");
        }
        self.routes.insert(action, Box::new(handler));
        self
    }

    pub fn route(&self, req: &Request) -> Result<CallResult, CallError> {
        let route = self.routes.get(&req.call.action);

        match route {
            Some(handler) => handler.call(req).into_response(&req.call),
            None => panic!("No route found for action {:?}.", req.call.action),
        }
    }
}
