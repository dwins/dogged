use ::redblack::RedBlackTree as Tree;

#[test]
fn sorted_results() {
    let mut permutations: Vec<Vec<u32>> = Vec::new();
    fn permute(results: &mut Vec<Vec<u32>>, v: &mut Vec<u32>, i: usize) {
        if i >= v.len() - 1 {
            results.push(v.clone());
        } else {
            for j in i..(v.len()) {
                v.swap(i, j);
                permute(results, v, i + 1);
                v.swap(i, j);
            }
        }
    }

    {
        let mut nums = vec![1,2,3,4,5,6];
        permute(&mut permutations, &mut nums, 6);
    }

    let expected_order = vec![1,2,3,4,5,6];

    for permutation in permutations {
        let tree: Tree<u32> = permutation.iter().cloned().collect();
        let tree_nums: Vec<_> = tree.iter().collect();
        assert_eq!(tree_nums, expected_order);
    }
}

#[test]
fn containment() {
    let tree: Tree<i32> = Tree::new().insert(&0).insert(&1); // (0..4).rev().collect();
    assert!(tree.lookup(&0));
    assert!(!tree.lookup(&20));
    tree.validate();
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
