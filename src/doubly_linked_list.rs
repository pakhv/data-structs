use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};

#[derive(Debug)]
pub struct DoublyLinkedList<T>
where
    T: Clone,
{
    pub head: Option<Rc<RefCell<ListItem<T>>>>,
    pub tail: Option<Rc<RefCell<ListItem<T>>>>,
}

#[derive(Debug)]
pub struct ListItem<T>
where
    T: Clone,
{
    pub next: Option<Rc<RefCell<ListItem<T>>>>,
    pub prev: Option<Rc<RefCell<ListItem<T>>>>,
    pub value: T,
}

impl<T> DoublyLinkedList<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_head(&mut self, value: T) {
        match self.head.take() {
            Some(head) => {
                let new_head = Rc::new(RefCell::new(ListItem {
                    value,
                    next: Some(head.clone()),
                    prev: None,
                }));

                head.borrow_mut().prev = Some(Rc::clone(&new_head));
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(Rc::new(RefCell::new(ListItem {
                    next: None,
                    prev: None,
                    value,
                })));
                self.sync_tail();
            }
        }
    }

    pub fn push(&mut self, value: T) {
        match self.tail.take() {
            Some(tail) => {
                let new_tail = Rc::new(RefCell::new(ListItem {
                    value,
                    next: None,
                    prev: Some(tail.clone()),
                }));

                tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(Rc::new(RefCell::new(ListItem {
                    next: None,
                    prev: None,
                    value,
                })));
                self.sync_head();

                return;
            }
        }
    }

    pub fn pop_head(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                let new_head = head.borrow_mut().next.clone();

                let new_head = match new_head {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;

                        Some(new_head)
                    }
                    None => None,
                };

                self.head = new_head;
                self.sync_tail();

                Some(head.borrow_mut().value.clone())
            }
            None => None,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(tail) => {
                let new_tail = tail.borrow_mut().prev.clone();

                let new_tail = match new_tail {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next = None;

                        Some(new_tail)
                    }
                    None => None,
                };

                self.tail = new_tail;
                self.sync_head();

                Some(tail.borrow_mut().value.clone())
            }
            None => None,
        }
    }

    pub fn iter(&self) -> DoublyLinkedListIter<T> {
        DoublyLinkedListIter(self.head.clone())
    }

    pub fn foreach<F>(&mut self, mut f: F)
    where
        F: FnMut(RefMut<ListItem<T>>),
    {
        for item in self.iter() {
            f(item.borrow_mut());
        }
    }

    pub fn insert_after(&mut self, node: &Rc<RefCell<ListItem<T>>>, value: T) {
        let next_node = node.borrow().next.clone();
        let new_node = Rc::new(RefCell::new(ListItem {
            next: next_node.clone(),
            prev: Some(Rc::clone(&node)),
            value,
        }));

        match next_node.as_ref() {
            Some(next_node) => next_node.borrow_mut().prev = Some(Rc::clone(&new_node)),
            None => self.tail = Some(Rc::clone(&new_node)),
        };

        node.borrow_mut().next = Some(new_node);
    }

    pub fn insert_before(&mut self, node: &Rc<RefCell<ListItem<T>>>, value: T) {
        let prev_node = node.borrow().prev.clone();
        let new_node = Rc::new(RefCell::new(ListItem {
            prev: prev_node.clone(),
            next: Some(Rc::clone(&node)),
            value,
        }));

        match prev_node.as_ref() {
            Some(next_node) => next_node.borrow_mut().next = Some(Rc::clone(&new_node)),
            None => self.head = Some(Rc::clone(&new_node)),
        };

        node.borrow_mut().prev = Some(new_node);
    }

    pub fn remove(&mut self, node: &Rc<RefCell<ListItem<T>>>) {
        match node.borrow().prev.as_ref() {
            Some(prev) => match node.borrow().next.as_ref() {
                Some(next) => {
                    prev.borrow_mut().next = Some(Rc::clone(&next));
                    next.borrow_mut().prev = Some(Rc::clone(&prev));
                }
                None => {
                    prev.borrow_mut().next = None;
                    self.sync_tail();
                }
            },
            None => {
                self.head = node.borrow().next.clone();
                self.sync_tail();
            }
        }
    }

    fn sync_tail(&mut self) {
        if self.head.is_some() && self.head.as_ref().unwrap().borrow().next.is_none() {
            self.tail = Some(Rc::clone(&self.head.as_ref().unwrap()));
            return;
        }

        if self.head.is_none() {
            self.tail = None;
        }
    }

    fn sync_head(&mut self) {
        if self.tail.is_some() && self.tail.as_ref().unwrap().borrow().prev.is_none() {
            self.head = Some(Rc::clone(&self.tail.as_ref().unwrap()));
            return;
        }

        if self.tail.is_none() {
            self.head = None;
        }
    }
}

