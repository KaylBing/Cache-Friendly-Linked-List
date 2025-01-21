#[derive(Debug)]
struct Node {
    value: u8,
    next: Option<usize>, // Points to the next node's index in the Vec
}

#[derive(Debug)]
struct LinkedList {
    nodes: Vec<Node>,    // Backing store for the nodes
    head: Option<usize>, // Index of the head node in the Vec
}

impl LinkedList {
    /// Creates a new empty LinkedList
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
        }
    }

    /// Adds a new value to the front of the linked list
    fn push_front(&mut self, value: u8) {
        let new_node = Node {
            value,
            next: self.head, // The new node points to the current head
        };
        self.nodes.push(new_node);
        self.head = Some(self.nodes.len() - 1); // Update head to the new node's index
    }

    /// Iterates over the linked list and prints values
    fn print_list(&self) {
        let mut current = self.head;
        while let Some(index) = current {
            let node = &self.nodes[index];
            print!("{} -> ", node.value);
            current = node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_front(10);
    list.push_front(20);
    list.push_front(30);

    println!("Linked list:");
    list.print_list();
}

