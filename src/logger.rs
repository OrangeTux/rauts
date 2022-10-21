use std::any::TypeId;

use tracing::info;

use crate::{handler::Handler, request::Request};

pub struct Logger<T: Handler>(pub T);
impl<T> Handler for Logger<T>
where
    T: Handler,
{
    type Response = T::Response;
    fn call(&self, args: &Request) -> Self::Response {
        let from = &args.1;
        info!(?from, "ocpp msg: {:?}", args.0);
        self.0.call(args)
    }

    fn routing_key(&self) -> TypeId {
        self.0.routing_key()
    }
}
