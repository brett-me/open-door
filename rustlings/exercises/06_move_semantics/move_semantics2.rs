fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec.to_vec()
}

fn main() {
    // You can optionally experiment here.
    let mut vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(&mut vec0);
    println!("vec0 = {:?}... vec1 = {:?}", vec0, vec1);
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let mut vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(&mut vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
