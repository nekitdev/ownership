pub enum OneOrTwo<T> {
    One(T),
    Two(T, T),
}

pub struct ExactlyOneError<I: Iterator> {
    start: Option<OneOrTwo<I::Item>>,
    iterator: I,
}

impl<I: Iterator> ExactlyOneError<I> {
    pub const fn new(start: Option<OneOrTwo<I::Item>>, iterator: I) -> Self {
        Self { start, iterator }
    }
}

impl<I: Iterator> Iterator for ExactlyOneError<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.start.take() {
            Some(OneOrTwo::Two(next, after)) => {
                self.start = Some(OneOrTwo::One(after));

                Some(next)
            }
            Some(OneOrTwo::One(next)) => Some(next),
            None => self.iterator.next(),
        }
    }
}

pub trait ExactlyOne: Iterator + Sized {
    fn exactly_one(mut self) -> Result<Self::Item, ExactlyOneError<Self>> {
        match self.next() {
            Some(one) => match self.next() {
                Some(two) => Err(ExactlyOneError::new(Some(OneOrTwo::Two(one, two)), self)),
                None => Ok(one),
            },
            None => Err(ExactlyOneError::new(None, self)),
        }
    }
}

impl<I: Iterator> ExactlyOne for I {}
