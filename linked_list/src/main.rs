// std::collections::linked_list;
mod linked_list {
    use std::cell::RefCell;
    use std::fmt::Display;
    use std::rc::{Rc, Weak};

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
                value: value,
                next: None,
            }
        }
    }

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

        pub fn head(&self) -> &Option<Rc<RefCell<Node<T>>>> {
            &self.head
        }

        pub fn push(&mut self, value: T) {
            let new_tail = Rc::new(RefCell::new(Node::new(value)));

            match &self.tail {
                Some(old) => {
                    match old.upgrade() {
                        Some(old_tail) => {
                            old_tail.borrow_mut().next = Some(new_tail.clone());
                        }
                        None => {
                            self.head = Some(new_tail.clone());
                        }
                    }
                }
                None => {
                    self.head = Some(new_tail.clone());
                }
            }

            self.tail = Some(Rc::downgrade(&new_tail));
        }

        pub fn print(ll: LinkedList<T>) {
            let mut node = ll.head.clone();

            while let Some(v) = node {
                println!("{}", v.borrow().value);
                node = v.borrow().next.clone();
            }
        }
    }
}

fn main() {
    use linked_list::LinkedList;

    let mut a = LinkedList::new(12);

    a.push(13);
    a.push(11);
    a.push(111);

    LinkedList::print(a);
}
