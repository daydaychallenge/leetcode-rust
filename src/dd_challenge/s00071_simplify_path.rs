use super::Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut path_stack: Vec<_> = vec![];

        for p in path.split("/") {
            if p == ".." {
                path_stack.pop();
            } else if p == "." {
                continue;
            } else {
                if !p.is_empty() {
                    path_stack.push(p);
                }
            }
        }

        // "/".to_string() + &path_stack.join("/")
        format!("/{}", path_stack.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_simplify_path() {
        assert_eq!(Solution::simplify_path("/home/".to_string()), "/home".to_string());
        assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());
        assert_eq!(Solution::simplify_path("/home//foo/".to_string()), "/home/foo".to_string());
        assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_string()), "/c".to_string());
        assert_eq!(Solution::simplify_path("/a/../../b/../c//.//".to_string()), "/c".to_string());
        assert_eq!(Solution::simplify_path("/a//b////c/d//././/..".to_string()), "/a/b/c".to_string());
    }
}