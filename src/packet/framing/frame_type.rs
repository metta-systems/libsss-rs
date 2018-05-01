use pnet_macros_support::packet::PrimitiveValues;

/// Represents the "frame_type" header field.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameType(pub u8);

impl FrameType {
    /// Create an ICMP type
    pub fn new(val: u8) -> FrameType {
        FrameType(val)
    }
}

impl PrimitiveValues for FrameType {
    type T = (u8,);
    fn to_primitive_values(&self) -> (u8,) {
        (self.0,)
    }
}
