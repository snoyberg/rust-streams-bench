use std::ops::Add;

pub enum Step<T> {
    Done,
    Yield(Option<T>),
}

pub fn range<T>(low: T, high: T) -> Range<T> {
    Range {
        low: low,
        high: high,
    }
}

pub trait Stream {
    type Item;

    fn next(&mut self) -> Step<Self::Item>;


    #[inline]
    fn filter<P>(self, p: P) -> Filter<Self, P>
        where P: FnMut(&Self::Item) -> bool,
              Self: Sized {
        Filter {
            stream: self,
            predicate: p,
        }
    }

    #[inline]
    fn fold<F, A>(self, init: A, mut f: F) -> A
        where F: FnMut(A, Self::Item) -> A,
              Self: Sized {
        // FIXME understand why we don't just make the arguments self
        // and init mutable
        let mut accum = init;
        let mut s = self;
        loop {
            match s.next() {
                Step::Done => {
                    return accum;
                }
                Step::Yield(None) => (),
                Step::Yield(Some(x)) => {
                    accum = f(accum, x);
                }
            }
        }
    }
}

pub struct Range<T> {
    low: T,
    high: T,
}
impl Stream for Range<u64> {
    type Item = u64;
    #[inline]
    fn next(&mut self) -> Step<Self::Item> {
        if self.low == self.high {
            Step::Done
        } else {
            let res = self.low;
            self.low += 1;
            Step::Yield(Some(res))
        }
    }
}
pub struct Filter<S, P> {
    stream: S,
    predicate: P,
}
impl<S: Stream, P> Stream for Filter<S, P>
    where P: FnMut(&S::Item) -> bool {
    type Item = S::Item;
    #[inline]
    fn next(&mut self) -> Step<S::Item> {
        match self.stream.next() {
            Step::Done => Step::Done,
            Step::Yield(None) => Step::Yield(None),
            Step::Yield(Some(x)) => {
                if (self.predicate)(&x) {
                    Step::Yield(Some(x))
                } else {
                    Step::Yield(None)
                }
            }
        }
    }
}

pub fn do_it() -> u64 {
    range(1 as u64, 10000001)
        .filter(|x| x % 2 == 0)
        .fold(0, Add::add)
}
