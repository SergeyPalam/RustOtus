use crate::err_house;
use anyhow::{bail, Result};
use log::*;
use tokio::io::AsyncReadExt;

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

    pub fn deserialize(bin_pack: &[u8]) -> Result<Self> {
        if bin_pack.is_empty() {
            error!("Pack is empty");
            bail!(err_house::ErrorKind::DeserializationError);
        }

        let type_pack = TypePack::from(bin_pack[0]);
        match type_pack {
            TypePack::Simple => {
                let payload_len = bin_pack[1] as usize;
                if bin_pack.len() < payload_len + 2 {
                    error!(
                        "Pack is to short. Pack len is {}, but payload len is {}",
                        bin_pack.len(),
                        payload_len
                    );
                    bail!(err_house::ErrorKind::DeserializationError);
                }
                let payload = bin_pack[2..].to_vec();
                Ok(Self { type_pack, payload })
            }
            TypePack::Unknown(val) => {
                error!("Unknown type pack: {val}");
                bail!(err_house::ErrorKind::UnknownTypePack)
            }
        }
    }

    pub async fn from_reader<T>(reader: &mut T) -> Result<Self>
    where
        T: Unpin,
        T: AsyncReadExt,
    {
        let mut type_pack = vec![0];
        reader.read_exact(&mut type_pack).await?;
        let type_pack = TypePack::from(type_pack[0]);
        match type_pack {
            TypePack::Simple => {
                let mut len = vec![0];
                reader.read_exact(&mut len).await?;
                let mut payload = vec![0; len[0] as usize];
                reader.read_exact(&mut payload).await?;
                Ok(Self { type_pack, payload })
            }
            TypePack::Unknown(val) => {
                error!("Unknown type pack: {val}");
                bail!(err_house::ErrorKind::UnknownTypePack)
            }
        }
    }

    pub fn into_payload(self) -> Vec<u8> {
        self.payload
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

    #[tokio::test]
    async fn test_pack_from_reader() {
        let bytes = vec![SIMPLE_PACK, 3, 1, 2, 3];
        let mut stream = Cursor::new(bytes);
        let pack = TranportPack::from_reader(&mut stream).await.unwrap();
        if let TypePack::Unknown(_) = pack.type_pack {
            panic!();
        }
        assert_eq!(pack.payload.len(), 3);
        assert_eq!(pack.payload, vec![1, 2, 3]);

        let bytes = vec![SIMPLE_PACK, 3, 1, 2];
        let mut stream = Cursor::new(bytes);

        let pack = TranportPack::from_reader(&mut stream).await;
        assert!(pack.is_err());

        let bytes = vec![SIMPLE_PACK, 3, 3, 4, 5, 47];
        let mut stream = Cursor::new(bytes);

        let pack = TranportPack::from_reader(&mut stream).await.unwrap();
        if let TypePack::Unknown(_) = pack.type_pack {
            panic!();
        }
        assert_eq!(pack.payload.len(), 3);
        assert_eq!(pack.payload, vec![3, 4, 5]);

        let bytes = vec![];
        let mut stream = Cursor::new(bytes);

        let pack = TranportPack::from_reader(&mut stream).await;
        assert!(pack.is_err());
    }
}
