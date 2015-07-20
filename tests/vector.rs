extern crate dogged;
use dogged::vector::*;

#[test]
fn from_vec65536() {
    let v: Vector<u32> = (0..65536).collect();
    for i in 0..65536 {
        assert_eq!(v[i], i)
    }
}
