// #![allow(clippy::upper_case_acronyms)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr)]
pub enum StateOfOperation {
    Off = 0,
    LowPower = 1,
    Fault = 2,
    Bulk = 3,
    Absorption = 4,
    Float = 5,
    Storage = 6,
    Equalize = 7,
    Inverting = 9,
    PowerSupply = 11,
    StartingUp = 245,
    RepeatedAbsorption = 246,
    AutoEqualize = 247,
    BatterySafe = 248,
    ExternalControl = 252,
}

impl Default for StateOfOperation {
    fn default() -> Self {
        StateOfOperation::Off
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr)]
pub enum ErrorCode {
    None = 0,
    BatteryVoltageTooHigh = 2,
    ChargerTemperatureTooHigh = 17,
    ChargerOverCurrent = 18,
    ChargerCurrentReversed = 19,
    BulkTimeLimitExceeded = 20,
    CurrentSensorIssue = 21,
    TerminalsOverheated = 26,
    ConverterIssue = 28,
    InputVoltageTooHigh = 33,
    InputCurrentTooHigh = 34,
    InputShutdownBatVoltage = 38,
    InputShutdownCurrentFlow = 39,
    LostComWithDevices = 65,
    SynchronisedChargingIssue = 66,
    BMSConnectionLost = 67,
    NetworkMisconfigured = 68,
    FactoryCalibrationDataLost = 116,
    InvalidFirmware = 117,
    UserSettingsInvalid = 119,
}

impl Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::None
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr, EnumIter, Copy)]
pub enum OffReason {
    None = 0,
    NoInputPower = 1,
    SwitchedOffPowerSwitch = 2,
    SwitchedOffDMR = 4,
    RemoteInput = 8,
    ProtectionActive = 16,
    Paygo = 32,
    BMS = 64,
    EngineShutdownDetection = 128,
    AnalysingInputVoltage = 256,
}

impl Default for OffReason {
    fn default() -> Self {
        OffReason::None
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr, EnumIter, Copy)]
pub enum AlarmReason {
    None = 0,
    LowVoltage = 1,
    HighVoltage = 2,
    LowSOC = 4,
    LowStarterVoltage = 8,
    HighStarterVoltage = 16,
    LowTemperature = 32,
    HighTemperature = 64,
    MidVoltage = 128,
    Overload = 256,
    DCripple = 512,
    LowVACout = 1024,
    HighVACout = 2048,
}

impl Default for AlarmReason {
    fn default() -> Self {
        AlarmReason::None
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr, EnumIter, Copy)]
pub enum WarningReason {
    None = 0,
    LowVoltage = 1,
    HighVoltage = 2,
    LowSOC = 4,
    LowStarterVoltage = 8,
    HighStarterVoltage = 16,
    LowTemperature = 32,
    HighTemperature = 64,
    MidVoltage = 128,
    Overload = 256,
    DCripple = 512,
    LowVACout = 1024,
    HighVACout = 2048,
}

impl Default for WarningReason {
    fn default() -> Self {
        WarningReason::None
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr)]
pub enum DeviceMode {
    None = 0,
    VE_REG_MODE_INVERTER = 2,
    VE_REG_MODE_OFF = 4,
    VE_REG_MODE_ECO = 5,
}

impl Default for DeviceMode {
    fn default() -> Self {
        DeviceMode::None
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr)]
pub enum BluetoothStatus {
    Off = 0,
    On = 1,
}
impl Default for BluetoothStatus {
    fn default() -> Self {
        BluetoothStatus::Off
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr)]
pub enum BluetoothCapBle {
    None = 0,
    BLE_Supports_Switching_Off = 1,
    BLE_Switching_Off_Is_Permanent = 2,
}
impl Default for BluetoothCapBle {
    fn default() -> Self {
        BluetoothCapBle::None
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr)]
pub enum Load {
    Off = 0,
    On = 1,
}

