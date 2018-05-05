use pnet_macros_support::packet::PrimitiveValues;

/// Represents the "frame_type" header field.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameType(pub u8);

impl FrameType {
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

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod FrameTypes {
    use packet::framing::frame_type::FrameType;
    macro_rules! new_frame_type {
        ($id:ident, $val:expr) => {
            pub const $id: FrameType = FrameType($val);
        };
    }

    new_frame_type!(EMPTY, 0);
    new_frame_type!(PADDING, 1);
    new_frame_type!(STREAM, 2);
    new_frame_type!(CLOSE, 3);
    new_frame_type!(DETACH, 4);
    new_frame_type!(DECONGESTION, 5);
    new_frame_type!(PRIORITY, 6);
    new_frame_type!(RESET, 7);
    new_frame_type!(ACK, 8);
    new_frame_type!(SETTINGS, 9);
}
