use crate::converter::convert;
use crate::converter::models::{Alarm, AlarmReason, ErrorCode, Load, Relay, StateOfOperation};
use crate::parser::Parser;

#[test]
fn pipe_line1() {
    let data = "\r\nPID\t0xA053\r\nFW\t159\r\nSER#\tHQ2132QY2KR\r\nV\t12540\r\nI\t40\r\nVPV\t18540\r\nPPV\t5\r\nCS\t3\r\nERR\t0\r\nLOAD\tON\r\nIL\t300\r\nH19\t144\r\nH20\t1\r\nH21\t6\r\nH22\t4\r\nH23\t14\r\nHSDS\t16\r\nChecksum\t?"
        .as_bytes();
    let mut parser = Parser::new();
    let parse = parser.parse_slice(data).expect("parsing failed");
    println!("{:?}", parse);
    let conv = convert(parse).unwrap();
    println!("{:?}", conv);
    assert_eq!(conv.PID.unwrap(), "0xA053".to_string());
    assert_eq!(conv.FW.unwrap(), "159".to_string());
    assert_eq!(conv.SER.unwrap(), "HQ2132QY2KR".to_string());
    assert_eq!(conv.V.unwrap(), 12.54);
    assert_eq!(conv.I.unwrap(), 0.04);
    assert_eq!(conv.VPV.unwrap(), 18.54);
    assert_eq!(conv.PPV.unwrap(), 5.0);
    assert_eq!(conv.CS.unwrap(), StateOfOperation::Bulk);
    assert_eq!(conv.ERR.unwrap(), ErrorCode::None);
    assert_eq!(conv.LOAD.unwrap(), Load::On);
    assert_eq!(conv.IL.unwrap(), 0.3);
    assert_eq!(conv.H19.unwrap(), 1440.0);
    assert_eq!(conv.H20.unwrap(), 10.0);
    assert_eq!(conv.H21.unwrap(), 6.0);
    assert_eq!(conv.H22.unwrap(), 40.0);
    assert_eq!(conv.H23.unwrap(), 14.0);
    assert_eq!(conv.HSDS.unwrap(), 16.0);
}

#[test]
fn pipe_line2() {
    let data = "\r\nPID\t0x203\r\nV\t26201\r\nI\t0\r\nP\t0\r\nCE\t0\r\n:A243434\r\nSOC\t1000\r\nTTG\t-1\r\nAlarm\tOFF\r\nRelay\tOFF\r\nAR\t0\r\nBMV\t700\r\nFW\t0307\r\nChecksum\t"
        .as_bytes();
    let mut vec_data = data.to_vec();
    vec_data.push(0xd8);

    let mut parser = Parser::new();
    let parse = parser.parse_slice(vec_data.as_slice()).expect("parsing failed");
    let conv = convert(parse).unwrap();
    assert_eq!(conv.PID.unwrap(), "0x203".to_string());
    assert_eq!(conv.V.unwrap(), 26.201);
    assert_eq!(conv.I.unwrap(), 0.0);
    assert_eq!(conv.P.unwrap(), 0.0);
    assert_eq!(conv.CE.unwrap(), 0.0);
    assert_eq!(conv.SOC.unwrap(), 1000.0);
    assert_eq!(conv.TTG.unwrap(), -60.0);
    assert_eq!(conv.Alarm.unwrap(), Alarm::Off);
    assert_eq!(conv.Relay.unwrap(), Relay::Off);
    assert_eq!(conv.AR.unwrap(), [AlarmReason::None]);
    assert_eq!(conv.BMV.unwrap(), "700".to_string());
    assert_eq!(conv.FW.unwrap(), "0307".to_string());
    assert_eq!(conv.Checksum.unwrap(), 0xd8);
}
