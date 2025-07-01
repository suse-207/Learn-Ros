pub struct List<T> {
    head: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T>{
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>,
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
            elem: elem,
            next: self.head.take(),
        });
    
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // peek 方法会检查列表是否为空。若列表不为空，返回指向列表头部元素的不可变引用
    pub fn peek(&self) -> Option<&T> {
        // self.head 类型为 Option<Box<Node<T>>>，as_ref() 方法将 Option<Box<Node<T>>> 转换为 Option<&Box<Node<T>>>，这样做是为了在不转移 self.head 所有权的情况下访问其内部值。
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    // 定义一个公共方法 into_iter，接收 self 作为参数，意味着会获取 List<T> 实例的所有权
    // 返回类型为 IntoIter<T>，即一个迭代器类型
    pub fn into_iter(self) -> IntoIter<T> {
        // 创建一个 IntoIter<T> 实例，将当前的 List<T> 实例作为参数传入
        // IntoIter 是一个元组结构体，这里用传入的 List<T> 实例初始化它
        IntoIter(self)
    }

    // 定义一个公共方法 iter，它是一个泛型方法，包含生命周期参数 'a
    // &'a self 表示接收一个对 List<T> 实例的不可变引用，该引用的生命周期为 'a
    // 返回类型为 Iter<'a, T>，意味着返回的迭代器与传入的 List<T> 实例具有相同的生命周期 'a
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // 创建一个 Iter<'a, T> 结构体实例
        Iter {
            // self.head 的类型是 Link<T>，即 Option<Box<Node<T>>>
            // as_deref() 方法将 Option<Box<Node<T>>> 转换为 Option<&Node<T>>
            // 这样做是为了在不转移 self.head 所有权的情况下，获取其内部 Node<T> 的不可变引用
            // 最终将这个 Option<&Node<T>> 赋值给 Iter 结构体的 next 字段
            next: self.head.as_deref()
        }
    }

    //note:
    // 1. 调用 as_deref() 方法后，Option<&Node<T>> 表示一个 Option，它可能包含一个 &Node<T> 类型的引用，也可能为 None
    // 2. 当 Option 包含 &Node<T> 时，as_deref() 方法返回一个 Option<&Node<T>>，表示存在一个不可变引用
    // 3. 当 Option 为 None 时，as_deref() 方法返回 None，表示不存在引用
    // 4. 这个过程不移动 self.head 的值，只是借用其内部值的引用
    // 5. 借用的引用的生命周期与 self.head 相同，即 'a，确保在使用引用的过程中 self.head 不会失效

    // 6. 总结：
    //    - self.head.as_deref() 将 Option<Box<Node<T>>> 转换为 Option<&Node<T>>，表示存在一个不可变引用
    //    - 借用的引用的生命周期与 self.head 相同，确保在使用引用的过程中 self.head 不会失效
    //    - 不移动 self.head 的值，只是借用其内部值的引用
    // 7. 注意：
    //    - as_deref() 方法不会移动 self.head 的值，只是借用其内部值的引用
    //    - 借用的引用的生命周期与 self.head 相同，确保在使用引用的过程中 self.head 不会失效
    //    - 不移动 self.head 的值，只是借用其内部值的引用

    // 8. as_ref 与 as_deref 区别：
    //    - as_ref 用于简单地将 Option<T> 转换为 Option<&T>；as_deref 则针对 Option 内部是实现了 Deref trait 的类型，会先解引用再转换为引用。
    //    - as_ref 适用于任意 Option<T> 类型；as_deref 适用于 Option<T> 中 T 实现了 Deref trait 的情况

    pub fn iter_mut(&mut self) -> IterMut<'_,T>{
        IterMut{
            next: self.head.as_deref_mut(),
        }
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
         // 1. 将 head 替换为 None，获取原 head 的所有权
        let mut cur_link = self.head.take();
        
        // 2. 循环处理每个节点
        while let Some(mut boxed_node) = cur_link {
            // 3. 将当前节点的next替换为None,并获取原next的所有权
            cur_link = boxed_node.next.take();
            // 4. boxed_node 在此作用域结束时被动释放
        }
        // 5. 当循环结束时，所有节点 都被正确释放
    }
}

// 为 IntoIter<T> 结构体实现 Iterator 特征，泛型参数为 T
impl<T> Iterator for IntoIter<T> {
    // 关联类型 Item，指定迭代器产生的值的类型。这里设置为 T，意味着迭代器会逐个产生 T 类型的值
    type Item = T;

    // 实现 Iterator 特征的 next 方法，该方法用于获取迭代器的下一个值
    // &mut self 表示获取 IntoIter<T> 实例的可变引用，调用该方法可能会改变迭代器的内部状态
    // 返回值类型为 Option<Self::Item>，即 Option<T>。如果有下一个值则返回 Some(T)，否则返回 None
    fn next(&mut self) -> Option<Self::Item> {
        // self.0 访问 IntoIter<T> 元组结构体中的第一个元素，也就是内部封装的 List<T> 实例
        // 调用 List<T> 的 pop 方法，该方法会从链表头部移除一个元素并返回该元素的值
        // 如果链表不为空，pop 方法返回 Some(T)；如果链表为空，返回 None
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) ->Option<Self::Item>{
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a,T> Iterator for IterMut<'a,T>{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        list.peek_mut().map(|value| {
            *value = 42;
        });
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

        #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }


}
