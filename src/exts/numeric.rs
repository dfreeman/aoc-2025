pub use num_traits::AsPrimitive;

pub trait DigitCount {
  fn digit_count(self) -> u32
  where
    Self: AsPrimitive<u64>,
  {
    match self.as_().checked_ilog10() {
      Some(d) => d + 1,
      None => 1,
    }
  }
}

impl DigitCount for u32 {}
impl DigitCount for u64 {}
impl DigitCount for isize {}
