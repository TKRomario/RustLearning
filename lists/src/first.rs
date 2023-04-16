use std::mem;

// Tail recursion: 
// recursive function call which is the last statement of the function and just returns the value of the recursively called function

// Bad since enum Type allocates first element on stack and additionally auto create mpty Junk node at the end
// pub enum List {
//     Empty,
//     Elem(i32, Box<List>)   
// }

pub struct List {
    head: Link,
}

struct Node {
    elem : i32,
    next: Link,
}

// Enums are transparent thus all of the Types used must be public
enum Link {
    Empty,
    More(Box<Node>),
}

// impl associates Code with a type
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
        // last expression of a function is automatically returned
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    // Option is a value that might exist or is none
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head , Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
        // Makro for denoting not implemented code panics on executiom
        //unimplemented!()
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }
}