impl Default for Load {
    fn default() -> Self {
        Load::Off
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr)]
pub enum Alarm {
    Off = 0,
    On = 1,
}
impl Default for Alarm {
    fn default() -> Self {
        Alarm::Off
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, FromRepr)]
pub enum Relay {
    Off = 0,
    On = 1,
}
impl Default for Relay {
    fn default() -> Self {
        Relay::Off
    }
}

#[allow(non_camel_case_types)]
#[derive(Display)]
pub enum Labels {
    V,     //                           [mV] Main (battery) voltage
    VS,    //                           [mV] Auxiliary (starter) voltage
    VM,    //                           [mV] Mid-point voltage of the battery bank
    DM,    //                           [‰] Mid-point deviation of the battery bank
    VPV,   //                           [mV] Panel voltage
    PPV,   //                           [W] Panel power
    I,     //                           [mA] Battery current
    IL,    //                           [mA] Load current
    LOAD,  //                           Load output state (ON/OFF)
    T,     //                           [°C] Battery temperature
    P,     //                           [W] Instantaneous power
    CE,    //                           [mAh] Consumed Amp Hours
    SOC,   //                           [‰] State-of-charge
    TTG,   //                           [Minutes] Time-to-go
    Alarm, //                           Alarm condition active
    Relay, //                           Relay state
    AR,    //                           Alarm reason
    OR,    //                           Off reason
    H1,    //                           [mAh] Depth of the deepest discharge
    H2,    //                           [mAh] Depth of the last discharge
    H3,    //                           [mAh] Depth of the average discharge
    H4,    //                           Number of charge cycles
    H5,    //                           Number of full discharges
    H6,    //                           [mAh] Cumulative Amp Hours drawn
    H7,    //                           [mV] Minimum main (battery) voltage
    H8,    //                           [mV] Maximum main (battery) voltage
    H9,    //                           [Seconds] Number of seconds since last full charge
    H10,   //                           Number of automatic synchronizations
    H11,   //                           Number of low main voltage alarms
    H12,   //                           Number of high main voltage alarms
    H13,   //                           Number of low auxiliary voltage alarms
    H14,   //                           Number of high auxiliary voltage alarms
    H15,   //                           [mV] Minimum auxiliary (battery) voltage
    H16,   //                           [mV] Maximum auxiliary (battery) voltage
    H17,   //                           [0.01 kWh] Amount of discharged energy
    H18,   //                           [0.01 kWh] Amount of charged energy
    H19,   //                           [0.01 kWh] Yield total (user resettable counter)
    H20,   //                           [0.01 kWh] Yield today
    H21,   //                           [W] Maximum power today
    H22,   //                           [0.01 kWh] Yield yesterday
    H23,   //                           [W] Maximum power yesterday
    ERR,   //                           Error code
    CS,    //                           State of operation
    BMV,   //                           Model description (deprecated)
    FW,    //                           Firmware version
    FWE,   //                           Firmware version
    PID,   //                           Product ID
    #[strum(serialize = "SER#")]
    SER, //                             Serial number
    HSDS,  //                           Day sequence number (0..364)
    MODE,  //                           Device mode
    AC_OUT_V, //                        [0.01 V] AC output voltage
    AC_OUT_I, //                        [0.1 A] AC output current
    AC_OUT_S, //                        [W] AC output power
    WARN,  //                           Warning reason
    BLE,   //                           Bluetooth status
    CAP_BLE, //                         Bloetooth capabilities
    Checksum, //                        Checksum from packet
    Calc_sum, //                        Checksum calculated by parser
    Unknown, //                         Unknown labels vector
    Time,  //                           Timestamp
}

