pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            (2..=n)
                .fold((1, 0), |(last, previous), _| (last + previous, last))
                .0
        }
    }
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

pub fn longest_str<'a, 'b>(texts: &'b [&'a str]) -> &'a str {
    texts
        .iter()
        .fold(None, |longest: Option<&'a str>, text| {
            Some(
                longest
                    .map(|longest| {
                        if text.len() > longest.len() {
                            text
                        } else {
                            longest
                        }
                    })
                    .unwrap_or(text),
            )
        })
        .unwrap()
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

    pub fn get_value_if_inner(&self) -> Option<i32> {
        todo!("Return the value of self iff it has 2 children, None otherwise")
    }
}

#[cfg(test)]
mod tree_tests {
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

    #[test]
    fn test_get_inner_value() {
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
        assert_eq!(tree.left.unwrap().get_value_if_inner(), None);
    }
}
