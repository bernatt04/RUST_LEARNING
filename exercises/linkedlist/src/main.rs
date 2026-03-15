use std::collections::LinkedList;

pub fn add_to_playlist(
    mut list: LinkedList<String>,
    song: String,
    urgent: bool,
) -> LinkedList<String> {
    if urgent == true {
        list.push_front(song.to_string());
    } else {
        list.push_back(song.to_string());
    }

    list
}

fn main() {
    let mut list: LinkedList<String> = LinkedList::new();

    list = add_to_playlist(list, "si".to_string(), false);
    list = add_to_playlist(list, "adios".to_string(), false);
    list = add_to_playlist(list, "hola".to_string(), true);
    list = add_to_playlist(list, "34aym".to_string(), false);
    list = add_to_playlist(list, "aym".to_string(), false);

    println!("{:?}", list);
}
