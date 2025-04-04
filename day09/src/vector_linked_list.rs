use std::ops::Deref;

pub type IDRef = Option<usize>;

#[derive(Clone, Debug, PartialEq)]
pub struct LLNode<T> {
    pub id: usize,
    pub prev: IDRef,
    pub next: IDRef,
    pub data: T,
}

impl<T> Deref for LLNode<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vll<T> {
    pub first: IDRef,
    pub last: IDRef,
    available: Vec<usize>,
    pub nodes: Vec<LLNode<T>>,
}
impl<T> Deref for Vll<T> {
    type Target = Vec<LLNode<T>>;
    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LLError {
    NodeAttached(usize),
    NodeDetached(usize),
    IDNotFound(usize),
    IDInvalid(usize),
    SameID(usize),
}
impl<T> Vll<T> {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
            available: Vec::new(),
            nodes: Vec::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            first: None,
            last: None,
            available: Vec::new(),
            nodes: Vec::with_capacity(capacity),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }
    pub fn is_attached(&self, id: usize) -> bool {
        self.first == Some(id) || self.nodes[id].prev.is_some()
    }
    pub fn push_last(&mut self, data: T) -> usize {
        let id = self.nodes.len();
        if self.is_empty() {
            self.nodes.push(LLNode {
                id,
                prev: None,
                next: None,
                data,
            });
            self.first = Some(id);
            self.last = Some(id);
        } else {
            self.nodes[self.last.unwrap()].next = Some(id);
            self.nodes.push(LLNode {
                id,
                prev: self.last,
                next: None,
                data,
            });
            self.last = Some(id);
        }
        id
    }
    pub fn detatch(&mut self, id: usize) -> Result<usize, LLError> {
        if id >= self.nodes.len() {
            return Err(LLError::IDNotFound(id));
        }
        // if prev is None then fail as node is already detached
        if let Some(prev) = self.nodes[id].prev {
            if let Some(next) = self.nodes[id].next {
                self.nodes[next].prev = self.nodes[id].prev;
            } else {
                self.last = self.nodes[id].prev;
            }
            self.nodes[prev].next = self.nodes[id].next;
            self.nodes[id].prev = None;
            self.nodes[id].next = None;
        } else if self.first == Some(id) {
            self.first = self.nodes[id].next;
            if let Some(first) = self.first {
                self.nodes[first].prev = None;
            }
            self.nodes[id].next = None;
        } else {
            return Err(LLError::NodeDetached(id));
        }
        Ok(id)
    }
    pub fn attach_after(&mut self, after: usize, id: usize) -> Result<usize, LLError> {
        if after == id {
            return Err(LLError::SameID(id));
        }
        if after >= self.nodes.len() {
            return Err(LLError::IDNotFound(after));
        }
        if id >= self.nodes.len() {
            return Err(LLError::IDNotFound(id));
        }
        let node_after = &self.nodes[after];
        if !self.is_attached(after) {
            return Err(LLError::NodeDetached(after));
        }
        let node = &self.nodes[id];
        if self.is_attached(id) {
            return Err(LLError::NodeAttached(id));
        }
        if let Some(next) = node_after.next {
            self.nodes[next].prev = Some(id);
        } else {
            self.last = Some(id);
        }
        self.nodes[id].next = self.nodes[after].next;
        self.nodes[id].prev = Some(after);
        self.nodes[after].next = Some(id);
        Ok(id)
    }
    pub fn move_after(&mut self, after: usize, id: usize) -> Result<usize, LLError> {
        if after == id {
            return Err(LLError::SameID(id));
        }
        self.detatch(id)?;
        self.attach_after(after, id)
    }
    pub fn iter(&self) -> VLLIteratorRef<T> {
        VLLIteratorRef::new(self)
    }
    pub fn iter_rev(&self) -> VLLIteratorReverseRef<T> {
        VLLIteratorReverseRef::new(self)
    }
}

pub struct VLLIteratorRef<'a, T> {
    list: &'a Vll<T>,
    current_node: IDRef,
}

impl<'a, T> VLLIteratorRef<'a, T> {
    fn new(list: &'a Vll<T>) -> Self {
        VLLIteratorRef {
            list,
            current_node: list.first,
        }
    }
}

impl<'a, T> Iterator for VLLIteratorRef<'a, T> {
    type Item = &'a LLNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(&self.list.nodes[self.current_node?]);
        self.current_node = self.list.nodes[self.current_node?].next;
        ret
    }
}

pub struct VLLIteratorReverseRef<'a, T> {
    list: &'a Vll<T>,
    current_node: IDRef,
}

impl<'a, T> VLLIteratorReverseRef<'a, T> {
    fn new(list: &'a Vll<T>) -> Self {
        VLLIteratorReverseRef {
            list,
            current_node: list.last,
        }
    }
}

impl<'a, T> Iterator for VLLIteratorReverseRef<'a, T> {
    type Item = &'a LLNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(&self.list.nodes[self.current_node?]);
        self.current_node = self.list.nodes[self.current_node?].prev;
        ret
    }
}
