use rauts::extract::{ChargerId, Request};
use rauts::handler::IntoHandler;
use rauts::middleware::Logger;
use rauts::ocpp::v16::{
    call::{Authorize, Call},
    call_result::{AuthorizeResponse, IdTagInfo, Status},
    Action,
};
use rauts::Router;

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

    let call: Call = Authorize {
        id_tag: "454564564".to_string(),
    }
    .into();

    let request = Request {
        call,
        charger_id: ChargerId("Alfen 123".to_string()),
    };

    dbg!(router.route(&request));
}
