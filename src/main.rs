mod handler;
mod logger;
mod request;
mod response;
mod router;

use handler::IntoHandler;
use ocpp::v16::authorize::Authorize;
use ocpp::v16::authorize_response::{AuthorizeResponse, IdTagInfo, Status};
use request::{Call, Request, Source, CGW, TPBE};

use logger::Logger;
use router::Router;

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

fn call_authorize_tpb(Call(authorize, ..): Call<Authorize, TPBE>) -> AuthorizeResponse {
    println!("TPBE: {:?}", authorize);
    AuthorizeResponse {
        id_tag_info: IdTagInfo {
            status: Status::Accepted,
            parent_id_tag: None,
            expiry_date: None,
        },
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .register(Logger(call_authorize_cgw.into_handler()))
        .register(Logger(call_authorize_tpb.into_handler()));
    //.route(call_authorize_cgw.into_handler())
    //.route(call_authorize_tpb.into_handler());

    let ocpp_message =
        ocpp::unpack(r#"[2, "424242", "Authorize",  {"idTag": "454564564"}]"#).unwrap();
    let request = Request(ocpp_message, Source::TPBE);
    dbg!(router.route(&request));

    let ocpp_message =
        ocpp::unpack(r#"[2, "6666666", "Authorize",  {"idTag": "1333333337"}]"#).unwrap();
    let request = Request(ocpp_message, Source::CGW);
    dbg!(router.route(&request));

    let ocpp_message =
        ocpp::unpack(r#"[2, "6666666", "BootNotification",  {"chargePointModel": "optimus prime", "chargePointVendor": ""}]"#).unwrap();
    let request = Request(ocpp_message, Source::Charger);
    dbg!(router.route(&request));
}
