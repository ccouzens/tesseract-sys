pub const kPointsPerInch: ::std::os::raw::c_int = 72;
pub const kMinCredibleResolution: ::std::os::raw::c_int = 70;
pub const kMaxCredibleResolution: ::std::os::raw::c_int = 2400;
pub const kResolutionEstimationFactor: ::std::os::raw::c_int = 10;
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
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Orientation {
    ORIENTATION_PAGE_UP = 0,
    ORIENTATION_PAGE_RIGHT = 1,
    ORIENTATION_PAGE_DOWN = 2,
    ORIENTATION_PAGE_LEFT = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WritingDirection {
    WRITING_DIRECTION_LEFT_TO_RIGHT = 0,
    WRITING_DIRECTION_RIGHT_TO_LEFT = 1,
    WRITING_DIRECTION_TOP_TO_BOTTOM = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextlineOrder {
    TEXTLINE_ORDER_LEFT_TO_RIGHT = 0,
    TEXTLINE_ORDER_RIGHT_TO_LEFT = 1,
    TEXTLINE_ORDER_TOP_TO_BOTTOM = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PageSegMode {
    PSM_OSD_ONLY = 0,
    PSM_AUTO_OSD = 1,
    PSM_AUTO_ONLY = 2,
    PSM_AUTO = 3,
    PSM_SINGLE_COLUMN = 4,
    PSM_SINGLE_BLOCK_VERT_TEXT = 5,
    PSM_SINGLE_BLOCK = 6,
    PSM_SINGLE_LINE = 7,
    PSM_SINGLE_WORD = 8,
    PSM_CIRCLE_WORD = 9,
    PSM_SINGLE_CHAR = 10,
    PSM_SPARSE_TEXT = 11,
    PSM_SPARSE_TEXT_OSD = 12,
    PSM_RAW_LINE = 13,
    PSM_COUNT = 14,
}
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
pub enum ParagraphJustification {
    JUSTIFICATION_UNKNOWN = 0,
    JUSTIFICATION_LEFT = 1,
    JUSTIFICATION_CENTER = 2,
    JUSTIFICATION_RIGHT = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OcrEngineMode {
    OEM_TESSERACT_ONLY = 0,
    OEM_LSTM_ONLY = 1,
    OEM_TESSERACT_LSTM_COMBINED = 2,
    OEM_DEFAULT = 3,
    OEM_COUNT = 4,
}
