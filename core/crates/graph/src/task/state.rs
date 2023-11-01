use anymap2::{any::CloneAnySendSync, Map};

/// Content container for nodes
pub type Content = Map<dyn CloneAnySendSync + Send + Sync>;

#[derive(Debug)]
pub struct Output(Option<Content>);

/// Node input value
pub struct Input(Vec<Content>);

pub struct Variables(Vec<Content>);

impl Output {
    pub fn new<H: Send + Sync + CloneAnySendSync>(val: H) -> Self {
        let mut map = Content::new();
        assert!(map.insert(val).is_none(), "Value already exists");
        Self(Some(map))
    }

    pub fn empty() -> Self {
        Self(None)
    }
}

impl Input {
    pub fn new(input: Vec<Content>) -> Self {
        Self(input)
    }

    pub fn get_iter(&self) -> std::slice::Iter<Content> {
        self.0.iter()
    }
}
