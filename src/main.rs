mod handler;
mod logger;
mod request;
mod router;

use handler::{IntoHandler, IntoResponse};
use ocpp::v16::authorize::Authorize;
use request::{Call, Request, Source, CGW, TPBE};

use router::Router;
//use logger::Logger;

impl IntoResponse for u8 {
    fn into_response(&self) -> String {
        format!("{}", self)
    }
}

fn call_authorize_cgw(Call(Authorize { id_tag }, ..): Call<Authorize, CGW>) -> u8 {
    println!("CGW: Authorize id_tag: {}", id_tag);
    4
}

fn call_authorize_tpb(Call(authorize, ..): Call<Authorize, TPBE>) {
    println!("TPBE: {:?}", authorize);
}

fn main() {
    tracing_subscriber::fmt::init();
    let router = Router::new()
        //.route(Logger(call_authorize_cgw.into_handler()))
        //.route(Logger(call_authorize_tpb.into_handler()));
        .route(call_authorize_cgw.into_handler())
        .route(call_authorize_tpb.into_handler());

    let ocpp_message =
        ocpp::unpack(r#"[2, "424242", "Authorize",  {"idTag": "454564564"}]"#).unwrap();
    let request = Request(ocpp_message, Source::TPBE);
    println!("Response {}", router.call(&request));

    let ocpp_message =
        ocpp::unpack(r#"[2, "6666666", "Authorize",  {"idTag": "1333333337"}]"#).unwrap();
    let request = Request(ocpp_message, Source::CGW);
    println!("Response {}", router.call(&request));

    let ocpp_message =
        ocpp::unpack(r#"[2, "6666666", "BootNotification",  {"chargePointModel": "optimus prime", "chargePointVendor": ""}]"#).unwrap();
    let request = Request(ocpp_message, Source::Charger);
    println!("Response {}", router.call(&request));
}
