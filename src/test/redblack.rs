use ::redblack::Tree;

#[test]
fn sorted_results() {
    let tree: Tree<i32> = (0..4).rev().collect();
    let tree_nums: Vec<_> = tree.iter().collect();
    let range_nums: Vec<_> = (0..4).collect();
    assert_eq!(tree_nums, range_nums);
}

#[test]
fn containment() {
    let tree: Tree<i32> = (0..4).rev().collect();
    assert!(tree.lookup(&2));
    assert!(!tree.lookup(&20));
}


#[test]
fn removal() {
    let tree: Tree<i32> = (0..4).rev().collect();
    assert!(tree.lookup(&0));
    assert!(tree.lookup(&1));
    assert!(tree.lookup(&2));
    assert!(tree.lookup(&3));
    let tree = tree.remove(&1);
    assert!( tree.lookup(&0));
    assert!(!tree.lookup(&1));
    assert!( tree.lookup(&2));
    assert!( tree.lookup(&3));
}
