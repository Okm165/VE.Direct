use super::convert;
use crate::converter::models::*;
use std::collections::HashMap;

#[test]
fn si_converters_test() {
    assert_eq!(convert_mv("2000".to_string()).unwrap(), 2 as V);
    assert_eq!(convert_ma("2000".to_string()).unwrap(), 2 as A);
    assert_eq!(convert_mah("2000".to_string()).unwrap(), 2 as Ah);
    assert_eq!(convert_kwh("2000".to_string()).unwrap(), 2000000 as Wh);
}

#[test]
fn state_of_operation_test() {
    assert_eq!(convert_state_of_operation("0".to_string()).unwrap(), StateOfOperation::Off);
    assert_eq!(convert_state_of_operation("5".to_string()).unwrap(), StateOfOperation::Float);
    assert_eq!(convert_state_of_operation("245".to_string()).unwrap(), StateOfOperation::StartingUp);
    assert_eq!(convert_state_of_operation("3".to_string()).unwrap(), StateOfOperation::Bulk);
    convert_state_of_operation("-1120".to_string()).expect_err("");
    convert_state_of_operation("1120".to_string()).expect_err("");
}

#[test]
fn convert_error_code_test() {
    assert_eq!(convert_error_code("0".to_string()).unwrap(), ErrorCode::None);
    convert_error_code("5".to_string()).expect_err("");
    assert_eq!(convert_error_code("2".to_string()).unwrap(), ErrorCode::BatteryVoltageTooHigh);
    assert_eq!(convert_error_code("34".to_string()).unwrap(), ErrorCode::InputCurrentTooHigh);
    convert_error_code("-1120".to_string()).expect_err("");
    convert_error_code("1120".to_string()).expect_err("");
}

#[test]
fn convert_alarm_reason_test() {
    assert_eq!(convert_alarm_reason("0".to_string()).unwrap(), [AlarmReason::None]);
    assert_eq!(convert_alarm_reason("5".to_string()).unwrap(), [AlarmReason::LowVoltage, AlarmReason::LowSOC]);
    assert_eq!(convert_alarm_reason("2".to_string()).unwrap(), [AlarmReason::HighVoltage]);
    assert_eq!(convert_alarm_reason("34".to_string()).unwrap(), [AlarmReason::HighVoltage, AlarmReason::LowTemperature]);
    convert_alarm_reason("-1120".to_string()).expect_err("");
}

#[test]
fn convert_device_mode_test() {
    assert_eq!(convert_device_mode("0".to_string()).unwrap(), DeviceMode::None);
    assert_eq!(convert_device_mode("2".to_string()).unwrap(), DeviceMode::VE_REG_MODE_INVERTER);
    assert_eq!(convert_device_mode("4".to_string()).unwrap(), DeviceMode::VE_REG_MODE_OFF);
    assert_eq!(convert_device_mode("5".to_string()).unwrap(), DeviceMode::VE_REG_MODE_ECO);
    convert_device_mode("34".to_string()).expect_err("");
    convert_device_mode("-1120".to_string()).expect_err("");
}

#[test]
fn convert_device_off_reason() {
    assert_eq!(convert_off_reason("0x00000000".to_string()).unwrap(), [OffReason::None]);
    assert_eq!(convert_off_reason("0x00000010".to_string()).unwrap(), [OffReason::ProtectionActive]);
    assert_eq!(convert_off_reason("0x00000020".to_string()).unwrap(), [OffReason::Paygo]);
    assert_eq!(convert_off_reason("0x00000021".to_string()).unwrap(), [OffReason::NoInputPower, OffReason::Paygo]);
    assert_eq!(convert_off_reason("0x00000042".to_string()).unwrap(), [OffReason::SwitchedOffPowerSwitch, OffReason::BMS]);
    convert_off_reason("0x000f0000".to_string()).expect_err("");
    convert_off_reason("-12".to_string()).expect_err("");
}

#[test]
fn convert_ble_test() {
    assert_eq!(convert_ble("0x00000000".to_string()).unwrap(), BluetoothStatus::Off);
    assert_eq!(convert_ble("0x00000001".to_string()).unwrap(), BluetoothStatus::On);
    convert_ble("0x000f0000".to_string()).expect_err("");
    convert_ble("-12".to_string()).expect_err("");
}

