use rauts::extract::{ChargerId, Request};
use rauts::handler::IntoHandler;
use rauts::middleware::Logger;
use rauts::ocpp::v16::{
    call::{Call, Heartbeat},
    call_result, Action,
};
use rauts::Router;

use chrono::prelude::*;

/// A handler function must have 0 or more arguments. Each argument must implement
/// `rauts::extract::FromRequest`.
///
/// This handler function doesn't receive any arguments.
fn heartbeat() -> call_result::Heartbeat {
    call_result::Heartbeat {
        current_time: Utc::now(),
    }
}

fn main() {
    let router: Router<Action> =
        Router::new().register(Logger(heartbeat.into_handler()), Action::Heartbeat);

    let call: Call = Heartbeat {}.into();
    let request = Request {
        call,
        charger_id: ChargerId("Alfen 123".to_string()),
    };

    dbg!(router.route(&request).unwrap());
}
