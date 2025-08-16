#[derive(Default, Debug)]
struct BitwiseTrie {
    children: [Option<Box<BitwiseTrie>>; 2],
    val: i32,
}

impl BitwiseTrie {
    fn new() -> Self {
        BitwiseTrie {
            children: [None, None],
            val: 0,
        }
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut trie = BitwiseTrie::new();

        for &x in nums.iter() {
            let mut node = &mut trie;

            for i in (0..32).rev() {
                let bit = (x >> i) & 1;
                node = node.children[bit as usize].get_or_insert(Box::new(BitwiseTrie::new()));
            }
            node.val = x;
        } 

        for &x in nums.iter() {
            let mut node = &trie;

            for i in (0..32).rev() {
                let bit = (x >> i) & 1;
                let rev = bit ^ 1;

                if let Some(ref child) = node.children[rev as usize] {
                    node = child;
                } else {
                    node = node.children[bit as usize].as_ref().unwrap();
                }
            }

            res = res.max(x ^ node.val);
        }

        res
    }
}
