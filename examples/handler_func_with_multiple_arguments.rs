use rauts::extract::{ChargerId, Request};
use rauts::handler::IntoHandler;
use rauts::ocpp::v16::{
    call::{Authorize, Call},
    call_result,
    call_result::{IdTagInfo, Status},
    Action,
};
use rauts::Router;

/// A handler function must have zero or more arguments. Each argument must implement
/// `rauts::extract::FromRequest`.
///
/// This handler functions has two arguments, both implementing `rauts::extract::FromRequest`.
fn authorize(
    Authorize { id_tag }: Authorize,
    ChargerId(charger_id): ChargerId,
) -> call_result::Authorize {
    let status = match (id_tag.as_str(), charger_id.as_str()) {
        ("424242", "Alfen 123") => Status::Accepted,
        _ => Status::Invalid,
    };
    call_result::Authorize {
        id_tag_info: IdTagInfo {
            status,
            parent_id_tag: None,
            expiry_date: None,
        },
    }
}

fn main() {
    let router: Router<Action> =
        Router::new().register(authorize.into_handler(), Action::Authorize);

    let call: Call = Authorize {
        id_tag: "454564564".to_string(),
    }
    .into();

    let request = Request {
        call,
        charger_id: ChargerId("Alfen 123".to_string()),
    };

    dbg!(router.route(&request).unwrap());
}
