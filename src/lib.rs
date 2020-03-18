//! HashMap/Set<Pin<Arc<T>>>` based fully safety tree collection
//! # Examples
//! ```
//! let pt = PinTree::<i32>::new();
//! 
//! let a = pt.node(1);
//! let b = pt.node(2);
//! let c = pt.node(2);
//! 
//! pt.set_parent(b, a);
//! pt.set_parent(c, a);
//! //    a
//! //  ↙ ↘
//! // b    c
//! 
//! assert_eq!(pt.is_parent(b, a), true);
//! assert_eq!(pt.is_child(c, a), true);
//! ```
//! See [PinTree](struct.PinTree.html)

use std::collections::hash_set::Iter;
use std::collections::{HashMap, HashSet};
use std::convert::AsRef;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

trait HashBox {
    fn get_ptr_usize(&self) -> usize;
}
impl<T> HashBox for Pin<Arc<T>>
where
    T: ?Sized,
{
    fn get_ptr_usize(&self) -> usize {
        self.deref() as *const _ as *const () as usize
    }
}

/// `HashMap/Set<Pin<Arc<T>>>` based fully safety tree collection
pub struct PinTree<T> {
    nodes: HashSet<PinNode<T>>,
    parents: HashMap<PinNode<T>, PinNode<T>>,
    childs: HashMap<PinNode<T>, HashSet<PinNode<T>>>,
    _empty_node_set: HashSet<PinNode<T>>,
}
impl<T> PinTree<T> {
    /// Create a PinTree
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            parents: HashMap::new(),
            childs: HashMap::new(),
            _empty_node_set: HashSet::new(),
        }
    }
    /// Create a PinNode and add it to PinTree
    pub fn node(&mut self, v: T) -> &PinNode<T> {
        let n: PinNode<T> = PinNode::new(v);
        self.nodes.insert(n.clone());
        self.nodes.get(&n).unwrap()
    }
    /// add a PinNode to PinTree
    pub fn node_from(&mut self, node: PinNode<T>) -> bool {
        self.nodes.insert(node)
    }
    fn remove_child(&mut self, parent: &PinNode<T>, this: &PinNode<T>) -> bool {
        if let Some(childs) = self.childs.get_mut(parent) {
            childs.remove(this)
        } else {
            false
        }
    }
    /// Set parent-child relationship   
    /// If nodes are not in PinTree, they will be added
    pub fn set_parent(&mut self, this: &PinNode<T>, parent: &PinNode<T>) -> bool {
        self.node_from(this.clone());
        self.node_from(parent.clone());
        if self.is_parent(this, parent) {
            return false;
        }
        self.remove_child(parent, this);
        self.parents.insert(this.clone(), parent.clone());
        if !self.childs.contains_key(parent) {
            self.childs.insert(parent.clone(), HashSet::new());
        }
        let childs = self.childs.get_mut(parent).unwrap();
        childs.insert(this.clone());
        true
    }
    /// Unset parent-child relationship   
    pub fn unset_parent(&mut self, this: &PinNode<T>, parent: &PinNode<T>) -> bool {
        if self.is_parent(this, parent) {
            return false;
        }
        self.parents.remove(&this);
        self.remove_child(parent, this);
        true
    }
    /// Check if it's parent
    pub fn is_parent(&self, this: &PinNode<T>, parent: &PinNode<T>) -> bool {
        self.parents
            .get(&this)
            .map(|v| *v == *parent)
            .unwrap_or(false)
    }
    /// Check if it's child
    pub fn is_child(&self, this: &PinNode<T>, parent: &PinNode<T>) -> bool {
        self.childs
            .get(parent)
            .map(|childs| childs.contains(this))
            .unwrap_or(false)
    }
    /// Get the parent of node
    pub fn get_parent(&self, this: &PinNode<T>) -> Option<&PinNode<T>> {
        self.parents.get(&this)
    }
    /// Get the childs of node
    pub fn get_childs(&self, parent: &PinNode<T>) -> Iter<PinNode<T>> {
        self.childs
            .get(parent)
            .map(|childs| childs.iter())
            .unwrap_or(self._empty_node_set.iter())
    }
    /// Remove node from PinTree
    pub fn remove(&mut self, this: &PinNode<T>) -> bool {
        if !self.nodes.contains(this) {
            return false;
        }
        if let Some(parent) = self.parents.get(this).map(|p| p.clone()) {
            self.remove_child(&parent, this);
            self.parents.remove(this);
        }
        self.nodes.remove(this);
        true
    }
}

/// PinNode is a PinArc Box  
/// Wrap of Pin<Arc<T>>
pub struct PinNode<T> {
    inner: Pin<Arc<T>>,
}
impl<T> PinNode<T> {
    /// Create a PinNode
    pub fn new(v: T) -> Self {
        PinNode { inner: Arc::pin(v) }
    }
}
impl<T> Clone for PinNode<T> {
    fn clone(&self) -> Self {
        PinNode {
            inner: self.inner.clone(),
        }
    }
}
impl<T> Hash for PinNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let p = self.inner.get_ptr_usize();
        p.hash(state);
    }
}
impl<T> PartialEq for PinNode<T> {
    fn eq(&self, other: &Self) -> bool {
        let p = self.inner.get_ptr_usize();
        let po = other.inner.get_ptr_usize();
        p == po
    }
}
impl<T> Eq for PinNode<T> {}
impl<T> Deref for PinNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
impl<T> AsRef<T> for PinNode<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}
