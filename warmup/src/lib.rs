pub fn fibonacci(n: u32) -> u32 {
    todo!("Implement a function that returns the n-th fibonacci number, starting with 0, 1")
}

#[cfg(test)]
mod fibonacci_tests {
    use super::fibonacci;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5)
    }
}

pub fn longest_str<'a, 'b>(strings: &'b [&'a str]) -> &'a str {
    todo!(
        "Return the longest string slice of those passed in, if nothing is passed in, you should panic!()
        If there are multiple strings of the same length are all longest, return the first one"
    )
}

#[cfg(test)]
mod longest_str_tests {
    use super::longest_str;

    #[test]
    fn test_longest_str() {
        let strings = ["aes", "des", "ed25519", "elgamal"];
        assert_eq!(longest_str(&strings), "ed25519");
    }
}

pub struct Tree {
    pub value: i32,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

impl Tree {
    pub fn sum(&self) -> i32 {
        todo!("Traverse and return the sum of the tree")
    }
}

#[cfg(test)]
mod tree_sum_tests {
    use super::Tree;

    #[test]
    fn test_tree_sum() {
        let tree = Tree {
            value: 7,
            left: Some(Box::new(Tree {
                value: 5,
                left: None,
                right: Some(Box::new(Tree {
                    value: 10,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(Tree {
                value: 3,
                left: None,
                right: None,
            })),
        };
        assert_eq!(tree.sum(), 25);
    }
}
