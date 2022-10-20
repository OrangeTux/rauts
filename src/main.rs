mod handler;
mod logger;
mod request;
mod response;
mod router;

use handler::IntoHandler;
use ocpp::call_result::{CallResult, Payload};
use ocpp::v16::authorize::Authorize;
use ocpp::v16::authorize_response::{AuthorizeResponse, IdTagInfo, Status};
use request::{Call, Request, Source, CGW, TPBE};
use response::IntoResponse;

use router::Router;
//use logger::Logger;

fn call_authorize_cgw(Call(Authorize { id_tag }, ..): Call<Authorize, CGW>) -> AuthorizeResponse {
    println!("CGW: Authorize id_tag: {}", id_tag);
    AuthorizeResponse {
        id_tag_info: IdTagInfo {
            status: Status::Accepted,
            parent_id_tag: None,
            expiry_date: None,
        },
    }
}

fn call_authorize_tpb(Call(authorize, ..): Call<Authorize, TPBE>) {
    println!("TPBE: {:?}", authorize);
}

fn main() {
    tracing_subscriber::fmt::init();
    let router = Router::new()
        //.route(Logger(call_authorize_cgw.into_handler()))
        //.route(Logger(call_authorize_tpb.into_handler()));
        .route(call_authorize_cgw.into_handler());
    //.route(call_authorize_tpb.into_handler());

    let ocpp_message =
        ocpp::unpack(r#"[2, "424242", "Authorize",  {"idTag": "454564564"}]"#).unwrap();
    let request = Request(ocpp_message, Source::TPBE);
    dbg!(router.call(&request));

    let ocpp_message =
        ocpp::unpack(r#"[2, "6666666", "Authorize",  {"idTag": "1333333337"}]"#).unwrap();
    let request = Request(ocpp_message, Source::CGW);
    dbg!(router.call(&request));

    let ocpp_message =
        ocpp::unpack(r#"[2, "6666666", "BootNotification",  {"chargePointModel": "optimus prime", "chargePointVendor": ""}]"#).unwrap();
    let request = Request(ocpp_message, Source::Charger);
    dbg!(router.call(&request));
}
