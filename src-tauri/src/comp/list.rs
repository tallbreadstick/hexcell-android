/*

MY SINGLY LINKED LIST IMPLEMENTATION

In order to efficiently reverse the char results from converting from decimal, I had to
use some sort of stack. I decided to use a simple singly linked list using Box<_> rather
than Rc<RefCell<_>>, since it is only uni-directional.

*/

pub struct ListNode<T> {
    pub data: T,
    pub next: Option<Box<ListNode<T>>>
}

impl<T> ListNode<T> {
    pub fn new(data: T) -> ListNode::<T> {
        ListNode::<T> { data, next: None }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
    count: usize
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList::<T> {
        LinkedList::<T> { head: None, count: 0 }
    }
    // THESE ARE NOT NECESSARY AND THE COMPILER IS WHINING, SO AWAY WITH THESE METHODS
    // pub fn is_empty(&self) -> bool {
    //     self.head.is_none()
    // }
    // pub fn len(&self) -> usize {
    //     self.count
    // }
    pub fn push(&mut self, data: T) {
        let mut node = Box::new(ListNode::new(data));
        node.next = self.head.take();
        self.head = Some(node);
        self.count += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.count -= 1;
            node.data
        })
    }
}