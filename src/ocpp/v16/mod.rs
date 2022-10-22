pub mod call;
pub mod call_error;
pub mod call_result;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Action {
    Authorize,
    BootNotification,
    CancelReservation,
    ChangeAvailability,
    ChangeConfiguration,
    ClearCache,
    DataTransfer,
    DiagnosticsStatusNotification,
    GetCompositeSchedule,
    GetDiagnostics,
    GetLocalListVersion,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    RemoteStartTransaction,
    RemoteStopTransaction,
    ReserveNow,
    Reset,
    SendLocalList,
    SetChargingProfile,
    StartTransaction,
    StopTransaction,
    TriggerMessage,
    UnlockConnector,
    UpdateFirmware,
}

impl TryFrom<&String> for Action {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let variant = match value.as_str() {
            "Authorize" => Self::Authorize,
            "BootNotification" => Self::BootNotification,
            "CancelReservation" => Self::CancelReservation,
            "ChangeAvailability" => Self::ChangeAvailability,
            "ChangeConfiguration" => Self::ChangeConfiguration,
            "ClearCache" => Self::ClearCache,
            "DataTransfer" => Self::DataTransfer,
            "DiagnosticsStatusNotification" => Self::DiagnosticsStatusNotification,
            "GetCompositeSchedule" => Self::GetCompositeSchedule,
            "GetDiagnostics" => Self::GetDiagnostics,
            "GetLocalListVersion" => Self::GetLocalListVersion,
            "FirmwareStatusNotification" => Self::FirmwareStatusNotification,
            "Heartbeat" => Self::Heartbeat,
            "MeterValues" => Self::MeterValues,
            "RemoteStartTransaction" => Self::RemoteStartTransaction,
            "RemoteStopTransaction" => Self::RemoteStopTransaction,
            "ReserveNow" => Self::ReserveNow,
            "Reset" => Self::Reset,
            "SendLocalList" => Self::SendLocalList,
            "SetChargingProfile" => Self::SetChargingProfile,
            "StartTransaction" => Self::StartTransaction,
            "StopTransaction" => Self::StopTransaction,
            "TriggerMessage" => Self::TriggerMessage,
            "UnlockConnector" => Self::UnlockConnector,
            "UpdateFirmware" => Self::UpdateFirmware,
            _ => {
                todo!("Create proper error");
            }
        };

        Ok(variant)
    }
}

/// `UniqueId` models the unique id present in every OCPP message.
#[derive(Clone, Debug, PartialEq)]
pub struct UniqueId(String);

impl UniqueId {
    /// Generate a new `UniqueId`.
    fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for UniqueId {
    /// Generate a new `UniqueId`.
    fn default() -> Self {
        Self::generate()
    }
}
