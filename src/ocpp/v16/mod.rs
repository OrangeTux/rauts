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

impl From<Action> for String {
    fn from(val: Action) -> Self {
        match val {
            Action::Authorize => "Authorize",
            Action::BootNotification => "BootNotification",
            Action::CancelReservation => "CancelReservation",
            Action::ChangeAvailability => "ChangeAvailability",
            Action::ChangeConfiguration => "ChangeConfiguration",
            Action::ClearCache => "ClearCache",
            Action::DataTransfer => "DataTransfer",
            Action::DiagnosticsStatusNotification => "DiagnosticsStatusNotification",
            Action::GetCompositeSchedule => "GetCompositeSchedule",
            Action::GetDiagnostics => "GetDiagnostics",
            Action::GetLocalListVersion => "GetLocalListVersion",
            Action::FirmwareStatusNotification => "FirmwareStatusNotification",
            Action::Heartbeat => "Heartbeat",
            Action::MeterValues => "MeterValues",
            Action::RemoteStartTransaction => "RemoteStartTransaction",
            Action::RemoteStopTransaction => "RemoteStopTransaction",
            Action::ReserveNow => "ReserveNow",
            Action::Reset => "Reset",
            Action::SendLocalList => "SendLocalList",
            Action::SetChargingProfile => "SetChargingProfile",
            Action::StartTransaction => "StartTransaction",
            Action::StopTransaction => "StopTransaction",
            Action::TriggerMessage => "TriggerMessage",
            Action::UnlockConnector => "UnlockConnector",
            Action::UpdateFirmware => "UpdateFirmware",
        }
        .to_string()
    }
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

impl From<UniqueId> for String {
    fn from(value: UniqueId) -> Self {
        value.0
    }
}

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
