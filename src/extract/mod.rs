use crate::ocpp::v16::{
    call::{Authorize, Call},
    Action,
};

#[derive(Debug, Clone)]
pub struct ChargerId(pub String);

impl FromRequest for ChargerId {
    fn from_request(req: &Request) -> Self {
        req.charger_id.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub call: Call,
    pub charger_id: ChargerId,
}

impl FromRequest for Request {
    fn from_request(req: &Request) -> Self {
        req.clone()
    }
}

pub trait FromRequest {
    fn from_request(req: &Request) -> Self;
}

impl FromRequest for Authorize {
    fn from_request(req: &Request) -> Self {
        let payload: Self = serde_json::from_value(req.call.payload.clone()).unwrap();
        payload
        //match &req.call.payload {
        //Payload::Authorize(payload) => payload.clone(),
        //_ => todo!("We probably want to use TryFromRequest instead of FromRequest."),
        //}
    }
}

impl FromRequest for Action {
    fn from_request(req: &Request) -> Self {
        req.call.action.clone()
    }
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<$($param,)*> FromRequest for ($($param,)*) where
        $( $param: FromRequest, )*
        {

        #[allow(unused_variables, non_snake_case)]
        fn from_request(req: &Request) -> Self
        {
            $( let $param = $param::from_request(req); )*
            ($($param,)*)
        }

    }
});

factory_tuple! {}
factory_tuple! { A }
factory_tuple! { A B }
factory_tuple! { A B C }
factory_tuple! { A B C D }
factory_tuple! { A B C D E }
factory_tuple! { A B C D E F }
factory_tuple! { A B C D E F G }
factory_tuple! { A B C D E F G H }
factory_tuple! { A B C D E F G H I }
factory_tuple! { A B C D E F G H I J }
factory_tuple! { A B C D E F G H I J K }
factory_tuple! { A B C D E F G H I J K L }
