use rauts::extract::{ChargerId, Request};
use rauts::handler::IntoHandler;
use rauts::ocpp::v16::{
    call::{Authorize, Call, Payload},
    call_result,
    call_result::{IdTagInfo, Status},
    Action,
};
use rauts::Router;

/// A handler function must have zero or more arguments. Each argument must implement
/// `rauts::extract::FromRequest`.
///
/// This handler functions has one argument of type `Request`.
fn authorize(request: Request) -> call_result::Authorize {
    let status = match request.call.payload {
        Payload::Authorize(payload) => {
            if payload.id_tag.as_str() == "454564564" {
                Status::Accepted
            } else {
                Status::Invalid
            }
        }
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
