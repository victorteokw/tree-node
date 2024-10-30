use tree_node::tree::Tree;

#[test]
fn test_tree() {
    let tree = Tree::new("Kunkun".to_owned());
    let cici = tree.root().add_child("Cici".to_owned());
    let cici_value = cici.value();
    cici.add_child("Coco".to_owned());
    tree.root().add_child("Loves".to_owned());
    println!("see cici value: {:?}", cici_value);
    println!("see tree {:?}", tree);
    assert_eq!(true, true);
}