use std::any::TypeId;
use std::collections::HashMap;

use crate::handler::Handler;
use crate::request::*;
use crate::response::IntoResponse;
use ocpp::call::Payload;
use ocpp::call_error::CallError;
use ocpp::call_result::CallResult;
use ocpp::v16::authorize::Authorize;
use ocpp::Message;

pub struct Router {
    routes: HashMap<TypeId, Box<dyn Handler<Response = Box<dyn IntoResponse>>>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Default::default(),
        }
    }

    pub fn route<H: Handler<Response = Box<dyn IntoResponse>> + 'static>(
        mut self,
        handler: H,
    ) -> Self
    where
        H: Handler,
    {
        let routing_key = handler.routing_key();
        if self.routes.contains_key(&routing_key) {
            panic!("Route already exists");
        }
        self.routes.insert(routing_key, Box::new(handler));
        self
    }

    pub fn call(&self, req: &Request) -> Result<CallResult, CallError> {
        use ocpp::v16::boot_notification::BootNotification;
        let type_id = match &req.0 {
            Message::Call(call) => match &call.payload {
                Payload::Authorize(_) => match req.1 {
                    Source::TPBE => TypeId::of::<(Call<Authorize, TPBE>,)>(),
                    Source::CGW => TypeId::of::<(Call<Authorize, CGW>,)>(),
                    Source::Charger => TypeId::of::<(Call<Authorize, Charger>,)>(),
                },
                Payload::BootNotification(_) => match req.1 {
                    Source::TPBE => TypeId::of::<(Call<BootNotification, TPBE>,)>(),
                    Source::CGW => TypeId::of::<(Call<BootNotification, CGW>,)>(),
                    Source::Charger => TypeId::of::<(Call<Authorize, Charger>,)>(),
                },
                Payload::CancelReservation(_) => todo!(),
                Payload::ChangeAvailability(_) => todo!(),
                Payload::ChangeConfiguration(_) => todo!(),
                Payload::ClearCache(_) => todo!(),
                Payload::ClearChargingProfile(_) => todo!(),
                Payload::DataTransfer(_) => todo!(),
                Payload::DiagnosticsStatusNotification(_) => todo!(),
                Payload::FirmwareStatusNotification(_) => todo!(),
                Payload::GetCompositeSchedule(_) => todo!(),
                Payload::GetConfiguration(_) => todo!(),
                Payload::GetDiagnostics(_) => todo!(),
                Payload::GetLocalListVersion(_) => todo!(),
                Payload::Heartbeat(_) => todo!(),
                Payload::MeterValues(_) => todo!(),
                Payload::RemoteStartTransaction(_) => todo!(),
                Payload::RemoteStopTransaction(_) => todo!(),
                Payload::ReserveNow(_) => todo!(),
                Payload::Reset(_) => todo!(),
                Payload::SendLocalList(_) => todo!(),
                Payload::SetChargingProfile(_) => todo!(),
                Payload::StartTransaction(_) => todo!(),
                Payload::StatusNotification(_) => todo!(),
                Payload::StopTransaction(_) => todo!(),
                Payload::TriggerMessage(_) => todo!(),
                Payload::UnlockConnector(_) => todo!(),
                Payload::UpdateFirmware(_) => todo!(),
            },
            Message::CallResult(_) => todo!(),
            Message::PartialCallResult(_) => todo!(),
            Message::CallError(_) => todo!(),
        };

        match self.routes.get(&type_id) {
            Some(handler) => handler.call(req).into_response(),
            None => Err(CallError::new(
                "123".to_string(),
                "InternalError".to_string(),
                "".to_string(),
                HashMap::default(),
            )),
        }
    }
}
