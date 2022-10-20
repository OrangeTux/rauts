use std::{any::TypeId, marker::PhantomData};

use crate::request::{FromRequest, Request};

pub struct ConcreteHandler<F, Args> {
    func: F,
    args: PhantomData<Args>,
}

pub trait IntoHandler<F, Args> {
    fn into_handler(self: Self) -> ConcreteHandler<F, Args>;
}

pub trait Handler {
    fn call(&self, args: &Request); // -> Self::Output;
    fn routing_key(&self) -> TypeId;
}

impl<Func, A: FromRequest + 'static> Handler for ConcreteHandler<Func, (A,)>
where
    Func: Fn(A),
{
    #[allow(unused_variables)]
    fn call(&self, request: &Request) {
        (self.func)(A::from_request(request));
    }
    fn routing_key(&self) -> TypeId {
        TypeId::of::<(A,)>()
    }
}
impl<Func, A> IntoHandler<Func, (A,)> for Func
where
    A: FromRequest,
    Func: Fn(A),
{
    #[inline]
    #[allow(non_snake_case)]
    fn into_handler(self: Func) -> ConcreteHandler<Func, (A,)> {
        ConcreteHandler {
            func: self,
            args: PhantomData,
        }
    }
}

//macro_rules! factory_tuple_handler ({ $($param:ident)* } => {
//impl<Func, $($param:FromRequest +'static,)*> Handler for ConcreteHandler<Func, ($($param,)*)>
//where
//Func: Fn($( $param, )*),
//{
//#[allow(unused_variables)]
//fn call(&self, request: &Request) {
//(self.func)($( $param::from_request(request), )*);
//}
//fn routing_key(&self) -> TypeId {
//TypeId::of::<($( $param, )*)>()
//}
//}
//});

//factory_tuple_handler! {}
//factory_tuple_handler! { A }
//factory_tuple_handler! { A B }
//factory_tuple_handler! { A B C }
//factory_tuple_handler! { A B C D }
//factory_tuple_handler! { A B C D E }
//factory_tuple_handler! { A B C D E F }
//factory_tuple_handler! { A B C D E F G }
//factory_tuple_handler! { A B C D E F G H }
//factory_tuple_handler! { A B C D E F G H I }
//factory_tuple_handler! { A B C D E F G H I J }
//factory_tuple_handler! { A B C D E F G H I J K }
//factory_tuple_handler! { A B C D E F G H I J K L }

//macro_rules! factory_tuple ({ $($param:ident)* } => {
//impl<Func, $($param,)*> IntoHandler<Func, ($($param,)*)> for Func
//where
//$( $param: FromRequest, )*
//Func: Fn($($param),*)
//{
//// type Output = Box<dyn Debug>;

//#[inline]
//#[allow(non_snake_case)]
//fn into_handler(self: Func) -> ConcreteHandler<Func, ($($param,)*)> { // -> Self::Output {
//ConcreteHandler {
//func: self,
//args: PhantomData
//}
//}
//}
//});

//factory_tuple! {}
//factory_tuple! { A }
//factory_tuple! { A B }
//factory_tuple! { A B C }
//factory_tuple! { A B C D }
//factory_tuple! { A B C D E }
//factory_tuple! { A B C D E F }
//factory_tuple! { A B C D E F G }
//factory_tuple! { A B C D E F G H }
//factory_tuple! { A B C D E F G H I }
//factory_tuple! { A B C D E F G H I J }
//factory_tuple! { A B C D E F G H I J K }
//factory_tuple! { A B C D E F G H I J K L }
