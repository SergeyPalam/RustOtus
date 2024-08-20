use crate::err_house;

const SUCCESS_RESP: u8 = 0x7C;
const ERR_RESP: u8 = 0xB5;

const TURN_ON_CMD: u8 = 0x01;
const TURN_OFF_CMD: u8 = 0x04;
const POWER_CMD: u8 = 0x07;
const CHECK_TURNED: u8 = 0x0A;

const SIZE_REQ_HEADER: usize = 0x01;
const SIZE_RESP_HEADER: usize = 0x02;

#[derive(Clone, Copy)]
pub enum ReqType {
    TurnOn,
    TurnOff,
    Power,
    CheckTurned,
    Unknown(u8),
}

impl From<u8> for ReqType {
    fn from(value: u8) -> Self {
        match value {
            TURN_ON_CMD => ReqType::TurnOn,
            TURN_OFF_CMD => ReqType::TurnOff,
            POWER_CMD => ReqType::Power,
            CHECK_TURNED => ReqType::CheckTurned,
            _ => ReqType::Unknown(value),
        }
    }
}

impl From<ReqType> for u8 {
    fn from(value: ReqType) -> Self {
        match value {
            ReqType::TurnOn => TURN_ON_CMD,
            ReqType::TurnOff => TURN_OFF_CMD,
            ReqType::Power => POWER_CMD,
            ReqType::CheckTurned => CHECK_TURNED,
            ReqType::Unknown(val) => val,
        }
    }
}

pub struct SockRequest {
    req_type: ReqType,
    payload: Vec<u8>,
}

impl SockRequest {
    fn new(req_type: ReqType, payload: Vec<u8>) -> Self {
        Self { req_type, payload }
    }
    pub fn new_turn_on() -> Self {
        Self::new(ReqType::TurnOn, Vec::new())
    }

    pub fn new_turn_off() -> Self {
        Self::new(ReqType::TurnOff, Vec::new())
    }

    pub fn new_check_turned() -> Self {
        Self::new(ReqType::CheckTurned, Vec::new())
    }

    pub fn new_get_power() -> Self {
        Self::new(ReqType::Power, Vec::new())
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.push(self.req_type.into());
        res.extend_from_slice(&self.payload);
        res
    }

    pub fn deserialize(pack: &[u8]) -> Result<Self, err_house::Err> {
        if pack.len() < SIZE_REQ_HEADER {
            return Err(err_house::Err::new(&format!(
                "Can't deserialize request: wrong pack len: {}",
                pack.len()
            )));
        }

        let req_type = ReqType::from(pack[0]);
        Ok(SockRequest {
            req_type,
            payload: pack[SIZE_REQ_HEADER..].to_vec(),
        })
    }
}

#[derive(Copy, Clone)]
pub enum RespType {
    Success,
    Error,
    Unknown(u8),
}

impl From<u8> for RespType {
    fn from(value: u8) -> Self {
        match value {
            SUCCESS_RESP => Self::Success,
            ERR_RESP => Self::Error,
            _ => Self::Unknown(value),
        }
    }
}

impl From<RespType> for u8 {
    fn from(value: RespType) -> Self {
        match value {
            RespType::Success => SUCCESS_RESP,
            RespType::Error => ERR_RESP,
            RespType::Unknown(val) => val,
        }
    }
}

pub struct SockResponse {
    resp_type: RespType,
    req_type: ReqType,
    payload: Vec<u8>,
}

impl SockResponse {
    pub fn new(resp_type: RespType, req_type: ReqType, payload: Vec<u8>) -> Self {
        Self {
            resp_type,
            req_type,
            payload,
        }
    }

    pub fn get_resp_type(&self) -> RespType {
        self.resp_type
    }

    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.push(self.resp_type.into());
        res.push(self.req_type.into());
        res.extend_from_slice(&self.payload);
        res
    }
    pub fn deserialize(pack: &[u8]) -> Result<Self, err_house::Err> {
        if pack.len() < SIZE_RESP_HEADER {
            return Err(err_house::Err::new(&format!(
                "Can't deserialize response: wrong pack len: {}",
                pack.len()
            )));
        }

        let resp_type = RespType::from(pack[0]);
        if let RespType::Unknown(val) = resp_type {
            return Err(err_house::Err::new(&format!(
                "Unknown response type: {}",
                val
            )));
        }
        let req_type = ReqType::from(pack[1]);
        if let ReqType::Unknown(val) = req_type {
            return Err(err_house::Err::new(&format!(
                "Unknown request type: {}",
                val
            )));
        }
        Ok(Self {
            resp_type,
            req_type,
            payload: pack[SIZE_RESP_HEADER..].to_vec(),
        })
    }
}
