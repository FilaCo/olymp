pub struct BST<K, V>
where
    K: PartialOrd,
{
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> BST<K, V>
where
    K: PartialOrd,
{
    pub fn new() -> Self {
        Self { root: None }
    }
}

impl<K, V> Default for BST<K, V>
where
    K: PartialOrd,
{
    fn default() -> Self {
        Self::new()
    }
}

struct Node<K, V>
where
    K: PartialOrd,
{
    key: K,
    value: V,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    parent: Option<Box<Self>>,
}
