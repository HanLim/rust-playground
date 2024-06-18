// std::collections::linked_list;
mod linked_list {
    use std::cell::RefCell;
    use std::fmt::Display;
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
            Node {
                value,
                next: None,
            }
        }
    }

    impl <T> Display for Node<T> 
    where T: Display + Copy + Clone
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
    }

    impl<T> LinkedList<T>
    where
        T: Display + Copy + Clone,
    {
        pub fn new(value: T) -> LinkedList<T> {
            let head = Rc::new(RefCell::new(Node::new(value)));

            LinkedList {
                head: Some(head.clone()),
                tail: Some(Rc::downgrade(&head)),
            }
        }

        pub fn new_empty() -> LinkedList<T> {
            LinkedList {
                head: None,
                tail: None,
            }
        }

        pub fn head(&self) -> Option<T> {
            match &self.head {
                Some(h) => Some(h.borrow().value),
                None => None
            }
        }

        pub fn tail(&self) -> Option<T> {
            if let Some(t) = &self.tail {
                if let Some(_rc) = t.upgrade() {
                    return Some(_rc.borrow().value);
                }
            }
            None
        }

        pub fn push(&mut self, value: T) {
            let new_tail = Rc::new(RefCell::new(Node::new(value)));

            if let Some(t) = &self.tail {
                if let Some(old_tail) = t.upgrade() {
                    old_tail.borrow_mut().next = Some(new_tail.clone());
                } else {
                self.head = Some(new_tail.clone());
                } 
            } else {
                self.head = Some(new_tail.clone())
            }

            self.tail = Some(Rc::downgrade(&new_tail));
        }
    }

}

fn main() {
    use linked_list::LinkedList;

    let mut ll = LinkedList::new(12);

    ll.push(13);
    ll.push(11);
    ll.push(111);

    println!("{:#?}", ll);
}
