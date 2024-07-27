use core::ops::Range;

pub trait CapNum {
    type Inner;

    fn range() -> Range<Self::Inner>;
}
