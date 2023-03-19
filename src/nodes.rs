use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[allow(dead_code)]
#[derive(Debug)]
pub struct DoublyNode<T> {
    pub value: T,
    pub prev: Option<Weak<RefCell<DoublyNode<T>>>>,
    pub next: Option<Rc<RefCell<DoublyNode<T>>>>,
}

impl<T: Copy> DoublyNode<T> {
    pub fn new(val: T) -> Self {
        DoublyNode {
            value: val,
            prev: None,
            next: None,
        }
    }
    pub fn set_next(&mut self, node: Option<Rc<RefCell<DoublyNode<T>>>>) {
        match node {
            None => self.next = None,
            Some(node) => self.next = Some(node),
        }
    }

    pub fn set_prev(&mut self, node: Option<Rc<RefCell<DoublyNode<T>>>>) {
        match node {
            None => self.prev = None,
            Some(node) => {
                self.prev = Some(Rc::downgrade(&node));
            }
        }
    }
    pub fn swap(&mut self, node: Option<Rc<RefCell<DoublyNode<T>>>>) -> Option<DoublyNode<T>> {
        match node {
            None => {
                match &self.next {
                    None => {
                        match &mut self.prev {
                            None => {} // Do nothing
                            Some(prev) => {
                                let prev = prev.upgrade();
                                if let Some(prev) = prev {
                                    prev.borrow_mut().next = None;
                                }
                            }
                        }
                    }
                    Some(next) => match &mut self.prev {
                        None => {
                            next.borrow_mut().prev = None;
                        }
                        Some(prev) => {
                            next.borrow_mut().prev = Some(prev.clone());
                            let prev = prev.upgrade();
                            if let Some(prev) = prev {
                                prev.borrow_mut().next = Some(next.clone());
                            }
                        }
                    },
                }
            }
            Some(node) => {
                let node: Option<Rc<RefCell<DoublyNode<T>>>> = Some(node);

                match &mut self.next {
                    None => {}
                    Some(next) => {
                        if let Some(node_rc) = node.as_ref() {
                            next.borrow_mut().prev = Some(Rc::downgrade(node_rc));
                            node_rc.borrow_mut().next = Some(next.clone());
                        }
                    }
                }
                match &mut self.prev {
                    None => {}
                    Some(prev) => {
                        if let Some(node_rc) = node.as_ref() {
                            let prev = prev.upgrade();
                            if let Some(prev) = prev {
                                prev.borrow_mut().next = Some(node_rc.clone());
                                node_rc.borrow_mut().prev = Some(Rc::downgrade(&prev));
                            }
                        }
                    }
                }
            }
        }

        Some(DoublyNode::new(self.value))
    }
}

impl<T: Copy> From<DoublyNode<T>> for Option<Rc<RefCell<DoublyNode<T>>>> {
    fn from(node: DoublyNode<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_nodes_and_swap_node() {
        // Create the nodes
        let node_1 = Rc::new(RefCell::new(DoublyNode::<i32>::new(1)));
        let node_2 = Rc::new(RefCell::new(DoublyNode::<i32>::new(2)));
        let node_3 = Rc::new(RefCell::new(DoublyNode::<i32>::new(3)));
        let node_4 = Rc::new(RefCell::new(DoublyNode::<i32>::new(4)));

        // Set the next pointers
        node_1.borrow_mut().set_next(Some(node_2.clone()));
        node_2.borrow_mut().set_next(Some(node_3.clone()));
        node_3.borrow_mut().set_next(Some(node_4.clone()));
        assert!(node_1.borrow().next.as_ref().unwrap().borrow().value == node_2.borrow().value);
        assert!(node_2.borrow().next.as_ref().unwrap().borrow().value == node_3.borrow().value);
        assert!(node_3.borrow().next.as_ref().unwrap().borrow().value == node_4.borrow().value);

        // Set the prev pointers
        node_4.borrow_mut().set_prev(Some(node_3.clone()));
        node_3.borrow_mut().set_prev(Some(node_2.clone()));
        node_2.borrow_mut().set_prev(Some(node_1.clone()));

        // Remove node_2
        node_2.borrow_mut().swap(None);
        assert!(node_1.borrow().next.as_ref().unwrap().borrow().value == node_3.borrow().value);
        assert!(
            node_3
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .value
                == node_1.borrow().value
        );
        // Remove node node_1
        // Check if prev node_3 is none
        node_1.borrow_mut().swap(None);
        assert!(node_3.borrow().prev.is_none());
        assert!(
            node_4
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .value
                == node_3.borrow().value
        );

        //Remove node_4
        // Check if next of node_3 is none

        node_4.borrow_mut().swap(None);
        assert!(node_3.borrow().next.is_none());
    }

    #[test]
    fn build_nodes() {
        // Create the nodes
        let node_1 = Rc::new(RefCell::new(DoublyNode::<i32>::new(1)));
        let node_2 = Rc::new(RefCell::new(DoublyNode::<i32>::new(2)));
        let node_3 = Rc::new(RefCell::new(DoublyNode::<i32>::new(3)));
        let node_4 = Rc::new(RefCell::new(DoublyNode::<i32>::new(4)));

        let node_40 = Rc::new(RefCell::new(DoublyNode::<i32>::new(40)));
        let node_50 = Rc::new(RefCell::new(DoublyNode::<i32>::new(50)));
        let node_60 = Rc::new(RefCell::new(DoublyNode::<i32>::new(60)));
        // Set the next pointers
        node_1.borrow_mut().set_next(Some(node_2.clone()));
        node_2.borrow_mut().set_next(Some(node_3.clone()));
        node_3.borrow_mut().set_next(Some(node_4.clone()));
        assert!(node_1.borrow().next.as_ref().unwrap().borrow().value == node_2.borrow().value);
        assert!(node_2.borrow().next.as_ref().unwrap().borrow().value == node_3.borrow().value);
        assert!(node_3.borrow().next.as_ref().unwrap().borrow().value == node_4.borrow().value);

        // Set the prev pointers
        node_4.borrow_mut().set_prev(Some(node_3.clone()));
        node_3.borrow_mut().set_prev(Some(node_2.clone()));
        node_2.borrow_mut().set_prev(Some(node_1.clone()));
        assert!(
            node_4
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .value
                == node_3.borrow().value
        );
        assert!(
            node_3
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .value
                == node_2.borrow().value
        );
        assert!(
            node_2
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .value
                == node_1.borrow().value
        );
        // Start swapping nodes
        node_1.borrow_mut().swap(Some(node_40.clone()));
        node_4.borrow_mut().swap(Some(node_60.clone()));
        assert!(
            node_40.borrow_mut().next.as_ref().unwrap().borrow().value == node_2.borrow().value
        );
        assert!(
            node_60
                .borrow_mut()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .value
                == node_3.borrow().value
        );
        node_2.borrow_mut().swap(Some(node_50.clone()));

        assert!(
            node_50.borrow_mut().next.as_ref().unwrap().borrow().value == node_3.borrow().value
        );
        assert!(
            node_50
                .borrow_mut()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .value
                == node_40.borrow().value
        );
    }
}
