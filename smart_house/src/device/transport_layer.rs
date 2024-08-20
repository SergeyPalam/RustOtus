use crate::err_house;
use std::io::Read;

const SIMPLE_PACK: u8 = 0xA2;

#[derive(Clone, Copy)]
pub enum TypePack {
    Simple,
    Unknown(u8),
}

impl From<u8> for TypePack {
    fn from(value: u8) -> Self {
        match value {
            SIMPLE_PACK => Self::Simple,
            _ => Self::Unknown(value),
        }
    }
}

impl From<TypePack> for u8 {
    fn from(value: TypePack) -> Self {
        match value {
            TypePack::Simple => SIMPLE_PACK,
            TypePack::Unknown(val) => val,
        }
    }
}

pub struct TranportPack {
    type_pack: TypePack,
    payload: Vec<u8>,
}

impl TranportPack {
    pub fn new(type_pack: TypePack, payload: Vec<u8>) -> Self {
        Self { type_pack, payload }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let type_pack: u8 = self.type_pack.into();
        let len: u8 = self.payload.len() as u8;

        res.push(type_pack);
        res.push(len);
        res.extend_from_slice(&self.payload);
        res
    }

    pub fn from_reader(reader: &mut impl Read) -> Result<Self, err_house::Err> {
        let mut type_pack = vec![0];
        reader.read_exact(&mut type_pack)?;
        let type_pack = TypePack::from(type_pack[0]);
        match type_pack {
            TypePack::Simple => {
                let mut len = vec![0];
                reader.read_exact(&mut len)?;
                let mut payload = vec![0; len[0] as usize];
                reader.read_exact(&mut payload)?;
                Ok(Self { type_pack, payload })
            }
            TypePack::Unknown(val) => Err(err_house::Err::new(&format!("Unknown type pack {val}"))),
        }
    }

    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_pack_to_bytes() {
        let pack = TranportPack::new(TypePack::Simple, vec![0, 0, 0]);
        let bytes: Vec<u8> = pack.serialize();
        let expected = vec![SIMPLE_PACK, 3, 0, 0, 0];
        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_pack_from_reader() {
        let bytes = vec![SIMPLE_PACK, 3, 1, 2, 3];
        let mut stream = Cursor::new(bytes);
        let pack = TranportPack::from_reader(&mut stream).unwrap();
        if let TypePack::Unknown(_) = pack.type_pack {
            panic!();
        }
        assert_eq!(pack.payload.len(), 3);
        assert_eq!(pack.payload, vec![1, 2, 3]);

        let bytes = vec![SIMPLE_PACK, 3, 1, 2];
        let mut stream = Cursor::new(bytes);

        let pack = TranportPack::from_reader(&mut stream);
        assert!(pack.is_err());

        let bytes = vec![SIMPLE_PACK, 3, 3, 4, 5, 47];
        let mut stream = Cursor::new(bytes);

        let pack = TranportPack::from_reader(&mut stream).unwrap();
        if let TypePack::Unknown(_) = pack.type_pack {
            panic!();
        }
        assert_eq!(pack.payload.len(), 3);
        assert_eq!(pack.payload, vec![3, 4, 5]);

        let bytes = vec![];
        let mut stream = Cursor::new(bytes);

        let pack = TranportPack::from_reader(&mut stream);
        assert!(pack.is_err());
    }
}
