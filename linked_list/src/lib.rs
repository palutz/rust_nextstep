pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // map instead of match .. { None => None, Some => Some }
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::List;
    #[test]
    fn test01() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test02() {
        // testing generics (using &str)
        let mut list = List::new();

        list.push("Ciao");
        list.push("Hello");

        let s = list.pop();

        assert_eq!(s, Some("Hello"));
    }
    #[test]
    fn test_peek() {
        let mut list = List::new();

        list.push("Ciao");
        list.push("Hello");

        let s = list.peek();
        assert_eq!(s, Some(&"Hello"));
        // testing peek_mut
        let sm = list.peek_mut().map(|_| "abcd");
        assert_eq!(sm, Some("abcd"));
        // and now modifying inside the list
        list.peek_mut().map(|val| *val = "abcd");
        assert_eq!(Some(&"abcd"), list.peek());

        let mut alist = List::new();

        alist.push(1);
        alist.push(2);
        alist.push(3);

        alist.peek_mut().map(|val| *val = 13);
        assert_eq!(Some(&13), alist.peek());
    }
}
