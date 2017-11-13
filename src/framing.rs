
enum SequenceNumber {
    Len2(u16),
    Len4(u32),
    Len6(u64),
    Len8(u64),
}

impl From<u64> for SequenceNumber {
    fn from(x: u64) -> Self {
        if x < 65536 {
            Len2(x)
        } else if x < 0x100000000 {
            Len4(x)
        } else if x < 0x1000000000000 {
            Len6(x)
        } else {
            Len8(x)
        }
    }
}

impl From<u32> for SequenceNumber {
    fn from(x: u32) -> Self {
        if x < 65536 { Len2(x) } else { Len4(x) }
    }
}

impl From<u16> for SequenceNumber {
    fn from(x: u16) -> Self {
        Len2(x)
    }
}

impl From<u8> for SequenceNumber {
    fn from(x: u8) -> Self {
        Len2(x)
    }
}

struct PacketHeader<'a> {
    data: &'a [u8],
}

impl<'a> PacketHeader<'a> {
    fn new(
        buffer: &'a [u8],
        version: Option<u16>,
        fec_group: Option<u8>,
        seq_num: SequenceNumber,
    ) -> Option<Self> {
        let mut header_size = 1;
        let mut flags = 0u8;

        if version.is_some() {
            header_size += 2;
        }

        if fec_group.is_some() {
            header_size += 1;
        }

        match seq_num {
            SequenceNumber::Len2(_) => {
                header_size += 2;
            }
            SequenceNumber::Len4(_) => {
                header_size += 4;
            }
            SequenceNumber::Len6(_) => {
                header_size += 6;
            }
            SequenceNumber::Len8(_) => {
                header_size += 8;
            }
        }

        if buffer.len() < header_size {
            None
        } else {
            let mut out = Header { data: buffer };
            //pack values into header
            out.data[0] = flags;

            Some(out)
        }
    }

    fn version(&self) -> Option<u16> {
        //if version bit is set Some(version)
        None
    }

    fn fec_group(&self) -> Option<u8> {
        //if fec bit is set then Some(fec)
        None
    }

    fn sequence_number(&self) -> SequenceNumber {
        //return enum with value based on size in format byte
        unimplemented!()
    }
}
