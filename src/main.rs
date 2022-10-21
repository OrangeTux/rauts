mod handler;
mod logger;
mod request;
mod response;
mod router;

use handler::IntoHandler;
use ocpp::call::Payload;
use ocpp::v16::authorize::Authorize;
use ocpp::v16::authorize_response::{AuthorizeResponse, IdTagInfo, Status};
use request::{ChargerId, FromRequest, Request};

use logger::Logger;
use router::Router;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Authorize,
    Heartbeat,
}

impl FromRequest for Action {
    fn from_request(req: &Request) -> Self {
        match req.call.action.as_str() {
            "Heartbeat" => Self::Heartbeat,
            "Authorize" => Self::Authorize,
            // TODO: We'd probably want to rename this function to try_from_request.
            _ => panic!("Not implemented"),
        }
    }
}

fn authorize(Authorize { id_tag }: Authorize, ChargerId(charger_id): ChargerId) -> AuthorizeResponse {
    let status = match (id_tag.as_str(), charger_id.as_str()) {
        ("424242", "Alfen 123") => Status::Accepted,
        _ => Status::Invalid
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
        call: ocpp::call::Call::new(
            "424242".to_string(),
            "Authorize".to_string(),
            Payload::Authorize(Authorize {
                id_tag: "454564564".to_string(),
            }),
        ),
        charger_id: ChargerId("Alfen 123".to_string()),
    };

    dbg!(router.route(&request));
}
