pub mod models;

#[cfg(test)]
mod tests;

use crate::parser::models::*;
use chrono::Utc;
use std::collections::HashMap;

pub type VEDirectParse = HashMap<String, Vec<u8>>;
pub struct Parser {
    first_parse: bool,
    parse_buf: Vec<u8>,
    fields: VEDirectParse,
    sum: u16,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    /// Create Parser instance
    pub fn new() -> Self {
        Parser {
            first_parse: true,
            parse_buf: Vec::new(),
            fields: HashMap::new(),
            sum: 0,
        }
    }

    fn parse_field(data: &[u8], mut read_pos: usize) -> Result<(VEField, usize), VEError> {
        if read_pos + 1 >= data.len() {
            return Err(VEError::NeedMoreData);
        }

        if data[read_pos] == CR && data[read_pos + 1] == LF {
            read_pos += 2;
            match data[read_pos..].iter().position(|&c| c == TAB) {
                Some(pos) => {
                    let label = String::from_utf8((data[read_pos..(read_pos + pos)]).to_vec())
                        .map_err(|e| VEError::Parse(format!("label string was invalid {}", e)))?;

                    read_pos = read_pos + pos + 1;
                    if read_pos >= data.len() {
                        return Err(VEError::NeedMoreData);
                    }
                    let endpos_res = data[read_pos..].iter().position(|&c| c == CR);
                    match endpos_res {
                        Some(endpos) => {
                            if endpos > 0 {
                                let value = &data[read_pos..(read_pos + endpos)];
                                Ok((
                                    VEField {
                                        label,
                                        value: value.to_vec(),
                                    },
                                    read_pos + endpos,
                                ))
                            } else {
                                Ok((VEField { label, value: vec![0] }, read_pos))
                            }
                        }
                        None => {
                            if label == "Checksum" {
                                Ok((
                                    VEField {
                                        label,
                                        value: vec![u8::from_le(data[read_pos])],
                                    },
                                    read_pos + 1,
                                ))
                            } else {
                                Err(VEError::NeedMoreData)
                            }
                        }
                    }
                }
                None => Err(VEError::NeedMoreData),
            }
        } else {
            Err(VEError::Parse("Illegal field start".to_string()))
        }
    }

    /// Parses victron energy protocol packet, if not complete returns VEError::NeedMoreData and stores its state
    pub fn parse_slice(&mut self, data: &[u8]) -> Result<VEDirectParse, VEError> {
        if self.first_parse {
            // skip to first field start as we might have started somewhere in the middle
            match data.iter().position(|&c| c == CR) {
                Some(pos) => self.parse_buf.extend_from_slice(&data[pos..]),
                None => return Err(VEError::NeedMoreData),
            }
            self.first_parse = false;
        } else {
            self.parse_buf.extend(data);
        }

        let mut cp = 0;
        loop {
            // skip hex mode messages, those can periodically occur
            while cp + 1 < self.parse_buf.len() && self.parse_buf[cp] == COLON && self.parse_buf[cp + 1] == A {
                match self.parse_buf[cp..].iter().position(|&c| c == CR) {
                    Some(pos) => {
                        cp += pos;
                    }
                    None => {
                        self.parse_buf.drain(0..cp);
                        return Err(VEError::NeedMoreData);
                    }
                }
            }

            while cp + 3 < self.parse_buf.len()
                && self.parse_buf[cp] == CR
                && self.parse_buf[cp + 1] == LF
                && self.parse_buf[cp + 2] == COLON
                && self.parse_buf[cp + 3] == A
            {
                match self.parse_buf[cp + 2..].iter().position(|&c| c == CR) {
                    Some(pos) => {
                        cp = cp + pos + 2;
                    }
                    None => {
                        self.parse_buf.drain(0..cp);
                        return Err(VEError::NeedMoreData);
                    }
                }
            }

            match Self::parse_field(&self.parse_buf, cp) {
                Ok((field, read_pos)) => {
                    cp = read_pos;
                    self.fields.insert(field.label.clone(), field.value.clone());

                    self.sum = (self.sum + CR as u16) & 0xff;
                    self.sum = (self.sum + LF as u16) & 0xff;
                    for byte in field.label.to_string().as_bytes().iter() {
                        self.sum = (self.sum + (*byte) as u16) & 0xff;
                    }
                    self.sum = (self.sum + TAB as u16) & 0xff;
                    for byte in field.value.clone().iter() {
                        self.sum = (self.sum + (*byte) as u16) & 0xff;
                    }

                    if field.label == "Checksum" {
                        self.fields.insert("Checksum".to_string(), vec![u8::from_le(field.value[0])]);
                        self.fields
                            .insert("Calc_sum".to_string(), vec![u8::from_le(self.sum.try_into().unwrap())]);
                        self.fields
                            .insert("Time".to_string(), Utc::now().timestamp().to_string().as_bytes().to_vec());
                        let ret = self.fields.clone();
                        self.parse_buf.drain(0..cp);
                        self.fields.clear();
                        self.sum = 0;
                        return Ok(ret);
                    }
                }
                Err(VEError::NeedMoreData) => {
                    self.parse_buf.drain(0..cp);
                    return Err(VEError::NeedMoreData);
                }
                Err(VEError::Parse(error)) => {
                    self.parse_buf.clear();
                    self.first_parse = true;
                    return Err(VEError::Parse(error));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}
