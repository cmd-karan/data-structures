#[derive(Debug)]
struct Node<i32> {
    data: i32,
    next: Option<Box<Node<i32>>>
}
impl Node<i32> {
    fn new(data: i32) -> Self {
        Self {
            data: data,
            next: None
        }
    }
    fn print(&self) {
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
struct LinkedList<i32> {
    head: Option<Box<Node<i32>>>,
    size: u32
}
impl LinkedList<i32> {
    fn new() -> Self {
        Self {
            head: None,
            size: 0
        }
    }
    fn print(&self) {
        println!("{:#?}", self);
    }
    fn push(&mut self, data: i32) {
        let mut pre_node = Node::new(-1);
        pre_node.next = self.head.take();

        let mut curr = &mut pre_node;
        while let Some(ref mut node) = curr.next {
            curr = node;
        }
        curr.next = Some(Box::new(Node::new(data)));
        self.size += 1;
        self.head = pre_node.next;
    }
}

pub fn driver() {
    println!("---Linked List---");
    let mut linked_list = LinkedList::new();
    linked_list.print();
    linked_list.push(0);
    linked_list.push(1);
    linked_list.push(2);
    linked_list.print();
}