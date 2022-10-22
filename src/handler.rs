use crate::extract::{FromRequest, Request};
use crate::response::IntoResponse;
use std::{any::TypeId, marker::PhantomData};

pub struct ConcreteHandler<F, Args, O> {
    func: F,
    args: PhantomData<Args>,
    o: PhantomData<O>,
}

pub trait IntoHandler<F, Args, O> {
    fn into_handler(self: Self) -> ConcreteHandler<F, Args, O>;
}

pub trait Handler {
    type Response;
    fn call(&self, args: &Request) -> Self::Response;
    fn routing_key(&self) -> TypeId;
}

macro_rules! factory_tuple_handler ({ $($param:ident)* } => {
    impl<Func, O, $($param:FromRequest +'static,)*> Handler for ConcreteHandler<Func, ($($param,)*), O>
    where
        O: IntoResponse + 'static,
        Func: Fn($( $param, )*) -> O,
    {
        type Response = Box<dyn IntoResponse>;
        #[allow(unused_variables)]
            fn call(&self, request: &Request) -> Self::Response {
                Box::new((self.func)($( $param::from_request(request), )*))
            }

        fn routing_key(&self) -> TypeId {
            TypeId::of::<($( $param, )*)>()
        }
    }
});

factory_tuple_handler! {}
factory_tuple_handler! { A }
factory_tuple_handler! { A B }
factory_tuple_handler! { A B C }
factory_tuple_handler! { A B C D }
factory_tuple_handler! { A B C D E }
factory_tuple_handler! { A B C D E F }
factory_tuple_handler! { A B C D E F G }
factory_tuple_handler! { A B C D E F G H }
factory_tuple_handler! { A B C D E F G H I }
factory_tuple_handler! { A B C D E F G H I J }
factory_tuple_handler! { A B C D E F G H I J K }
factory_tuple_handler! { A B C D E F G H I J K L }

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, O, $($param,)*> IntoHandler<Func, ($($param,)*), O> for Func
    where
        O: IntoResponse,
        $( $param: FromRequest, )*
        Func: Fn($($param),*) -> O
    {
        // type Output = Box<dyn Debug>;

        #[inline]
        #[allow(non_snake_case)]
        fn into_handler(self: Func) -> ConcreteHandler<Func, ($($param,)*), O> { // -> Self::Output {
            ConcreteHandler {
                func: self,
                args: PhantomData,
                o: PhantomData,
            }
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
