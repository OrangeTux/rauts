use crate::{handler::Handler, request::Request};
use std::any::TypeId;
use tracing::info;

pub struct Logger<T: Handler>(pub T);
impl<T> Handler for Logger<T>
where
    T: Handler,
{
    type Response = T::Response;
    fn call(&self, args: &Request) -> Self::Response {
        let from = &args.charger_id;
        info!(?from, "ocpp msg: {:?}", args.call);
        let response = self.0.call(args);

        response
    }

    fn routing_key(&self) -> TypeId {
        self.0.routing_key()
    }
}
