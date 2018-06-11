use std::iter::Iterator;
use std::mem;
use std::rc::Rc;

type NodeLink<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    val: T,
    next: NodeLink<T>,
}

pub struct List<T> {
    head: NodeLink<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, val: T) {
        let next = mem::replace(&mut self.head, None);
        self.head = Some(Rc::new(Node { val, next }));
    }

    pub fn iter(&self) -> ListIterator<T> {
        let next_node = match self.head {
            Some(ref node) => Some(&(**node)),
            None => None,
        };
        ListIterator { next_node }
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        let head = match self.head {
            Some(ref node) => Some(Rc::clone(node)),
            None => None,
        };
        List { head }
    }
}

pub struct ListIterator<'a, T>
where
    T: 'a,
{
    next_node: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let cur_node = mem::replace(&mut self.next_node, None)?;
        let next_link = &cur_node.next;
        if let Some(node) = next_link {
            self.next_node = Some(&*node);
        }
        Some(&cur_node.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::Iterator;

    #[test]
    fn new_list_is_empty() {
        let l: List<i32> = List::new();
        assert_eq!(l.iter().next(), None);
    }

    #[test]
    fn list_is_a_stack() {
        let mut l: List<i32> = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn list_can_branch() {
        let mut l1: List<&str> = List::new();
        l1.push("foo");
        l1.push("bar");
        let mut l2: List<&str> = l1.clone();
        l2.push("baz");
        let l1collected: Vec<&&str> = l1.iter().collect();
        assert_eq!(l1collected, vec![&"bar", &"foo"]);
        let l2collected: Vec<&&str> = l2.iter().collect();
        assert_eq!(l2collected, vec![&"baz", &"bar", &"foo"]);
    }
}
