use std::iter::Iterator;

pub struct ProcessOkInner<I> {
    iter: I,
}

pub struct ProcessOk<I, F> {
    iter: I,
    processor: F,
}

impl<I, T, E> Iterator for ProcessOkInner<I>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|result| match result {
            Ok(value) => Ok((self.f)(value)),
            Err(err) => Err(err),
        })
    }
}

impl<I, T, F, U, E> Iterator for ProcessOk<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(T) -> U,
{
    type Item = Result<U, E>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub trait IterExt42: Iterator {
    fn process_ok<T, IterT, F, U, IterU, E>(self, processor: F) -> ProcessOkInner<Self, F>
    where
        Self: Iterator<Item = Result<T, E>> + Sized,
        F: FnOnce(IterT) -> IterU,
        IterT: Iterator<Item = T>,
        IterU: Iterator<Item = U>,
    {
        let result = processor(ProcessOkInner {
            iter: self,
            f: processor,
        });

        result
    }
}

// IMPORTANT it was not enough to `pub trait IterExt42: Iterator`, this line is also needed
// wait… kinda obv since the first only shows IterExt42 is a subtype of Iterator, and the next one means
// its equivalent. True.
impl<T> IterExt42 for T where T: Iterator {}
