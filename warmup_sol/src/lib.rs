pub fn fibonacci_recursive(n: u32) -> u32 { //inefficient
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

pub fn fibonacci_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a; //shadowing
        a = b;
        b = temp + b;
    }
    a
}

#[cfg(test)]
mod fibonacci_tests {
    use super::*;

    #[test]
    fn test_fibonacci_recursive() {
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
        assert_eq!(fibonacci_recursive(5), 5);
    }

    #[test]
    fn test_fibonacci_iterative() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(5), 5);
    }
}

pub fn longest_str<'a, 'b>(strings: &'b [&'a str]) -> &'a str {
    if strings.is_empty() {
        panic!("The input slice is empty");
    }

    let mut longest = strings[0];
    for &string in strings.iter() {
        if string.len() > longest.len() {
            longest = string;
        }
    }

    longest
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
        let left_sum = match &self.left {
            Some(left) => left.sum(),
            None => 0,
        };
        let right_sum = match &self.right {
            Some(right) => right.sum(),
            None => 0,
        };
        self.value + left_sum + right_sum
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
