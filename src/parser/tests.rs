use super::Parser;
use crate::parser::models::VEError;

#[test]
fn parse_field() {
    let data = "\r\nPID\t0xA053\r\nFW\t159\r\nChecksum\t?\r\nFW\t159".as_bytes();
    let (field, read_pos) = Parser::parse_field(data, 0).expect("parsing failed");
    assert_eq!(field.label, "PID".to_string());
    assert_eq!(field.value, "0xA053".as_bytes());
    assert_eq!(read_pos, 12);

    let (field, read_pos) = Parser::parse_field(data, read_pos).expect("parsing failed");
    assert_eq!(field.label, "FW".to_string());
    assert_eq!(field.value, "159".as_bytes());
    assert_eq!(read_pos, 20);

    assert_eq!(Parser::parse_field(data, 19).err().unwrap(), VEError::Parse("Illegal field start".to_string()));

    let (field, read_pos) = Parser::parse_field(data, read_pos).expect("parsing failed");
    assert_eq!(field.label, "Checksum".to_string());
    assert_eq!(read_pos, 32);

    assert_eq!(Parser::parse_field(data, read_pos).err().unwrap(), VEError::NeedMoreData);
}

#[test]
fn middle_of_packet_hex_message() {
    let data = ":ADFF482323".as_bytes();
    let mut parser = Parser::new();
    assert_eq!(parser.parse_slice(data).err().unwrap(), VEError::NeedMoreData);
}

#[test]
fn hex_message() {
    let data = "\r\n:ADFF482323".as_bytes();
    let mut parser = Parser::new();
    assert_eq!(parser.parse_slice(data).err().unwrap(), VEError::NeedMoreData);
}

