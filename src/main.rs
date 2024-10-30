use crate::tree::Tree;

pub mod tree;
pub mod node;

fn main() {
    let mut tree = Tree::new("Kunkun".to_owned());
    let cici = tree.root_mut().add_child("Cici".to_owned());
    let cici_value = cici.value();
    cici.add_child("Coco".to_owned());
    tree.root_mut().add_child("Loves".to_owned());
    println!("see cici value: {:?}", cici_value);

    println!("see tree {:?}", tree);
}