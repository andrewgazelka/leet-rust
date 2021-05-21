use std::ops::{Index, IndexMut};

use crate::Solution;

struct Arr2D<T> {
    backing: Vec<T>,
    width: usize,
}

impl<T: Clone> Arr2D<T> {
    fn new(x: usize, y: usize, default: T) -> Arr2D<T> {
        Self {
            backing: vec![default; x * y],
            width: x,
        }
    }
}

impl<T> Index<(usize, usize)> for Arr2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let x = index.0;
        let y = index.1;
        let idx = x + y * self.width;
        &self.backing[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Arr2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let x = index.0;
        let y = index.1;
        let idx = x + y * self.width;
        &mut self.backing[idx]
    }
}

enum Matcher {
    Exact(char),
    Any,
}

impl Matcher {
    fn is_valid(&self, other: char) -> bool {
        match self {
            Matcher::Exact(val) => *val == other,
            Matcher::Any => true
        }
    }
}

#[derive(Copy, Clone)]
enum PatternKind {
    One,
    Wildcard
}

struct PatternElem {
    matcher: Matcher,
    kind: PatternKind,
}

type Pattern = Vec<PatternElem>;

enum CompileError {
    UnpairedWildcard
}

fn character_matcher(character: char) -> Matcher {
    match character {
        '.' => Matcher::Any,
        _ => Matcher::Exact(character)
    }
}

fn compile_pattern(pattern_str: &str) -> Result<Pattern, CompileError> {
    let pattern_str: Vec<_> = pattern_str.chars().collect();


    let mut vec = Vec::new();

    if pattern_str.is_empty() {
        return Ok(vec)
    }

    let mut p_idx = pattern_str.len() - 1;

    loop {
        let mut character_on = pattern_str[p_idx];
        let wildcard = character_on == '*';

        if wildcard {
            if p_idx == 0 {
                return Err(CompileError::UnpairedWildcard);
            }

            p_idx -= 1;
            character_on = pattern_str[p_idx];
        }

        let matcher = character_matcher(character_on);

        let kind = if wildcard {
            PatternKind::Wildcard
        } else {
            PatternKind::One
        };

        vec.push(PatternElem {
            matcher,
            kind
        });

        if p_idx > 0 {
            p_idx -= 1;
        } else {
            break
        }
    }
    vec.reverse();
    Ok(vec)
}

impl Solution {
    /// support *, .
    /// match *entire* string
    pub fn is_match(string: String, pattern: String) -> bool {

        let string: Vec<_> = string.chars().collect();
        let pattern = compile_pattern(&pattern).ok().unwrap();

        let str_len = string.len();
        let pattern_len = pattern.len();


        let mut dp = Arr2D::new(str_len + 1, pattern_len + 1, false);

        dp[(str_len, pattern_len)] = true;

        // for each character we want to see how far we can get back

        for s_idx in (0..=str_len).rev() {
            for p_idx in (0..pattern_len).rev() {
                let pc = &pattern[p_idx];

                let matches = if s_idx == str_len {
                    false
                } else {
                    pc.matcher.is_valid(string[s_idx])
                };

                let res = match pc.kind {
                    PatternKind::One => {
                        matches && dp[(s_idx +1, p_idx + 1)]
                    },
                    PatternKind::Wildcard => {
                        (matches && dp[(s_idx + 1, p_idx )]) || dp[(s_idx, p_idx + 1 )]
                    }
                };

                dp[(s_idx, p_idx)] = res;
            }
        }

        dp[(0, 0)]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    fn is_match(a: &str, b: &str) -> bool {
        Solution::is_match(a.to_string(), b.to_string())
    }

    #[test]
    fn it_works() {
        assert!(!is_match("a", ""));
        assert!(!is_match("aa", "a"));
        assert!(is_match("aa", "a*"));
        assert!(is_match("ab", ".*"));
        assert!(is_match("aab", "c*a*b"));
        assert!(!is_match("mississippi", "mis*is*p*."));
    }
}