#[allow(non_snake_case)]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct VEDirectData {
    pub V: Option<f64>,
    pub VS: Option<f64>,
    pub VM: Option<f64>,
    pub DM: Option<f64>,
    pub VPV: Option<f64>,
    pub PPV: Option<f64>,
    pub I: Option<f64>,
    pub IL: Option<f64>,
    pub LOAD: Option<Load>,
    pub T: Option<f64>,
    pub P: Option<f64>,
    pub CE: Option<f64>,
    pub SOC: Option<f64>,
    pub TTG: Option<f64>,
    pub Alarm: Option<Alarm>,
    pub Relay: Option<Relay>,
    pub AR: Option<Vec<AlarmReason>>,
    pub OR: Option<Vec<OffReason>>,
    pub H1: Option<f64>,
    pub H2: Option<f64>,
    pub H3: Option<f64>,
    pub H4: Option<f64>,
    pub H5: Option<f64>,
    pub H6: Option<f64>,
    pub H7: Option<f64>,
    pub H8: Option<f64>,
    pub H9: Option<f64>,
    pub H10: Option<f64>,
    pub H11: Option<f64>,
    pub H12: Option<f64>,
    pub H13: Option<f64>,
    pub H14: Option<f64>,
    pub H15: Option<f64>,
    pub H16: Option<f64>,
    pub H17: Option<f64>,
    pub H18: Option<f64>,
    pub H19: Option<f64>,
    pub H20: Option<f64>,
    pub H21: Option<f64>,
    pub H22: Option<f64>,
    pub H23: Option<f64>,
    pub ERR: Option<ErrorCode>,
    pub CS: Option<StateOfOperation>,
    pub BMV: Option<String>,
    pub FW: Option<String>,
    pub FWE: Option<String>,
    pub PID: Option<String>,
    pub SER: Option<String>,
    pub HSDS: Option<f64>,
    pub MODE: Option<DeviceMode>,
    pub AC_OUT_V: Option<f64>,
    pub AC_OUT_I: Option<f64>,
    pub AC_OUT_S: Option<f64>,
    pub WARN: Option<Vec<WarningReason>>,
    pub Calc_sum: Option<u8>,
    pub Checksum: Option<u8>,
    pub BLE: Option<BluetoothStatus>,
    pub CAP_BLE: Option<BluetoothCapBle>,
    pub Time: Option<i64>,
    pub Unknown: Option<Vec<String>>,
}

pub type V = f64; // volt
pub type A = f64; // ampere
pub type Ah = f64; // ampere hour
pub type Wh = f64; // watt hour

#[allow(non_camel_case_types)]
pub type mV = f64; // mili volt
#[allow(non_camel_case_types)]
pub type cV = f64; // centy volt
#[allow(non_camel_case_types)]
pub type mA = f64; // mili ampere
#[allow(non_camel_case_types)]
pub type dA = f64; // deci ampere
#[allow(non_camel_case_types)]
pub type mAh = f64; // mili ampere hour
#[allow(non_camel_case_types)]
pub type daWh = f64; // deca watt hour
#[allow(non_camel_case_types)]
pub type kWh = f64; // kilo watt hour

pub fn convert_mv(str: String) -> Result<V> {
    let val = str.parse::<mV>().map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    Ok(val / 1000_f64)
}

pub fn convert_ma(str: String) -> Result<A> {
    let val = str.parse::<mA>().map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    Ok(val / 1000_f64)
}

pub fn convert_mah(str: String) -> Result<Ah> {
    let val = str.parse::<mAh>().map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    Ok(val / 1000_f64)
}

pub fn convert_kwh(str: String) -> Result<Wh> {
    let val = str.parse::<kWh>().map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    Ok(val * 1000_f64)
}

pub fn convert_dawh(str: String) -> Result<Wh> {
    let val = str.parse::<daWh>().map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    Ok(val * 10_f64)
}

pub fn convert_cv(str: String) -> Result<V> {
    let val = str.parse::<cV>().map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    Ok(val / 100_f64)
}

pub fn convert_da(str: String) -> Result<A> {
    let val = str.parse::<dA>().map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    Ok(val / 10_f64)
}

pub fn convert_none<T>(val: T) -> Result<T> {
    Ok(val)
}

pub fn convert_parse<T>(str: String) -> Result<T>
where
    T: FromStr,
{
    str.parse::<T>().map_err(|_| Error::from(ErrorKind::InvalidData))
}

pub fn convert_minutes(str: String) -> Result<f64> {
    let val = str.parse::<f64>().map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    Ok(val * 60_f64)
}

