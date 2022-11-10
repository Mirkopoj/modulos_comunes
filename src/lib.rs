const TCPMESSAGELEN: usize = 1;
pub type TcpMessage = [u8;TCPMESSAGELEN];
pub const EMPTYTCPMESSAGE: TcpMessage = [0;TCPMESSAGELEN];

#[derive(Copy, Clone,PartialEq)]
pub struct DataStruct {
    pub cinta: bool,
    pub pogos: bool,
    pub selector: bool,
    pub sensor: bool,
}

pub fn to_bytes(struct_in: DataStruct) -> TcpMessage {
    let mut bytes_ret: TcpMessage = [0;1];
    if struct_in.cinta      { bytes_ret[0] |= 0x01; };
    if struct_in.pogos      { bytes_ret[0] |= 0x02; };
    if struct_in.selector   { bytes_ret[0] |= 0x04; };
    if struct_in.sensor     { bytes_ret[0] |= 0x08; };

    bytes_ret
}

pub trait Convert {
    fn to_bytes(&self) -> TcpMessage;
    
}
impl Convert for DataStruct {
    fn to_bytes(&self) -> TcpMessage {
        to_bytes(*self)
    }
}

pub fn from_bytes(bytes_in: &[u8]) -> DataStruct {
    let struct_ret = DataStruct {
        cinta:      (bytes_in[0] & 0x01) != 0 ,
        pogos:      (bytes_in[0] & 0x02) != 0 ,
        selector:   (bytes_in[0] & 0x04) != 0 ,
        sensor:     (bytes_in[0] & 0x08) != 0 ,
    };

    struct_ret
}
