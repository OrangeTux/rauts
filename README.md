# Rauts
Rauts is library for building [Open Charge Point Protocol][OCPP] (OCPP) servers
and clients. OCPP is de facto communication standard between chargers of electric vehicles (EV) and the servers
managing the chargers.

The project is in very early stage. Currently, the focus is on designing a satisfying API for routing
requests and responses.

## Quickstart

Clone the repository and run one of the [examples](examples/):

```bash
$ git clone https://github.com/OrangeTux/rauts.git
$ cd rauts
$ cargo run --example handler_func_with_multiple_arguments
```

## Handlers

The project is heavily inspired by "magic handler functions" as implemented in
[Axum](https://docs.rs/axum/latest/axum/#handlers) and other projects.

A regular function automatically implements the `Handler` trait when:
1) It receives zero or more arguments. All arguments must implement `rauts::extract::FromRequest`.
2) It must return something that implements `rauts::response::IntoResponse`

### Arguments

These are valid examples of handler functions:

```rust
use rauts::ocpp::v16::{
    call::{Authorize, Call, Payload},
    call_result,
    call_result::{IdTagInfo, Status},
    Action,
};

/// This `Handler` doesn't receive any arguments.
fn heartbeat() -> call_result::Heartbeat {
    call_result::Heartbeat {
        current_time: Utc::now(),
    }
}

/// This `Handler` has one argument of type `Request`.
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

/// This `Handler` receives two arguments. Types of both arguments implement `rauts::extract::FromRequest`.
fn also_authorize(
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
```

## Router

`Handler`s are registered at a `Router`. The `Router` is responsible for calling the correct `Handler` when the `Router` receives a `Request`.

```rust
use rauts::extract::{ChargerId, Request}
use rauts::handler::IntoHandler;
use rauts::Router;
use rauts::ocpp::v16::{
   call::{Call, Heartbeat},
   Action
}

let router: Router<Action> =
   Router::new()
      .register(heartbeat.into_handler(), Action::Heartbeat)
      .register(authorize.into_handler(), Action::Authorize);

let call: Call = Heartbeat {}.into();
let request = Request {
    call,
    charger_id: ChargerId("Alfen 123".to_string()),
};

router.route(&request).unwrap();
```

## Further reading
* [Magical handler functions in Rust - Bernard Kolobara](https://lunatic.solutions/blog/magic-handler-functions-in-rust/)
* [Rust's Axum style magic functions example -  Alex Puschinsky](https://github.com/alexpusch/rust-magic-function-params)
* [A Reddit thread discussing the previous post](https://www.reddit.com/r/rust/comments/xornz5/axums_magical_handler_methods_amazed_me_when_i/)

[Open Charge Point Protocol]: https://en.wikipedia.org/wiki/Open_Charge_Point_Protocol
