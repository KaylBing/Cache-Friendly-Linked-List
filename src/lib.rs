#[derive(Debug)]
struct Node {
    value: u8,
    next: Option<usize>, // Points to the next node's index in the Vec
}

#[derive(Debug)]
pub struct LinkedList {
    nodes: Vec<Node>,    // Backing store for the nodes
    head: Option<usize>, // Index of the head node in the Vec
}

impl LinkedList {
    /// Creates a new empty LinkedList
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
        }
    }

    /// Adds a new value to the front of the linked list
    pub fn push_front(&mut self, value: u8) {
        let new_node = Node {
            value,
            next: self.head, // The new node points to the current head
        };
        self.nodes.push(new_node);
        self.head = Some(self.nodes.len() - 1); // Update head to the new node's index
    }

    /// Iterates over the linked list and prints values
    pub fn print_list(&self) {
        let mut current = self.head;
        while let Some(index) = current {
            let node = &self.nodes[index];
            print!("{} -> ", node.value);
            current = node.next;
        }
        println!("None");
    }
}
