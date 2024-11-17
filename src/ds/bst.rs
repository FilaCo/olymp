use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct BST<K, V>
where
    K: Ord,
{
    root: LinkedNode<K, V>,
    size: usize,
    _pk: PhantomData<K>,
    _pv: PhantomData<V>,
}

impl<K, V> BST<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
            _pk: PhantomData,
            _pv: PhantomData,
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        unsafe { self.search(key).map(|node| &(*node.as_ptr()).value) }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        unsafe { self.search(key).map(|node| &mut (*node.as_ptr()).value) }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        unsafe {
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                key,
                value,
                left: None,
                right: None,
                parent: None,
            })));

            if self.root.is_none() {
                self.root = Some(node);

                return None;
            }

            let mut cur = self.root?;
            loop {
                cur = match &(*node.as_ptr()).key.cmp(&(*cur.as_ptr()).key) {
                    Ordering::Less => {
                        if let Some(old) = (*cur.as_ptr()).left {
                            old
                        } else {
                            (*cur.as_ptr()).left = Some(node);
                            self.size += 1;

                            break None;
                        }
                    }
                    Ordering::Equal => {
                        let old_value = Box::from_raw(cur.as_ptr()).value;
                        let boxed_node = Box::from_raw(node.as_ptr());

                        (*cur.as_ptr()).value = boxed_node.value;

                        break Some(old_value);
                    }
                    Ordering::Greater => {
                        if let Some(old) = (*cur.as_ptr()).right {
                            old
                        } else {
                            (*cur.as_ptr()).right = Some(node);
                            self.size += 1;

                            break None;
                        }
                    }
                }
            }
        }
    }

    pub fn erase(&mut self, key: &K) -> Option<V> {
        // self.search(key).map(|node| {})
        None
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn search(&self, key: &K) -> LinkedNode<K, V> {
        unsafe {
            let mut cur = self.root?;

            loop {
                cur = match key.cmp(&(*cur.as_ptr()).key) {
                    Ordering::Less => (*cur.as_ptr()).left?,
                    Ordering::Equal => break Some(cur),
                    Ordering::Greater => (*cur.as_ptr()).right?,
                }
            }
        }
    }
}

impl<K, V> Default for BST<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

type LinkedNode<K, V> = Option<NonNull<Node<K, V>>>;

struct Node<K, V>
where
    K: PartialOrd,
{
    key: K,
    value: V,
    left: LinkedNode<K, V>,
    right: LinkedNode<K, V>,
    parent: LinkedNode<K, V>,
}
