use rauts::extract::{ChargerId, Request};
use rauts::handler::IntoHandler;
use rauts::middleware::Logger;
use rauts::ocpp::v16::{
    call::{Authorize, Call, Heartbeat},
    call_result,
    call_result::{IdTagInfo, Status},
    Action,
};
use rauts::Router;

use chrono::prelude::*;

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

fn heartbeat() -> call_result::Heartbeat {
    call_result::Heartbeat {
        current_time: Utc::now(),
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    let router: Router<Action> = Router::new()
        .register(Logger(authorize.into_handler()), Action::Authorize)
        .register(Logger(heartbeat.into_handler()), Action::Heartbeat);

    let call: Call = Authorize {
        id_tag: "454564564".to_string(),
    }
    .into();

    let request = Request {
        call,
        charger_id: ChargerId("Alfen 123".to_string()),
    };

    dbg!(router.route(&request));

    let call: Call = Heartbeat {}.into();
    let request = Request {
        call,
        charger_id: ChargerId("Alfen 123".to_string()),
    };

    dbg!(router.route(&request));
}
