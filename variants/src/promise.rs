use std::collections::BTreeSet;

pub struct Promise {
    pub keyword: String,
    pub versions: BTreeSet<usize>,
}

impl Promise {
    pub fn new(keyword: String) -> Self {
        Self {
            keyword,
            versions: Default::default(),
        }
    }
}
