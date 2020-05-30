use crate::dd_challenge::Solution;
use std::cmp::Ordering;
use std::str::FromStr;
use std::string::ToString;

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut r: Vec<Log> = logs.into_iter()
            .map(|log| log.parse::<Log>().unwrap()).collect();
        r.sort();
        r.iter().map(|x| x.to_string()).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum Log {
    Digit(String, String),
    Letter(String, String),
}

impl Log {
    pub fn is_digit(&self) -> bool {
        match &self {
            Log::Digit(_, _) => true,
            _ => false
        }
    }
}

impl PartialOrd for Log {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.is_digit() && !self.is_digit() {
            Some(Ordering::Less)
        } else if !other.is_digit() && self.is_digit() {
            Some(Ordering::Greater)
        } else if self.is_digit() && other.is_digit() {
            Some(Ordering::Equal)
        } else {
            let mut res = Ordering::Equal;

            let (k1, v1) = if let Log::Letter(id, v) = &self {
                (id, v)
            } else {
                return Some(res)
            };

            let (k2, v2) = if let Log::Letter(id, v) = other {
                (id, v)
            } else {
                return Some(res);
            };

            res = v1.cmp(v2);

            if res == Ordering::Equal {
                res = k1.cmp(k2);
            }

            Some(res)

        }

    }
}

impl FromStr for Log {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tok: Vec<&str> = s.splitn(2, ' ').collect();
        if (tok[1].as_bytes()[0] as char).is_digit(10) {
            Ok(Log::Digit(tok[0].to_string(), tok[1].to_string()))
        } else {
            Ok(Log::Letter(tok[0].to_string(), tok[1].to_string()))
        }
    }
}
impl ToString for Log {
    fn to_string(&self) -> String {
        match &self {
            Log::Letter(k, v) => format!("{} {}", k, v),
            Log::Digit(k, v) => format!("{} {}", k, v),
        }
    }
}

/*
impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Log::Letter(k, v) => write!(f, "Letter: {}, {} ", k, v),
            Log::Digit(k, v) => write!(f, "Digit: {}, {}", k, v),
        }
    }
}*/

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    fn test_reorder_log_files() {
        let input = vec!["dig1 8 1 5 1".to_string(),"let1 art can".to_string(),
            "dig2 3 6".to_string(),"let2 own kit dig".to_string(),"let3 art zero".to_string()];
        let expected = vec!["let1 art can".to_string(),"let3 art zero".to_string(),
                            "let2 own kit dig".to_string(),"dig1 8 1 5 1".to_string(),
                            "dig2 3 6".to_string()];
        assert_eq!(Solution::reorder_log_files(input), expected);
    }
}