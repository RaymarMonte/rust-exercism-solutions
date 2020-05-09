use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {head: None}
    }

    pub fn len(&self) -> usize {
        let mut len: usize = 0;
        let mut iterator = self.head.as_ref();
        loop {
            iterator = match iterator {
                Some(x) => {
                    len += 1;
                    x.next.as_ref()
                },
                None => {
                    break;
                }
            }
        }
        len
    }

    pub fn push(&mut self, element: T) {
        let head = self.head.take();
        let new_node = Node {data: element, next: head};
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(x) => {
                self.head = x.next;
                Some(x.data)
            },
            None => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(x) => Some(&x.data),
            None => None
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut rev = SimpleLinkedList::new();
        loop {
            match self.pop() {
                Some(x) => rev.push(x),
                None =>  break,
            }
        }
        rev
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut linked_list = SimpleLinkedList::new();
        for item in iter {
            linked_list.push(item);
        }
        linked_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        loop {
            match self.pop() {
                Some(x) => vec.push(x),
                None => break
            }
        }
        vec.reverse();
        vec
    }
}
