use std::fmt::Display;
use sha256::digest;


fn main() {
    let s = "hello";
    let num_leaves = 8;


    let num_nodes = get_num_nodes(num_leaves);
    let mut tree = vec![String::from("_"); num_nodes];

    // set the leaves to be the hash of the data
    for i in 0..num_leaves {
        tree[i as usize] = digest(s.as_bytes());
    }

    pretty_print_tree(&tree[..]);
}

fn next_pow2(n: u32) -> usize {
    if n == 0 {
        return 1;
    }

    let leading_zeros = n.leading_zeros();
    let bits = 32 - leading_zeros;

    if n == 1 << (bits - 1) {
        n as usize
    } else {
        1 << bits
    }
}

fn get_num_nodes(num_leaves: u32) -> usize {
    let num_leaves = next_pow2(num_leaves);
    2 * num_leaves - 1
}

fn pretty_print_tree<T: Display
>(tree: &[T]) {
    let num_nodes = tree.len();
    let num_levels = (num_nodes as f64).log2().ceil() as u32;

    let mut starting_index = 0;
    let mut level_width = 1 << (num_levels - 1);

    while starting_index < num_nodes {
        for node in starting_index..(starting_index + level_width) {
            print!(" {:02} ", tree[node]);
        }

        println!();

        starting_index += level_width;
        level_width >>= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_pow2() {
        assert_eq!(next_pow2(0), 1);
        assert_eq!(next_pow2(1), 1);
        assert_eq!(next_pow2(2), 2);
        assert_eq!(next_pow2(3), 4);
        assert_eq!(next_pow2(4), 4);
        assert_eq!(next_pow2(5), 8);
        assert_eq!(next_pow2(6), 8);
        assert_eq!(next_pow2(7), 8);
        assert_eq!(next_pow2(8), 8);
        assert_eq!(next_pow2(9), 16);
        assert_eq!(next_pow2(10), 16);
    }

    #[test]
    fn test_get_num_nodes() {
        assert_eq!(get_num_nodes(0), 1);
        assert_eq!(get_num_nodes(1), 1);
        assert_eq!(get_num_nodes(2), 3);
        assert_eq!(get_num_nodes(3), 7);
        assert_eq!(get_num_nodes(4), 7);
        assert_eq!(get_num_nodes(5), 15);
        assert_eq!(get_num_nodes(6), 15);
        assert_eq!(get_num_nodes(7), 15);
        assert_eq!(get_num_nodes(8), 15);
        assert_eq!(get_num_nodes(9), 31);
        assert_eq!(get_num_nodes(10), 31);
    }
}
