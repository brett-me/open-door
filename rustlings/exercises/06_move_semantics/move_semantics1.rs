// TODO: Fix the compiler error in this function.
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec.to_vec()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let mut vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(&mut vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
