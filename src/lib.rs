use pruebas::{TestData, from_bytes as from_bytes_test};
const TCPMESSAGELEN: usize = 2;
pub type TcpMessage = [u8;TCPMESSAGELEN];
pub const EMPTYTCPMESSAGE: TcpMessage = [0;TCPMESSAGELEN];

#[derive(Copy, Clone, PartialEq, Default)]
pub struct DataStruct {
    pub cinta1: bool,
    pub cinta2: bool,
    pub pogos: bool,
    pub selector: bool,
    pub sensor1: bool,
    pub sensor2: bool,
    pub caracter: char,
    pub estado: Estado,
    pub pruebas: TestData,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Estado {
    Marcha,
    Pausa,
    Parado,
}


impl Default for Estado {
    fn default() -> Self { Estado::Parado }
}

pub fn to_bytes(struct_in: DataStruct) -> TcpMessage {
    let mut bytes_ret: TcpMessage = [0;TCPMESSAGELEN];
    if struct_in.cinta1     { bytes_ret[0] |= 0x01; };
    if struct_in.cinta2     { bytes_ret[0] |= 0x02; };
    if struct_in.pogos      { bytes_ret[0] |= 0x04; };
    if struct_in.selector   { bytes_ret[0] |= 0x08; };
    if struct_in.sensor1    { bytes_ret[0] |= 0x10; };
    if struct_in.sensor2    { bytes_ret[0] |= 0x20; };

    bytes_ret[0] |= match struct_in.estado {
        Estado::Parado => { 0x00 },
        Estado::Marcha => { 0x40 },
        Estado::Pausa  => { 0x80 },
    };
    
    bytes_ret[1] = struct_in.caracter as u8;

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
        cinta1:      (bytes_in[0] & 0x01) != 0 ,
        cinta2:      (bytes_in[0] & 0x02) != 0 ,
        pogos:       (bytes_in[0] & 0x04) != 0 ,
        selector:    (bytes_in[0] & 0x08) != 0 ,
        sensor1:     (bytes_in[0] & 0x10) != 0 ,
        sensor2:     (bytes_in[0] & 0x20) != 0 ,

        estado:      match bytes_in[0] & 0xC0{
            0x00 => { Estado::Parado },
            0x40 => { Estado::Marcha },
            0x80 => { Estado::Pausa },
            _ => { Estado::Parado },
        },

        caracter:    bytes_in[1] as char,

        pruebas:    from_bytes_test(&bytes_in[2..]),
    };

    struct_ret
}
