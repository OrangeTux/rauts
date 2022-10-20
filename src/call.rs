use crate::Request;
use crate::FromRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Heartbeat {}

impl FromRequest for Heartbeat {
    fn from_request(request: &Request) -> Self {
        serde_json::from_str(&request.0).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BootNotification {
    pub reason: enums::BootReason,
    pub charging_station: types::ChargingStation,
}

impl FromRequest for BootNotification {
    fn from_request(request: &Request) -> Self {
        serde_json::from_str(&request.0).unwrap()
    }
}

pub mod enums {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq)]
    pub enum Action {
        BootNotification,
        Heartbeat,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum BootReason {
        ApplicationReset,
        FirmwareUpdate,
        LocalReset,
        PowerUp,
        RemoteReset,
        ScheduledReset,
        Triggered,
        Unknown,
        Watchdog,
    }
}

mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct ChargingStation {
        pub serial_number: Option<String>,
        pub model: String,
        pub vendor_name: String,
        pub firmware_version: Option<String>,
        modem: Option<Modem>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct Modem {
        iccid: Option<String>,
        imsi: Option<String>,
    }
}
