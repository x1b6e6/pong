#[cfg(feature = "stm32f401")]
mod stm32f401;

#[cfg(feature = "stm32f401")]
pub use stm32f401::*;

#[cfg(feature = "stm32f103")]
mod stm32f103;

#[cfg(feature = "stm32f103")]
pub use stm32f103::*;
