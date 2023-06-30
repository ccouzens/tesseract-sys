pub const kPointsPerInch: ::std::os::raw::c_int = 72;
pub const kMinCredibleResolution: ::std::os::raw::c_int = 70;
pub const kMaxCredibleResolution: ::std::os::raw::c_int = 2400;
pub const kResolutionEstimationFactor: ::std::os::raw::c_int = 10;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PageIteratorLevel {
    RIL_BLOCK = 0,
    RIL_PARA = 1,
    RIL_TEXTLINE = 2,
    RIL_WORD = 3,
    RIL_SYMBOL = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PolyBlockType {
    PT_UNKNOWN = 0,
    PT_FLOWING_TEXT = 1,
    PT_HEADING_TEXT = 2,
    PT_PULLOUT_TEXT = 3,
    PT_EQUATION = 4,
    PT_INLINE_EQUATION = 5,
    PT_TABLE = 6,
    PT_VERTICAL_TEXT = 7,
    PT_CAPTION_TEXT = 8,
    PT_FLOWING_IMAGE = 9,
    PT_HEADING_IMAGE = 10,
    PT_PULLOUT_IMAGE = 11,
    PT_HORZ_LINE = 12,
    PT_VERT_LINE = 13,
    PT_NOISE = 14,
    PT_COUNT = 15,
}