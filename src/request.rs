use ocpp::call::Payload;
use ocpp::v16::authorize::Authorize;
use ocpp::Message;

#[derive(Debug, Clone)]
pub struct TPBE;
#[derive(Debug, Clone)]
pub struct CGW;
#[derive(Debug, Clone)]
pub struct Charger;

#[derive(Debug, Clone)]
pub enum Source {
    TPBE,
    CGW,
    Charger,
}

impl FromRequest for Source {
    fn from_request(req: &Request) -> Self {
        req.1.clone()
    }
}

pub struct Call<T, S>(pub T, pub S);

impl FromRequest for Call<Authorize, TPBE> {
    fn from_request(req: &Request) -> Self {
        match &req.0 {
            Message::Call(call) => match &call.payload {
                Payload::Authorize(authorize) => Call(authorize.clone(), TPBE),
                _ => panic!("Must be type"),
            },
            _ => panic!("Must be a call!"),
        }
    }
}

impl FromRequest for Call<Authorize, CGW> {
    fn from_request(req: &Request) -> Self {
        match &req.0 {
            Message::Call(call) => match &call.payload {
                Payload::Authorize(authorize) => Call(authorize.clone(), CGW),
                _ => panic!("This route shouldn't be possible"),
            },
            _ => panic!("This route shouldn't be possible"),
        }
    }
}

#[derive(Debug)]
pub struct Request(pub ocpp::Message, pub Source);

pub trait FromRequest {
    fn from_request(req: &Request) -> Self;
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
