#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "unstable", feature(iter_advance_by, try_trait_v2))]

mod adapter_impls {
    macro_rules! decl {
        ($($mod:ident :: $adapter:ident);+ $(;)?) => {
            $(
                mod $mod;
                pub use $mod::$adapter;
            )+
        };
    }
    decl! {
        map::PureMap;
    }
}
pub use adapter_impls::*;

/// Similar to an [`Iterator`], but assuming the input function has no side effects,
/// it can avoid additional calls
pub trait PureIterExt: Iterator + Sized {
    /// # Examples
    ///
    /// ```
    /// # use std::sync::atomic::{AtomicU32, Ordering::*};
    /// use pure_iter::PureIterExt;
    ///
    /// let mut a = AtomicU32::new(0);
    /// let mut b = AtomicU32::new(0);
    ///
    /// let c = (0..3).map(|_| a.fetch_add(1, Release)).nth(2);
    /// let d = (0..3).pure_map(|_| b.fetch_add(1, Release)).nth(2);
    ///
    /// assert_eq!(c, Some(2));
    /// assert_eq!(d, Some(0));
    ///
    /// assert_eq!(a.load(Acquire), 3);
    /// assert_eq!(b.load(Acquire), 1);
    /// ```
    fn pure_map<F, U>(self, f: F) -> PureMap<Self, F>
    where F: Fn(Self::Item) -> U,
    {
        PureMap::new(self, f)
    }
}
impl<I: Iterator> PureIterExt for I { }
