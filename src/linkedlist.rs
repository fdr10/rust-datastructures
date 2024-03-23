use crate::nodes::DoublyNode;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<DoublyNode<T>>>>,
    tail: Option<Rc<RefCell<DoublyNode<T>>>>,
    size: usize,
}

impl<T: Copy + Debug> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    // This is a comment
    pub fn pop_front(&mut self) -> Option<Rc<RefCell<DoublyNode<T>>>> {
        match &mut self.head {
            None => None,
            Some(head) => match &mut self.tail {
                None => None,
                Some(tail) => {
                    let popped_node = head.clone();
                    if Rc::ptr_eq(&head, &tail) {
                        self.head = None;
                        self.tail = None;
                    } else {
                        let next = head.borrow_mut().next.clone();
                        head.borrow_mut().swap(None);
                        self.head = next;
                    }

                    self.size -= 1;
                    Some(popped_node)
                }
            },
        }
    }

    pub fn push_front(&mut self, value: T) {
        let node = Rc::new(RefCell::new(DoublyNode::new(value)));
        match &mut self.head {
            // If the head is empty, make the head and tail point to the new node
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
            // If there is a head we set the prev to the new node, set the new node's next to head and update the self.head to point to the new node.
            Some(head) => {
                head.borrow_mut().set_prev(Some(node.clone()));
                node.borrow_mut().set_next(Some(head.clone()));
                self.head = Some(node.clone());
            }
        }
        self.size += 1;
    }
    pub fn pop_back(&mut self) -> Option<Rc<RefCell<DoublyNode<T>>>> {
        match &mut self.tail {
            None => None,
            Some(tail) => match &mut self.head {
                None => None,
                Some(head) => {
                    let popped_node = tail.clone();
                    if Rc::ptr_eq(&head, &tail) {
                        self.head = None;
                        self.tail = None;
                    } else {
                        let prev = tail.borrow().prev.clone();
                        tail.borrow_mut().swap(None);
                        if let Some(prev) = prev {
                            if let Some(prev) = prev.upgrade() {
                                self.tail = Some(prev);
                            }
                        }
                    }

                    self.size -= 1;
                    Some(popped_node)
                }
            },
        }
    }
    pub fn push_back(&mut self, value: T) {
        let node = Rc::new(RefCell::new(DoublyNode::new(value)));
        match &mut self.tail {
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
            Some(tail) => {
                tail.borrow_mut().set_next(Some(node.clone()));
                node.borrow_mut().set_prev(Some(tail.clone()));
                self.tail = Some(node.clone());
            }
        }
        self.size += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_linkedlist() {
        let mut dll = DoublyLinkedList::<i32>::new();
        dll.push_front(1);
        dll.push_front(2);
        dll.push_front(3);
        dll.push_front(4);
        dll.push_front(5);
        assert_eq!(dll.pop_front().unwrap().borrow_mut().value, 5);
        assert_eq!(dll.pop_front().unwrap().borrow_mut().value, 4);
        assert_eq!(dll.pop_front().unwrap().borrow_mut().value, 3);
        assert_eq!(dll.pop_front().unwrap().borrow_mut().value, 2);
        assert_eq!(dll.pop_front().unwrap().borrow_mut().value, 1);
        assert!(dll.pop_front().is_none());
        assert!(dll.tail.is_none());
        dll.push_back(1);
        dll.push_back(2);
        dll.push_back(3);
        dll.push_back(4);
        dll.push_back(5);
        assert_eq!(dll.pop_back().unwrap().borrow().value, 5);
        assert_eq!(dll.pop_back().unwrap().borrow().value, 4);
        assert_eq!(dll.pop_back().unwrap().borrow().value, 3);
        assert_eq!(dll.pop_back().unwrap().borrow().value, 2);
        assert_eq!(dll.pop_back().unwrap().borrow().value, 1);
        assert!(dll.pop_back().is_none());
    }
}
