use std::cell::RefCell;
use std::rc::Rc;

type SingleLink<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: SingleLink<T>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None
        }))
    }
}

pub struct List<T> {
    head: SingleLink<T>,
    tail: SingleLink<T>,
    pub length: usize
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn append(&mut self, value: T) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        }
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("pop error")
                .into_inner()
                .value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: List<i32> = List::new();
        if let Some(_) = list.head {
            panic!("head error");
        }
        if let Some(_) = list.tail {
            panic!("tail error");
        }
        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_append() {
        let mut list = List::new();
        list.append(42);
        let Some(head1) = list.head.as_ref() else {
            panic!("head error");
        };
        assert_eq!(head1.borrow().value, 42);
        let Some(tail1) = list.tail.as_ref() else {
            panic!("tail error");
        };
        assert_eq!(tail1.borrow().value, 42);
        assert_eq!(list.length, 1);

        list.append(36);
        let Some(head2) = list.head.as_ref() else {
            panic!("head error");
        };
        assert_eq!(head2.borrow().value, 42);
        let Some(tail2) = list.tail.as_ref() else {
            panic!("tail error");
        };
        assert_eq!(tail2.borrow().value, 36);
        assert_eq!(list.length, 2);
    }

    #[test]
    fn test_pop() {
        let mut list = List::new();
        list.append(42);
        list.append(36);
        list.append(30);

        let Some(first) = list.pop() else {
            panic!("pop error");
        };
        assert_eq!(first, 42);
        assert_eq!(list.length, 2);

        let Some(second) = list.pop() else {
            panic!("pop error");
        };
        assert_eq!(second, 36);
        assert_eq!(list.length, 1);

        let Some(third) = list.pop() else {
            panic!("pop error");
        };
        assert_eq!(third, 30);
        if let Some(_) = list.head {
            panic!("head error");
        }
        if let Some(_) = list.tail {
            panic!("tail error");
        }
        assert_eq!(list.length, 0);
    }
}
