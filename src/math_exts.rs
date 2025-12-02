pub use num_traits::AsPrimitive;

pub trait IntegralExts {
    fn digit_count(&self) -> u32
    where
        Self: Default + PartialEq + AsPrimitive<f64>,
    {
        if *self == Self::default() {
            1
        } else {
            self.as_().abs().log10() as u32 + 1
        }
    }
}

impl IntegralExts for i32 {}
impl IntegralExts for u32 {}
impl IntegralExts for i64 {}
impl IntegralExts for u64 {}
impl IntegralExts for isize {}
impl IntegralExts for usize {}
