use std::ops::Add;

pub enum Step<S: Stream> {
    Done,
    Yield(S::Item, S),
}

pub fn range<T>(low: T, high: T) -> Range<T> {
    Range {
        low: low,
        high: high,
    }
}

pub trait Stream: Sized {
    type Item;

    fn next(self) -> Step<Self>;


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
        let mut stream_option = Some(self);
        while let Some(stream) = stream_option.take() {
            match stream.next() {
                Step::Done => (),
                Step::Yield(x, new_stream) => {
                    accum = f(accum, x);
                    stream_option = Some(new_stream);
                }
            }
        }
        accum
    }
}

pub struct Range<T> {
    low: T,
    high: T,
}
impl Stream for Range<u64> {
    type Item = u64;
    #[inline]
    fn next(self) -> Step<Self> {
        if self.low == self.high {
            Step::Done
        } else {
            Step::Yield(self.low, Range {
                low: self.low + 1,
                high: self.high,
            })
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
    fn next(self) -> Step<Self> {
        let mut p = self.predicate;
        let mut stream = self.stream;
        loop {
            match stream.next() {
                Step::Done => {
                    return Step::Done;
                }
                Step::Yield(x, new_stream) => {
                    if p(&x) {
                        return Step::Yield(x, Filter {
                            predicate: p,
                            stream: new_stream,
                        });
                    } else {
                        stream = new_stream;
                    }
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