#[test]
fn convert_capble_test() {
    assert_eq!(convert_capble("0x00000000".to_string()).unwrap(), BluetoothCapBle::None);
    assert_eq!(convert_capble("0x00000001".to_string()).unwrap(), BluetoothCapBle::BLE_Supports_Switching_Off);
    assert_eq!(convert_capble("0x00000002".to_string()).unwrap(), BluetoothCapBle::BLE_Switching_Off_Is_Permanent);
    convert_capble("0x000f0000".to_string()).expect_err("");
    convert_capble("-12".to_string()).expect_err("");
}

#[test]
fn convert_alarm_test() {
    assert_eq!(convert_alarm("OFF".to_string()).unwrap(), Alarm::Off);
    assert_eq!(convert_alarm("On".to_string()).unwrap(), Alarm::On);
}

#[test]
fn convert_relay_test() {
    assert_eq!(convert_relay("Off".to_string()).unwrap(), Relay::Off);
    assert_eq!(convert_relay("On".to_string()).unwrap(), Relay::On);
}

#[test]
fn convert_load_test() {
    assert_eq!(convert_load("off".to_string()).unwrap(), Load::Off);
    assert_eq!(convert_load("On".to_string()).unwrap(), Load::On);
}

#[test]
fn convert_test() {
    let mut map = HashMap::<String, Vec<u8>>::new();
    map.insert("V".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("VS".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("VM".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("DM".to_string(), "0.012".as_bytes().to_vec());
    map.insert("VPV".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("PPV".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("I".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("IL".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("LOAD".to_string(), "ON".as_bytes().to_vec());
    map.insert("T".to_string(), "43.233".as_bytes().to_vec());
    map.insert("P".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("CE".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("SOC".to_string(), "23.12".as_bytes().to_vec());
    map.insert("TTG".to_string(), "23.12".as_bytes().to_vec());
    map.insert("Alarm".to_string(), "On".as_bytes().to_vec());
    map.insert("Relay".to_string(), "OFF".as_bytes().to_vec());
    map.insert("AR".to_string(), "8".as_bytes().to_vec());
    map.insert("OR".to_string(), "4".as_bytes().to_vec());
    map.insert("H1".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("H2".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("H3".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("H4".to_string(), "12".as_bytes().to_vec());
    map.insert("H5".to_string(), "12".as_bytes().to_vec());
    map.insert("H6".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("H7".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("H8".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("H9".to_string(), "1233".as_bytes().to_vec());
    map.insert("H10".to_string(), "554".as_bytes().to_vec());
    map.insert("H11".to_string(), "554".as_bytes().to_vec());
    map.insert("H12".to_string(), "554".as_bytes().to_vec());
    map.insert("H13".to_string(), "554".as_bytes().to_vec());
    map.insert("H14".to_string(), "554".as_bytes().to_vec());
    map.insert("H15".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("H16".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("H17".to_string(), "12488".as_bytes().to_vec());
    map.insert("H18".to_string(), "12488".as_bytes().to_vec());
    map.insert("H19".to_string(), "12488".as_bytes().to_vec());
    map.insert("H20".to_string(), "12488".as_bytes().to_vec());
    map.insert("H21".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("H22".to_string(), "12488".as_bytes().to_vec());
    map.insert("H23".to_string(), "12488.12".as_bytes().to_vec());
    map.insert("ERR".to_string(), "26".as_bytes().to_vec());
    map.insert("CS".to_string(), "3".as_bytes().to_vec());
    map.insert("BMV".to_string(), "STRING".as_bytes().to_vec());
    map.insert("FW".to_string(), "STRING".as_bytes().to_vec());
    map.insert("PID".to_string(), "STRING".as_bytes().to_vec());
    map.insert("SER#".to_string(), "STRING".as_bytes().to_vec());
    map.insert("HSDS".to_string(), "42".as_bytes().to_vec());
    map.insert("MODE".to_string(), "5".as_bytes().to_vec());
    map.insert("AC_OUT_V".to_string(), "12488".as_bytes().to_vec());
    map.insert("AC_OUT_I".to_string(), "12488".as_bytes().to_vec());
    map.insert("AC_OUT_S".to_string(), "12488".as_bytes().to_vec());
    map.insert("WARN".to_string(), "32".as_bytes().to_vec());
    map.insert("Calc_sum".to_string(), "35".as_bytes().to_vec());
    map.insert("Checksum".to_string(), "255".as_bytes().to_vec());
    map.insert("BLE".to_string(), "0".as_bytes().to_vec());
    map.insert("CAP_BLE".to_string(), "2".as_bytes().to_vec());
    map.insert("Time".to_string(), "12344556789".as_bytes().to_vec());
    map.insert("Bleble".to_string(), "ola".as_bytes().to_vec());

    let conv = convert(map);

    assert_eq!(conv.V.unwrap(), 12.48812);
    assert_eq!(conv.VS.unwrap(), 12.48812);
    assert_eq!(conv.VM.unwrap(), 12.48812);
    assert_eq!(conv.DM.unwrap(), 0.012);
    assert_eq!(conv.VPV.unwrap(), 12.48812);
    assert_eq!(conv.PPV.unwrap(), 12488.12);
    assert_eq!(conv.I.unwrap(), 12.48812);
    assert_eq!(conv.IL.unwrap(), 12.48812);
    assert_eq!(conv.LOAD.unwrap(), Load::On);
    assert_eq!(conv.T.unwrap(), 43.233);
    assert_eq!(conv.P.unwrap(), 12488.12);
    assert_eq!(conv.CE.unwrap(), 12.48812);
    assert_eq!(conv.SOC.unwrap(), 23.12);
    assert_eq!(conv.TTG.unwrap(), 23.12 * 60_f64);
    assert_eq!(conv.Alarm.unwrap(), Alarm::On);
    assert_eq!(conv.Relay.unwrap(), Relay::Off);
    assert_eq!(conv.AR.unwrap(), vec![AlarmReason::LowStarterVoltage]);
    assert_eq!(conv.OR.unwrap(), vec![OffReason::SwitchedOffDMR]);
    assert_eq!(conv.H1.unwrap(), 12.48812);
    assert_eq!(conv.H2.unwrap(), 12.48812);
    assert_eq!(conv.H3.unwrap(), 12.48812);
    assert_eq!(conv.H4.unwrap(), 12_f64);
    assert_eq!(conv.H5.unwrap(), 12_f64);
    assert_eq!(conv.H6.unwrap(), 12.48812);
    assert_eq!(conv.H7.unwrap(), 12.48812);
    assert_eq!(conv.H8.unwrap(), 12.48812);
    assert_eq!(conv.H9.unwrap(), 1233.0);
    assert_eq!(conv.H10.unwrap(), 554_f64);
    assert_eq!(conv.H11.unwrap(), 554_f64);
    assert_eq!(conv.H12.unwrap(), 554_f64);
    assert_eq!(conv.H13.unwrap(), 554_f64);
    assert_eq!(conv.H14.unwrap(), 554_f64);
    assert_eq!(conv.H15.unwrap(), 12.48812);
    assert_eq!(conv.H16.unwrap(), 12.48812);
    assert_eq!(conv.H17.unwrap(), 124880.0);
    assert_eq!(conv.H18.unwrap(), 124880.0);
    assert_eq!(conv.H19.unwrap(), 124880.0);
    assert_eq!(conv.H20.unwrap(), 124880.0);
    assert_eq!(conv.H21.unwrap(), 12488.12);
    assert_eq!(conv.H22.unwrap(), 124880.0);
    assert_eq!(conv.H23.unwrap(), 12488.12);
    assert_eq!(conv.ERR.unwrap(), ErrorCode::TerminalsOverheated);
    assert_eq!(conv.BMV.unwrap(), "STRING".to_string());
    assert_eq!(conv.FW.unwrap(), "STRING".to_string());
    assert_eq!(conv.PID.unwrap(), "STRING".to_string());
    assert_eq!(conv.SER.unwrap(), "STRING".to_string());
    assert_eq!(conv.HSDS.unwrap(), 42_f64);
    assert_eq!(conv.MODE.unwrap(), DeviceMode::VE_REG_MODE_ECO);
    assert_eq!(conv.AC_OUT_V.unwrap(), 124.88);
    assert_eq!(conv.AC_OUT_I.unwrap(), 1248.8);
    assert_eq!(conv.AC_OUT_S.unwrap(), 12488.0);
    assert_eq!(conv.WARN.unwrap(), vec![WarningReason::LowTemperature]);
    assert_eq!(conv.Calc_sum.unwrap(), 35_u8);
    assert_eq!(conv.Checksum.unwrap(), 255_u8);
    assert_eq!(conv.BLE.unwrap(), BluetoothStatus::Off);
    assert_eq!(conv.CAP_BLE.unwrap(), BluetoothCapBle::BLE_Switching_Off_Is_Permanent);
    assert_eq!(conv.Time.unwrap(), 12344556789);
    assert_eq!(conv.Unknown.unwrap(), vec!["Bleble: ola"]);
}
