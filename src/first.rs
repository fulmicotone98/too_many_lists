use std::mem;

pub struct List {
    head: Link,
}
#[derive(Debug, PartialEq)]
enum Link {
    Empty,
    More(Box<Node>),
}
#[derive(Debug, PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}

// With this Linked List structure we achived:
// - tail of a list never allocates extra junk: done!
// - enum is in a null-pointer-optimized form: done!
// - all elements uniformly allocated: done!

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let next_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        // We replaced self.head temporarly with Link::Empty, before replace it with the next_node
        self.head = Link::More(next_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        // println!("Curr. Link: {:?}", cur_link);
        while let Link::More(mut boxed_node) = cur_link {
            println!("Boxed Node: {:?}", boxed_node);
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
        // println!("Curr. Link: {:?}", cur_link);
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate the list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check pop on non-empty list (LIFO)
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more
        list.push(4);

        // Check exhaustion
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
