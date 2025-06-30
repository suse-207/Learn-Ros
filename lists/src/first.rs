use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
    
        self.head = Link::More(new_node);
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
         // 1. 将 head 替换为 Empty，获取原 head 的所有权
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        
        // 2. 循环处理每个节点
        while let Link::More(mut boxed_node) = cur_link {
            // 3. 将当前节点的next替换为Empty,并获取原next的所有权
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // 4. boxed_node 在此作用域结束时被动释放
        }
        // 5. 当循环结束时，所有节点 都被正确释放
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
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
}