pub struct DoublyLinkedListIter<T: Clone>(Option<Rc<RefCell<ListItem<T>>>>);

impl<T> Iterator for DoublyLinkedListIter<T>
where
    T: Clone,
{
    type Item = Rc<RefCell<ListItem<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_none() {
            return None;
        }

        let current_item = &self.0.clone().unwrap();
        let next = &current_item.borrow().next;

        let next = next.as_ref();

        self.0 = match next {
            Some(next) => Some(Rc::clone(next)),
            None => None,
        };

        Some(current_item.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_head_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push_head(5);
        list.push_head(25);

        assert_eq!(list.pop_head().unwrap(), 25);
        assert_eq!(list.pop_head().unwrap(), 5);
        assert!(list.pop_head().is_none());
        assert!(list.pop().is_none());
    }

    #[test]
    fn push_pop_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);
        list.push(25);

        assert_eq!(list.pop().unwrap(), 25);
        assert_eq!(list.pop().unwrap(), 5);
        assert!(list.pop().is_none());
        assert!(list.pop_head().is_none());
    }

    #[test]
    fn iter_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);
        list.push(25);
        list.push(50);
        list.push(-50);
        list.push(-25);
        list.push(-5);

        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().borrow().value, 5);
        assert_eq!(iter.next().unwrap().borrow().value, 25);
        assert_eq!(iter.next().unwrap().borrow().value, 50);
        assert_eq!(iter.next().unwrap().borrow().value, -50);
        assert_eq!(iter.next().unwrap().borrow().value, -25);
        assert_eq!(iter.next().unwrap().borrow().value, -5);
        assert!(iter.next().is_none());
    }

    #[test]
    fn foreach_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);
        list.push(25);
        list.push(50);

        list.foreach(|mut i| i.value = i.value * 5);

        assert_eq!(list.pop_head().unwrap(), 25);
        assert_eq!(list.pop_head().unwrap(), 125);
        assert_eq!(list.pop_head().unwrap(), 250);
        assert!(list.pop().is_none());
    }

    #[test]
    fn insert_after_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);
        list.push(25);
        list.push(50);

        let mut iter = list.iter();
        _ = iter.next();
        let second_item = iter.next().unwrap();

        list.insert_after(&second_item, 40);

        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().borrow().value, 5);
        assert_eq!(iter.next().unwrap().borrow().value, 25);
        assert_eq!(iter.next().unwrap().borrow().value, 40);
        assert_eq!(iter.next().unwrap().borrow().value, 50);
        assert!(iter.next().is_none());
    }

    #[test]
    fn insert_after_one_item_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);

        let first = list.head.clone();
        list.insert_after(&first.as_ref().unwrap(), 40);

        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().borrow().value, 5);
        assert_eq!(iter.next().unwrap().borrow().value, 40);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 40);
        assert!(iter.next().is_none());
    }

    #[test]
    fn insert_before_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);
        list.push(25);
        list.push(50);

        let mut iter = list.iter();
        _ = iter.next();
        let second_item = iter.next().unwrap();

        list.insert_before(&second_item, 40);

        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().borrow().value, 5);
        assert_eq!(iter.next().unwrap().borrow().value, 40);
        assert_eq!(iter.next().unwrap().borrow().value, 25);
        assert_eq!(iter.next().unwrap().borrow().value, 50);
        assert!(iter.next().is_none());
    }

    #[test]
    fn insert_before_one_item_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);

        let first = list.head.clone();
        list.insert_before(&first.as_ref().unwrap(), 40);

        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().borrow().value, 40);
        assert_eq!(iter.next().unwrap().borrow().value, 5);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 40);
        assert!(iter.next().is_none());
    }

    #[test]
    fn remove_one_item_list_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);

        let first = list.head.clone();
        list.remove(&first.as_ref().unwrap());

        let mut iter = list.iter();
        assert!(iter.next().is_none());
        assert!(list.head.as_ref().is_none());
        assert!(list.tail.as_ref().is_none());
    }

    #[test]
    fn remove_two_items_list_firt_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);
        list.push(25);

        let first = list.head.clone();
        list.remove(&first.as_ref().unwrap());

        assert_eq!(list.head.as_ref().unwrap().borrow().value, 25);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 25);
    }

    #[test]
    fn remove_two_items_list_second_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);
        list.push(25);

        let second = list.head.as_ref().unwrap().borrow().next.clone();
        list.remove(&second.as_ref().unwrap());

        assert_eq!(list.head.as_ref().unwrap().borrow().value, 5);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 5);
    }

    #[test]
    fn remove_three_items_list_second_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push(5);
        list.push(25);
        list.push(50);

        let second = list.head.as_ref().unwrap().borrow().next.clone();
        list.remove(&second.as_ref().unwrap());

        assert_eq!(list.head.as_ref().unwrap().borrow().value, 5);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 50);
    }
}