pub fn convert_state_of_operation(field: String) -> Result<StateOfOperation> {
    match StateOfOperation::from_repr(field.parse::<usize>().map_err(|_| Error::from(ErrorKind::InvalidData))?) {
        Some(v) => Ok(v),
        None => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn convert_error_code(field: String) -> Result<ErrorCode> {
    match ErrorCode::from_repr(field.parse::<usize>().map_err(|_| Error::from(ErrorKind::InvalidData))?) {
        Some(v) => Ok(v),
        None => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn convert_alarm_reason(field: String) -> Result<Vec<AlarmReason>> {
    let val = field.parse::<usize>().map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut ret = Vec::<AlarmReason>::new();

    if val == 0 {
        ret.push(AlarmReason::None);
        return Ok(ret);
    }

    for reason in AlarmReason::iter() {
        if val & (reason as usize) > 0 {
            ret.push(reason)
        }
    }

    if !ret.is_empty() {
        Ok(ret)
    } else {
        Err(Error::from(ErrorKind::InvalidData))
    }
}

pub fn convert_warning_reason(field: String) -> Result<Vec<WarningReason>> {
    let val = field.parse::<usize>().map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut ret = Vec::<WarningReason>::new();

    if val == 0 {
        ret.push(WarningReason::None);
        return Ok(ret);
    }

    for reason in WarningReason::iter() {
        if val & (reason as usize) > 0 {
            ret.push(reason)
        }
    }

    if !ret.is_empty() {
        Ok(ret)
    } else {
        Err(Error::from(ErrorKind::InvalidData))
    }
}

pub fn convert_device_mode(field: String) -> Result<DeviceMode> {
    match DeviceMode::from_repr(field.parse::<usize>().map_err(|_| Error::from(ErrorKind::InvalidData))?) {
        Some(v) => Ok(v),
        None => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn convert_off_reason(field: String) -> Result<Vec<OffReason>> {
    let without_prefix = field.trim_start_matches("0x");
    let val = usize::from_str_radix(without_prefix, 16).map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut ret = Vec::<OffReason>::new();

    if val == 0 {
        ret.push(OffReason::None);
        return Ok(ret);
    }

    for reason in OffReason::iter() {
        if val & (reason as usize) > 0 {
            ret.push(reason)
        }
    }

    if !ret.is_empty() {
        Ok(ret)
    } else {
        Err(Error::from(ErrorKind::InvalidData))
    }
}

pub fn convert_ble(field: String) -> Result<BluetoothStatus> {
    let without_prefix = field.trim_start_matches("0x");
    let val = usize::from_str_radix(without_prefix, 16).map_err(|_| Error::from(ErrorKind::InvalidData))?;
    match BluetoothStatus::from_repr(val) {
        Some(v) => Ok(v),
        None => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn convert_capble(field: String) -> Result<BluetoothCapBle> {
    let without_prefix = field.trim_start_matches("0x");
    let val = usize::from_str_radix(without_prefix, 16).map_err(|_| Error::from(ErrorKind::InvalidData))?;
    match BluetoothCapBle::from_repr(val) {
        Some(v) => Ok(v),
        None => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn convert_alarm(field: String) -> Result<Alarm> {
    let lower = field.to_lowercase();
    if lower == "on" {
        Ok(Alarm::On)
    } else if lower == "off" {
        Ok(Alarm::Off)
    } else {
        Err(Error::from(ErrorKind::InvalidData))
    }
}

pub fn convert_relay(field: String) -> Result<Relay> {
    let lower = field.to_lowercase();
    if lower == "on" {
        Ok(Relay::On)
    } else if lower == "off" {
        Ok(Relay::Off)
    } else {
        Err(Error::from(ErrorKind::InvalidData))
    }
}

pub fn convert_load(field: String) -> Result<Load> {
    let lower = field.to_lowercase();
    if lower == "on" {
        Ok(Load::On)
    } else if lower == "off" {
        Ok(Load::Off)
    } else {
        Err(Error::from(ErrorKind::InvalidData))
    }
}
