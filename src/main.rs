// src/main.rs
use Cache_Friendly_Linked_List::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    list.push_front(10);
    list.push_front(20);
    list.push_front(30);

    println!("Linked list:");
    list.print_list();
}
