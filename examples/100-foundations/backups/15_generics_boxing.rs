use std::iter::FromIterator;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let top = Box::new(Node {
            value: element,
            next: self.head.take(),
        });
        self.head = Some(top);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(x) => {
                self.head = x.next;
                self.len -= 1;
                Some(x.value)
            }
            None => None,
        }
    }

    pub fn rev(mut self) -> LinkedList<T> {
        let mut list = LinkedList::<T>::new();
        while let Some(x) = self.pop() {
            list.push(x);
        }
        list
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();
        for val in iter {
            list.push(val);
        }
        list
    }
}
fn main() {
    let c = vec![12, 23, 2131, 1231, 4352353, 123];
    let len = c.len();
    let list = LinkedList::from_iter(c);

    println!("{:#?}", list);

    assert_eq!(len, list.len());

    let mut list = list.rev();
    while let Some(x) = list.pop() {
        println!("{}", x);
    }
}
