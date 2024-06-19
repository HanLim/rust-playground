#[allow(dead_code)]
mod linked_list {
    use std::cell::RefCell;
    use std::fmt::{Debug, Display};
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    pub struct Node<T>
    where
        T: Display + Copy + Clone,
    {
        pub value: T,
        pub next: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T> Node<T>
    where
        T: Display + Copy + Clone,
    {
        pub fn new(value: T) -> Node<T> {
            Node { value, next: None }
        }
    }

    impl<T> Display for Node<T>
    where
        T: Display + Copy + Clone,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    #[derive(Debug)]
    pub struct LinkedList<T>
    where
        T: Display + Copy + Clone,
    {
        head: Option<Rc<RefCell<Node<T>>>>,
        tail: Option<Weak<RefCell<Node<T>>>>,
        len: usize,
    }

    impl<T> LinkedList<T>
    where
        T: Display + Copy + Clone + PartialEq,
    {
        pub fn new() -> Self {
            Self {
                head: None,
                tail: None,
                len: 0,
            }
        }

        pub fn from() -> Self {
            unimplemented!()
        }

        pub fn head(&self) -> Option<T> {
            match &self.head {
                Some(h) => Some(h.borrow().value.clone()),
                None => None,
            }
        }

        pub fn tail(&self) -> Option<T> {
            if let Some(t) = &self.tail {
                if let Some(_rc) = t.upgrade() {
                    return Some(_rc.borrow().value.clone());
                }
            }
            None
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn push_head(&mut self, value: T) {
            let new_head = Rc::new(RefCell::new(Node::new(value)));

            match &self.head {
                Some(head) => {
                    new_head.borrow_mut().next = Some(head.clone());
                    self.head = Some(new_head.clone());
                }
                None => {
                    self.head = Some(new_head.clone());
                    self.tail = Some(Rc::downgrade(&new_head));
                }
            }
            self.len += 1;
        }

        pub fn push_back(&mut self, value: T) {
            let new_tail = Rc::new(RefCell::new(Node::new(value)));

            if let Some(t) = self.tail.take() {
                if let Some(old_tail) = t.upgrade() {
                    old_tail.borrow_mut().next = Some(new_tail.clone());
                } else {
                    self.head = Some(new_tail.clone());
                }
            } else {
                self.head = Some(new_tail.clone())
            }

            self.tail = Some(Rc::downgrade(&new_tail));
            self.len += 1;
        }

        pub fn pop_head(&mut self) -> Option<T> {
            self.head.take().map(|old_head| {
                if let Some(next) = old_head.borrow_mut().next.take() {
                    self.head = Some(next);
                } else {
                    self.tail = None;
                }
                self.len -= 1;
                Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
            })
        }

        pub fn pop_back(&mut self) -> Option<T> {
            if self.len == 0 {
                return None;
            }

            if self.len == 1 {
                let head_node = self.head.take();
                self.tail = None;
                self.len -= 1;
                return head_node.map(|node| node.borrow_mut().value);
            }

            let mut current = self.head.clone();
            let mut prev = None;

            while let Some(ref curr_node) = current {
                if curr_node.borrow().next.is_none() {
                    break;
                }
                prev = current.clone();
                current = curr_node.clone().borrow().next.clone();
            }

            match prev {
                Some(_p) => {
                    _p.borrow_mut().next = None;
                    self.tail = Some(Rc::downgrade(&_p));
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }

            self.len -= 1;

            // current.map(|node| node.as_ref().borrow().value)
            current.map(|node| Rc::try_unwrap(node).ok().unwrap().into_inner().value)
        }

        pub fn insert_at(&mut self, index: usize, new_value: T) -> bool {
            unimplemented!()
        }

        pub fn remove_at(&mut self, index: usize) -> Option<T> {
            unimplemented!()
        }

        pub fn clear(&mut self) {
            self.head = None;
            self.tail = None;
            self.len = 0;

            // while self.pop_back().is_some() {}
        }

        pub fn replace(&mut self, index: usize, new_value: T) -> bool {
            if index > self.len {
                return false;
            }

            for (i, node) in self.iter_mut().enumerate() {
                if i == index {
                    node.borrow_mut().value = new_value;
                    return true;
                }
            }

            false
        }

        pub fn index_of(&self, value: T) -> Option<usize> {
            for (index, t) in self.iter().enumerate() {
                if value == t {
                    return Some(index);
                }
            }
            None
        }

        pub fn contains(&self, value: T) -> bool {
            for t in self.iter() {
                if value == t {
                    return true;
                }
            }
            false
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn display(&self) {
            let content: Vec<_> = self.iter().map(|node| node.to_string()).collect();
            println!("[{}]", content.join(" -> "));
        }

        // -----------------------------------------------------------
        // Iterators
        // -----------------------------------------------------------
        pub fn iter(&self) -> LinkedListIterator<T> {
            LinkedListIterator {
                current: self.head.clone(),
            }
        }

        pub fn iter_mut(&mut self) -> LinkedListIteratorMut<T> {
            LinkedListIteratorMut {
                current: self.head.clone(),
            }
        }
    }

    // -----------------------------------------------------------
    // Iterators
    // -----------------------------------------------------------

    impl<T> IntoIterator for LinkedList<T>
    where
        T: Display + Clone + Copy,
    {
        type Item = T;
        type IntoIter = LinkedListIterator<T>;

        fn into_iter(self) -> Self::IntoIter {
            LinkedListIterator { current: self.head }
        }
    }

    pub struct LinkedListIterator<T>
    where
        T: Display + Clone + Copy,
    {
        current: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T> Iterator for LinkedListIterator<T>
    where
        T: Display + Copy + Clone,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.current.take() {
                Some(node) => {
                    let node_ref = node.borrow();
                    self.current = node_ref.next.clone();
                    Some(node_ref.value)
                }
                None => None,
            }
        }
    }

    pub struct LinkedListIteratorMut<T>
    where
        T: Display + Copy + Clone,
    {
        current: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T> Iterator for LinkedListIteratorMut<T>
    where
        T: Display + Copy + Clone,
    {
        type Item = Rc<RefCell<Node<T>>>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.current.take() {
                Some(node) => {
                    let node_ref = node.borrow();
                    self.current = node_ref.next.clone();
                    Some(node.clone())
                }
                None => None,
            }
        }
    }
}

fn main() {
    use linked_list::LinkedList;

    let mut ll: LinkedList<i32> = LinkedList::new();

    // ll.push_back(1);
    // ll.push_back(21);
    // ll.push_back(13);

    // ll.display();

    // ll.replace(13, 311);
    // ll.display();
    println!("{:?}", ll.iter_mut().last());
}
