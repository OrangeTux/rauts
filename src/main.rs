mod handler;
mod logger;
mod ocpp;
mod request;
mod response;
mod router;

use handler::IntoHandler;
use ocpp::v16::{
    call::{Authorize, Call, Payload},
    call_result::{AuthorizeResponse, IdTagInfo, Status},
    Action,
};
use request::{ChargerId, Request};

use logger::Logger;
use router::Router;

fn authorize(
    Authorize { id_tag }: Authorize,
    ChargerId(charger_id): ChargerId,
) -> AuthorizeResponse {
    let status = match (id_tag.as_str(), charger_id.as_str()) {
        ("424242", "Alfen 123") => Status::Accepted,
        _ => Status::Invalid,
    };
    AuthorizeResponse {
        id_tag_info: IdTagInfo {
            status,
            parent_id_tag: None,
            expiry_date: None,
        },
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    let router: Router<Action> =
        Router::new().register(Logger(authorize.into_handler()), Action::Authorize);

    let request = Request {
        call: Call::new(
            "424242".to_string(),
            Action::Authorize,
            Payload::Authorize(Authorize {
                id_tag: "454564564".to_string(),
            }),
        ),
        charger_id: ChargerId("Alfen 123".to_string()),
    };

    dbg!(router.route(&request));
}
