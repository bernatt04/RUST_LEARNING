use std::collections::HashMap;

pub fn contar_animales(nombres: Vec<String>) -> HashMap<String, u32> {
    let mut my_map = HashMap::new();
    for name in nombres {
        *my_map.entry(name).or_insert(0) += 1;
    }
    println!("{:?}", my_map);
    return my_map;
}

fn main() {
    let mut vec = Vec::new();
    vec.push("dog".to_string());
    vec.push("dog".to_string());
    vec.push("cat".to_string());
    vec.push("dog".to_string());
    contar_animales(vec);
}
