use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct PureMap<I: Iterator, F> {
    iter: I,
    f: F
}

impl<I: Iterator, F> PureMap<I, F> {
    pub fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I, F1, U> Iterator for PureMap<I, F1>
where I: Iterator,
      F1: Fn(I::Item) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&self.f)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(&self.f)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize
    where Self: Sized,
    {
        self.iter.count()
    }

    fn last(self) -> Option<Self::Item>
    where Self: Sized,
    {
        self.iter.last().map(self.f)
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where Self: Sized,
          F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, |acc, ele| {
            f(acc, (self.f)(ele))
        })
    }

    #[cfg(feature = "unstable")]
    fn advance_by(&mut self, n: usize) -> Result<(), core::num::NonZero<usize>> {
        self.iter.advance_by(n)
    }

    #[cfg(feature = "unstable")]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where Self: Sized,
          F: FnMut(B, Self::Item) -> R,
          R: core::ops::Try<Output = B>,
    {
        self.iter.try_fold(init, |acc, ele| {
            f(acc, (self.f)(ele))
        })
    }
}

impl<I, F1, U> DoubleEndedIterator for PureMap<I, F1>
where I: DoubleEndedIterator,
      F1: Fn(I::Item) -> U,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(&self.f)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(&self.f)
    }

    fn rfold<B, F>(self, init: B, mut f: F) -> B
    where Self: Sized,
          F: FnMut(B, Self::Item) -> B,
    {
        self.iter.rfold(init, |acc, ele| {
            f(acc, (self.f)(ele))
        })
    }

    #[cfg(feature = "unstable")]
    fn advance_back_by(&mut self, n: usize) -> Result<(), core::num::NonZero<usize>> {
        self.iter.advance_back_by(n)
    }

    #[cfg(feature = "unstable")]
    fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where Self: Sized,
          F: FnMut(B, Self::Item) -> R,
          R: core::ops::Try<Output = B>,
    {
        self.iter.try_rfold(init, |acc, ele| {
            f(acc, (self.f)(ele))
        })
    }
}

impl<I, F1, U> ExactSizeIterator for PureMap<I, F1>
where I: ExactSizeIterator,
      F1: Fn(I::Item) -> U,
{
}

impl<I, F1, U> FusedIterator for PureMap<I, F1>
where I: FusedIterator,
      F1: Fn(I::Item) -> U,
{
}
