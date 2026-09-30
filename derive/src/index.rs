use proc_macro2::Span;

pub type Index = u32;

pub const ZERO: Index = 0;
pub const ONE: Index = 1;

pub fn call_site(index: Index) -> syn::Index {
    let span = Span::call_site();

    syn::Index { index, span }
}

pub struct Indexer {
    option: Option<Index>,
}

impl Indexer {
    pub const fn construct(option: Option<Index>) -> Self {
        Self { option }
    }

    pub const fn at(index: Index) -> Self {
        Self::construct(Some(index))
    }

    pub const fn new() -> Self {
        Self::at(ZERO)
    }
}

impl Iterator for Indexer {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.option?;

        self.option = index.checked_add(ONE);

        Some(index)
    }
}

pub const fn indexer() -> Indexer {
    Indexer::new()
}