#[test]
fn test_complete_stream() {
    let data = "\r\nPID\t0xA053\r\nFW\t159\r\nSER#\tHQ2132QY2KR\r\nV\t12540\r\nI\t40\r\nVPV\t18540\r\nPPV\t5\r\nCS\t3\r\nERR\t0\r\nLOAD\tON\r\nIL\t300\r\nH19\t144\r\nH20\t1\r\nH21\t6\r\nH22\t4\r\nH23\t14\r\nHSDS\t16\r\nChecksum\t?"
        .as_bytes();
    let mut parser = Parser::new();
    let map = parser.parse_slice(data).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0xA053".as_bytes());
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "159".as_bytes());
    assert_eq!(map.get(&"SER#".to_string()).unwrap(), "HQ2132QY2KR".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "12540".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "40".as_bytes());
    assert_eq!(map.get(&"VPV".to_string()).unwrap(), "18540".as_bytes());
    assert_eq!(map.get(&"PPV".to_string()).unwrap(), "5".as_bytes());
    assert_eq!(map.get(&"CS".to_string()).unwrap(), "3".as_bytes());
    assert_eq!(map.get(&"ERR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"LOAD".to_string()).unwrap(), "ON".as_bytes());
    assert_eq!(map.get(&"IL".to_string()).unwrap(), "300".as_bytes());
    assert_eq!(map.get(&"H19".to_string()).unwrap(), "144".as_bytes());
    assert_eq!(map.get(&"H20".to_string()).unwrap(), "1".as_bytes());
    assert_eq!(map.get(&"H21".to_string()).unwrap(), "6".as_bytes());
    assert_eq!(map.get(&"H22".to_string()).unwrap(), "4".as_bytes());
    assert_eq!(map.get(&"H23".to_string()).unwrap(), "14".as_bytes());
    assert_eq!(map.get(&"HSDS".to_string()).unwrap(), "16".as_bytes());
}

#[test]
fn test_multiple_complete_streams() {
    let data = "2540\r\nI\t40\r\nVPV\t18540\r\nPPV\t5\r\nCS\t3\r\nERR\t0\r\nLOAD\tON\r\nIL\t300\r\nH19\t144\r\nH20\t1\r\nH1\t6\r\nH22\t4\r\nH23\t14\r\nHSDS\t16\r\nChecksum\t?\r\nPID\t0xA053\r\nFW\t159\r\nSER#\tHQ2132QY2KR\r\nV\t12540\r\nI\t110\r\nVPV\t17660\r\nPPV\t5\r\nCS\t3\r\nERR\t0\r\nLOAD\tON\r\nIL\t300\r\nH19\t144\r\nH20\t1\r\nH21\t6\r\nH22\t4\r\nH23\t14\r\nHSDS\t16\r\nChecksum\t?"
        .as_bytes();
    let mut parser = Parser::new();
    let mut map = parser.parse_slice(data).expect("parsing failed");
    assert_eq!(map.get(&"I".to_string()).unwrap(), "40".as_bytes());
    assert_eq!(map.get(&"VPV".to_string()).unwrap(), "18540".as_bytes());
    assert_eq!(map.get(&"PPV".to_string()).unwrap(), "5".as_bytes());
    assert_eq!(map.get(&"CS".to_string()).unwrap(), "3".as_bytes());
    assert_eq!(map.get(&"ERR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"LOAD".to_string()).unwrap(), "ON".as_bytes());
    assert_eq!(map.get(&"IL".to_string()).unwrap(), "300".as_bytes());
    assert_eq!(map.get(&"H19".to_string()).unwrap(), "144".as_bytes());
    assert_eq!(map.get(&"H20".to_string()).unwrap(), "1".as_bytes());
    assert_eq!(map.get(&"H1".to_string()).unwrap(), "6".as_bytes());
    assert_eq!(map.get(&"H22".to_string()).unwrap(), "4".as_bytes());
    assert_eq!(map.get(&"H23".to_string()).unwrap(), "14".as_bytes());
    assert_eq!(map.get(&"HSDS".to_string()).unwrap(), "16".as_bytes());

    map = parser.parse_slice(data).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0xA053".as_bytes());
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "159".as_bytes());
    assert_eq!(map.get(&"SER#".to_string()).unwrap(), "HQ2132QY2KR".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "12540".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "110".as_bytes());
    assert_eq!(map.get(&"VPV".to_string()).unwrap(), "17660".as_bytes());
    assert_eq!(map.get(&"PPV".to_string()).unwrap(), "5".as_bytes());
    assert_eq!(map.get(&"CS".to_string()).unwrap(), "3".as_bytes());
    assert_eq!(map.get(&"ERR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"LOAD".to_string()).unwrap(), "ON".as_bytes());
    assert_eq!(map.get(&"IL".to_string()).unwrap(), "300".as_bytes());
    assert_eq!(map.get(&"H19".to_string()).unwrap(), "144".as_bytes());
    assert_eq!(map.get(&"H20".to_string()).unwrap(), "1".as_bytes());
    assert_eq!(map.get(&"H21".to_string()).unwrap(), "6".as_bytes());
    assert_eq!(map.get(&"H22".to_string()).unwrap(), "4".as_bytes());
    assert_eq!(map.get(&"H23".to_string()).unwrap(), "14".as_bytes());
    assert_eq!(map.get(&"HSDS".to_string()).unwrap(), "16".as_bytes());
}

#[test]
fn test_multiple_complete_streams_with_hex_messages() {
    let data = "\r\nPID\t0xA053\r\nFW\t159\r\nSER#\tHQ2132QY2KR\r\nV\t12540\r\nI\t40\r\nVPV\t18540\r\nPPV\t5\r\nCS\t3\r\nERR\t0\r\nLOAD\tON\r\nIL\t300\r\nH19\t144\r\nH20\t1\r\nH21\t6\r\nH22\t4\r\nH23\t14\r\nHSDS\t16\r\nChecksum\t?:A8DED009B1323\n:A8FED002F00A0\n:AECED00CE7232\n:ADBED0004106F\n:A0320001C0804\n\r\n:A4F1000010000000000AD000000AD000000E508AE05139D04FFFFFFFFFFFFFFFFFFFFFFFFFF4A\r\n:A5010000002000000040000002405C60400000000002E01000000000E0000000A00BA071300D7\r\nPID\t0xA053\r\nFW\t159\r\nSER#\tHQ2132QY2KR\r\nV\t12540\r\nI\t110\r\nVPV\t17660\r\nPPV\t5\r\nCS\t3\r\nERR\t0\r\nLOAD\tON\r\nIL\t300\r\nH19\t144\r\nH20\t1\r\nH21\t6\r\nH22\t4\r\nH23\t14\r\nHSDS\t16\r\nChecksum\t?"
        .as_bytes();

    let mut parser = Parser::new();
    let mut map = parser.parse_slice(data).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0xA053".as_bytes());
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "159".as_bytes());
    assert_eq!(map.get(&"SER#".to_string()).unwrap(), "HQ2132QY2KR".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "12540".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "40".as_bytes());
    assert_eq!(map.get(&"VPV".to_string()).unwrap(), "18540".as_bytes());
    assert_eq!(map.get(&"PPV".to_string()).unwrap(), "5".as_bytes());
    assert_eq!(map.get(&"CS".to_string()).unwrap(), "3".as_bytes());
    assert_eq!(map.get(&"ERR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"LOAD".to_string()).unwrap(), "ON".as_bytes());
    assert_eq!(map.get(&"IL".to_string()).unwrap(), "300".as_bytes());
    assert_eq!(map.get(&"H19".to_string()).unwrap(), "144".as_bytes());
    assert_eq!(map.get(&"H20".to_string()).unwrap(), "1".as_bytes());
    assert_eq!(map.get(&"H21".to_string()).unwrap(), "6".as_bytes());
    assert_eq!(map.get(&"H22".to_string()).unwrap(), "4".as_bytes());
    assert_eq!(map.get(&"H23".to_string()).unwrap(), "14".as_bytes());
    assert_eq!(map.get(&"HSDS".to_string()).unwrap(), "16".as_bytes());

    map = parser.parse_slice(data).expect("parsing failed");
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "159".as_bytes());
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0xA053".as_bytes());
    assert_eq!(map.get(&"SER#".to_string()).unwrap(), "HQ2132QY2KR".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "12540".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "110".as_bytes());
    assert_eq!(map.get(&"VPV".to_string()).unwrap(), "17660".as_bytes());
    assert_eq!(map.get(&"PPV".to_string()).unwrap(), "5".as_bytes());
    assert_eq!(map.get(&"CS".to_string()).unwrap(), "3".as_bytes());
    assert_eq!(map.get(&"ERR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"LOAD".to_string()).unwrap(), "ON".as_bytes());
    assert_eq!(map.get(&"IL".to_string()).unwrap(), "300".as_bytes());
    assert_eq!(map.get(&"H19".to_string()).unwrap(), "144".as_bytes());
    assert_eq!(map.get(&"H20".to_string()).unwrap(), "1".as_bytes());
    assert_eq!(map.get(&"H21".to_string()).unwrap(), "6".as_bytes());
    assert_eq!(map.get(&"H22".to_string()).unwrap(), "4".as_bytes());
    assert_eq!(map.get(&"H23".to_string()).unwrap(), "14".as_bytes());
    assert_eq!(map.get(&"HSDS".to_string()).unwrap(), "16".as_bytes());
}

#[test]
fn test_incomplete_stream() {
    let data = "\r\nPID\t0xA053\r\nFW\t159\r\nSER#\tHQ2132QY2KR\r\nV\t12540\r\nI\t40\r\nVPV\t18540\r\nPPV\t5\r\nCS\t3\r\nERR\t0\r\nLOAD\tON\r\nIL\t300\r\nH19\t144\r\nH20\t1\r\nH21\t6\r\nH2"
        .as_bytes();

    let mut parser = Parser::new();
    assert_eq!(parser.parse_slice(data).unwrap_err(), VEError::NeedMoreData)
}

#[test]
fn test_incomplete_stream_with_hex_messages() {
    let data = "\r\nPID\t0xA053\r\nFW\t159\r\nSER#\tHQ2132QY2KR\r\nV\t12540\r\nI\t40\r\nVPV\t18540\r\nPPV\t5\r\nCS\t3\r\n:A4F1000010000000000AD000000AD000000E508AE05139D04FFFFFFFFFFFFFFFF\r\nERR\t0\r\nLOAD\tON\r\nIL\t300\r\nH19\t144\r\nH20\t1\r\nH21\t6\r\n:A4F1000010000000000AD000000AD000000E508AE05139D04FFFFFFFFFFFFFFFFFFFFFFFFFF4A\r\n:A4F1000010000000000AD000000AD000000E508AE05139D04FFFFFFFFFFFFFFFFFFFFFFFFFF4A"
        .as_bytes();

    let mut parser = Parser::new();
    assert_eq!(parser.parse_slice(data).unwrap_err(), VEError::NeedMoreData)
}

#[test]
fn test_incomplete_stream_then_complete() {
    let mut data = "\r\nPID\t0xA053\r\nFW\t159\r\nSER#\tH2132QY2KR\r\nV\t12540\r\nI\t40\r\nVPV\t18540\r\nPPV\t5\r\nCS\t3\r\nERR\t0\r\nLOAD\tON\r\nIL\t300\r\nH19\t144\r\nH20\t1\r\nH2"
        .as_bytes();

    let mut parser = Parser::new();
    assert_eq!(parser.parse_slice(data).unwrap_err(), VEError::NeedMoreData);

    data = "1\t6\r\nH22\t4\r\nH23\t14\r\nHSDS\t16\r\nChecksum\t?".as_bytes();
    let map = parser.parse_slice(data).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0xA053".as_bytes());
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "159".as_bytes());
    assert_eq!(map.get(&"SER#".to_string()).unwrap(), "H2132QY2KR".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "12540".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "40".as_bytes());
    assert_eq!(map.get(&"VPV".to_string()).unwrap(), "18540".as_bytes());
    assert_eq!(map.get(&"PPV".to_string()).unwrap(), "5".as_bytes());
    assert_eq!(map.get(&"CS".to_string()).unwrap(), "3".as_bytes());
    assert_eq!(map.get(&"ERR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"LOAD".to_string()).unwrap(), "ON".as_bytes());
    assert_eq!(map.get(&"IL".to_string()).unwrap(), "300".as_bytes());
    assert_eq!(map.get(&"H19".to_string()).unwrap(), "144".as_bytes());
    assert_eq!(map.get(&"H20".to_string()).unwrap(), "1".as_bytes());
    assert_eq!(map.get(&"H21".to_string()).unwrap(), "6".as_bytes());
    assert_eq!(map.get(&"H22".to_string()).unwrap(), "4".as_bytes());
    assert_eq!(map.get(&"H23".to_string()).unwrap(), "14".as_bytes());
    assert_eq!(map.get(&"HSDS".to_string()).unwrap(), "16".as_bytes());
}

#[test]
fn test_checksum() {
    let data = "\r\nPID\t0x203\r\nV\t26201\r\nI\t0\r\nP\t0\r\nCE\t0\r\nSOC\t1000\r\nTTG\t-1\r\nAlarm\tOFF\r\nRelay\tOFF\r\nAR\t0\r\nBMV\t700\r\nFW\t0307\r\nChecksum\t"
        .as_bytes();

    let mut vec_data = data.to_vec();
    vec_data.push(0xd8);

    let mut parser = Parser::new();
    let map = parser.parse_slice(vec_data.as_slice()).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0x203".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "26201".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"P".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"CE".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"SOC".to_string()).unwrap(), "1000".as_bytes());
    assert_eq!(map.get(&"TTG".to_string()).unwrap(), "-1".as_bytes());
    assert_eq!(map.get(&"Alarm".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"Relay".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"AR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"BMV".to_string()).unwrap(), "700".as_bytes());
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "0307".as_bytes());
    assert_eq!(map.get(&"Calc_sum".to_string()).unwrap(), &[0])
}

#[test]
fn test_checksum_with_hex_messages() {
    let data = "\r\nPID\t0x203\r\nV\t26201\r\nI\t0\r\nP\t0\r\nCE\t0\r\n:A243434\r\nSOC\t1000\r\nTTG\t-1\r\nAlarm\tOFF\r\nRelay\tOFF\r\nAR\t0\r\nBMV\t700\r\nFW\t0307\r\nChecksum\t"
        .as_bytes();

    let mut vec_data = data.to_vec();
    vec_data.push(0xd8);

    let mut parser = Parser::new();
    let map = parser.parse_slice(vec_data.as_slice()).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0x203".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "26201".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"P".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"CE".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"SOC".to_string()).unwrap(), "1000".as_bytes());
    assert_eq!(map.get(&"TTG".to_string()).unwrap(), "-1".as_bytes());
    assert_eq!(map.get(&"Alarm".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"Relay".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"AR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"BMV".to_string()).unwrap(), "700".as_bytes());
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "0307".as_bytes());
    assert_eq!(map.get(&"Calc_sum".to_string()).unwrap(), &[0]);
}

#[test]
fn test_checksum_with_hex_messages_incomplete_then_complete_with_checksum() {
    let mut data = "\r\n:A243\r\nPID\t0x203\r\nV\t26201\r\nI\t0\r\nP\t0\r\nCE\t0\r\n:A243".as_bytes();

    let mut parser = Parser::new();
    assert_eq!(parser.parse_slice(data).unwrap_err(), VEError::NeedMoreData);

    data = "439994\r\nSOC\t1000\r\nTTG\t-1\r\nAlarm\tOFF\r\nRelay\tOFF\r\n:A243\r\nAR\t0\r\nBMV\t700\r\nFW\t0307\r\nChecksum\t"
        .as_bytes();

    let mut vec_data = data.to_vec();
    vec_data.push(0xd8);

    let map = parser.parse_slice(vec_data.as_slice()).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0x203".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "26201".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"P".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"CE".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"SOC".to_string()).unwrap(), "1000".as_bytes());
    assert_eq!(map.get(&"TTG".to_string()).unwrap(), "-1".as_bytes());
    assert_eq!(map.get(&"Alarm".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"Relay".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"AR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"BMV".to_string()).unwrap(), "700".as_bytes());
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "0307".as_bytes());
    assert_eq!(map.get(&"Calc_sum".to_string()).unwrap(), &[0])
}

#[test]
fn test_checksum_with_hex_messages_after() {
    let data = "\r\nPID\t0x203\r\nV\t26201\r\nI\t0\r\nP\t0\r\nCE\t0\r\n:A243434\r\nSOC\t1000\r\nTTG\t-1\r\nAlarm\tOFF\r\nRelay\tOFF\r\nAR\t0\r\nBMV\t700\r\nFW\t0307\r\nChecksum\t"
        .as_bytes();
    let mut vec_data = data.to_vec();
    vec_data.push(0xd8);
    let after = ":A8DED009B1323\n:A8FED002F00A0\n:AECED00CE7232\n:ADBED0004106F\n:A0320001C0804\n:ABCED003A46000022\n:ACCEC002D3900002D\n:ACBEC004D3A0D\n:ADBEC009100F3\n"
        .as_bytes();
    vec_data.extend(after.iter());

    let mut parser = Parser::new();
    let map = parser.parse_slice(vec_data.as_slice()).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0x203".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "26201".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"P".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"CE".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"SOC".to_string()).unwrap(), "1000".as_bytes());
    assert_eq!(map.get(&"TTG".to_string()).unwrap(), "-1".as_bytes());
    assert_eq!(map.get(&"Alarm".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"Relay".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"AR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"BMV".to_string()).unwrap(), "700".as_bytes());
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "0307".as_bytes());
    assert_eq!(map.get(&"Calc_sum".to_string()).unwrap(), &[0]);
}

#[test]
fn test_checksum_multiple() {
    let data = "\r\nPID\t0x203\r\nV\t26201\r\nI\t0\r\nP\t0\r\nCE\t0\r\nSOC\t1000\r\nTTG\t-1\r\nAlarm\tOFF\r\nRelay\tOFF\r\nAR\t0\r\nBMV\t700\r\nFW\t0307\r\nChecksum\t"
        .as_bytes();

    let mut vec_data = data.to_vec();
    vec_data.push(0xd8);

    let mut parser = Parser::new();
    let map = parser.parse_slice(vec_data.as_slice()).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0x203".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "26201".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"P".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"CE".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"SOC".to_string()).unwrap(), "1000".as_bytes());
    assert_eq!(map.get(&"TTG".to_string()).unwrap(), "-1".as_bytes());
    assert_eq!(map.get(&"Alarm".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"Relay".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"AR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"BMV".to_string()).unwrap(), "700".as_bytes());
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "0307".as_bytes());
    assert_eq!(map.get(&"Calc_sum".to_string()).unwrap(), &[0]);

    let data = "\r\nPID\t0x203\r\nChecksum\t0".as_bytes();
    let vec_data = data.to_vec();

    let map = parser.parse_slice(vec_data.as_slice()).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0x203".as_bytes());
    assert_eq!(map.get(&"Calc_sum".to_string()).unwrap(), &[189]);
}

#[test]
fn test_no_checksum_fill_with_zero() {
    let data = "\r\nPID\t0x203\r\nV\t26201\r\nI\t0\r\nP\t0\r\nCE\t0\r\nSOC\t1000\r\nTTG\t-1\r\nAlarm\tOFF\r\nRelay\tOFF\r\nAR\t0\r\nBMV\t700\r\nFW\t0307\r\nChecksum\t\r\n"
        .as_bytes();

    let mut parser = Parser::new();
    let map = parser.parse_slice(data).expect("parsing failed");
    assert_eq!(map.get(&"PID".to_string()).unwrap(), "0x203".as_bytes());
    assert_eq!(map.get(&"V".to_string()).unwrap(), "26201".as_bytes());
    assert_eq!(map.get(&"I".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"P".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"CE".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"SOC".to_string()).unwrap(), "1000".as_bytes());
    assert_eq!(map.get(&"TTG".to_string()).unwrap(), "-1".as_bytes());
    assert_eq!(map.get(&"Alarm".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"Relay".to_string()).unwrap(), "OFF".as_bytes());
    assert_eq!(map.get(&"AR".to_string()).unwrap(), "0".as_bytes());
    assert_eq!(map.get(&"BMV".to_string()).unwrap(), "700".as_bytes());
    assert_eq!(map.get(&"FW".to_string()).unwrap(), "0307".as_bytes());
    assert_eq!(map.get(&"Checksum".to_string()).unwrap(), &[0]);
}
