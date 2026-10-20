//! Case-sensitive shell-style glob matching with the semantics of Python's
//! `fnmatch.fnmatchcase` (3.13). `*` and `?` also match newlines. Trap T-16.

/// One compiled pattern element.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Literal(char),
    Any,
    Star,
    Set {
        negated: bool,
        items: Vec<SetItem>,
    },
    /// A bracket set that can never match (Python's `(?!)`).
    Never,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SetItem {
    Char(char),
    Range(char, char),
}

/// A compiled glob pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Glob {
    tokens: Vec<Token>,
}

impl Glob {
    /// Compile `pattern`. Every string is a valid pattern.
    #[must_use]
    pub fn new(pattern: &str) -> Self {
        let chars: Vec<char> = pattern.chars().collect();
        let mut tokens = Vec::new();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            i += 1;
            match c {
                '*' => {
                    if tokens.last() != Some(&Token::Star) {
                        tokens.push(Token::Star);
                    }
                }
                '?' => tokens.push(Token::Any),
                '[' => {
                    let mut j = i;
                    if j < chars.len() && chars[j] == '!' {
                        j += 1;
                    }
                    if j < chars.len() && chars[j] == ']' {
                        j += 1;
                    }
                    while j < chars.len() && chars[j] != ']' {
                        j += 1;
                    }
                    if j >= chars.len() {
                        tokens.push(Token::Literal('['));
                    } else {
                        tokens.push(compile_set(&chars[i..j]));
                        i = j + 1;
                    }
                }
                other => tokens.push(Token::Literal(other)),
            }
        }
        Self { tokens }
    }

    /// True when the whole of `name` matches the pattern.
    #[must_use]
    pub fn is_match(&self, name: &str) -> bool {
        let text: Vec<char> = name.chars().collect();
        let (mut t, mut p) = (0, 0);
        // Backtrack point: (token index after the star, text index it consumed up to).
        let mut star: Option<(usize, usize)> = None;
        loop {
            if p < self.tokens.len() {
                match &self.tokens[p] {
                    Token::Star => {
                        star = Some((p + 1, t));
                        p += 1;
                        continue;
                    }
                    token if t < text.len() && token_matches(token, text[t]) => {
                        t += 1;
                        p += 1;
                        continue;
                    }
                    _ => {}
                }
            } else if t == text.len() {
                return true;
            }
            match star {
                Some((resume, consumed)) if consumed < text.len() => {
                    star = Some((resume, consumed + 1));
                    p = resume;
                    t = consumed + 1;
                }
                _ => return false,
            }
        }
    }
}

fn token_matches(token: &Token, c: char) -> bool {
    match token {
        Token::Literal(l) => *l == c,
        Token::Any => true,
        Token::Set { negated, items } => {
            let hit = items.iter().any(|item| match item {
                SetItem::Char(x) => *x == c,
                SetItem::Range(lo, hi) => (*lo..=*hi).contains(&c),
            });
            hit != *negated
        }
        Token::Star | Token::Never => false,
    }
}

/// Mirror of the bracket handling in `fnmatch.translate`. `body` is the text
/// between `[` and `]`.
fn compile_set(body: &[char]) -> Token {
    let mut body = body;
    let mut negated = false;
    if body.first() == Some(&'!') {
        negated = true;
        body = &body[1..];
    }
    // Split on '-' operators, keeping a leading '-' (or one right after the
    // negation mark) inside the first chunk, as Python does.
    let mut chunks: Vec<Vec<char>> = Vec::new();
    let mut start = 0;
    let mut k = 1;
    while let Some(pos) = body
        .iter()
        .enumerate()
        .skip(k)
        .find_map(|(idx, c)| (*c == '-').then_some(idx))
    {
        chunks.push(body[start..pos].to_vec());
        start = pos + 1;
        k = pos + 3;
    }
    let tail = body[start.min(body.len())..].to_vec();
    if tail.is_empty() {
        if let Some(last) = chunks.last_mut() {
            last.push('-');
        } else {
            chunks.push(vec!['-']);
        }
    } else {
        chunks.push(tail);
    }
    // Drop empty (reversed) ranges.
    for k in (1..chunks.len()).rev() {
        let prev_last = chunks[k - 1].last().copied();
        let next_first = chunks[k].first().copied();
        if let (Some(a), Some(b)) = (prev_last, next_first)
            && a > b
        {
            let removed = chunks.remove(k);
            let prev = &mut chunks[k - 1];
            prev.pop();
            prev.extend_from_slice(&removed[1..]);
        }
    }
    let mut seq: Vec<Option<char>> = Vec::new(); // None = range operator
    for (index, chunk) in chunks.iter().enumerate() {
        if index > 0 {
            seq.push(None);
        }
        seq.extend(chunk.iter().copied().map(Some));
    }
    let items = scan_class(&seq);
    if items.is_empty() {
        return if negated { Token::Any } else { Token::Never };
    }
    Token::Set { negated, items }
}

/// Interpret the joined class like the regex engine: `a<op>b` is a range,
/// any other operator is a literal `-`.
fn scan_class(seq: &[Option<char>]) -> Vec<SetItem> {
    let mut items = Vec::new();
    let mut i = 0;
    while i < seq.len() {
        match seq[i] {
            None => {
                items.push(SetItem::Char('-'));
                i += 1;
            }
            Some(lo) => {
                if let (Some(None), Some(Some(hi))) = (seq.get(i + 1), seq.get(i + 2)) {
                    items.push(SetItem::Range(lo, *hi));
                    i += 3;
                } else {
                    items.push(SetItem::Char(lo));
                    i += 1;
                }
            }
        }
    }
    items
}

#[cfg(test)]
mod tests {
    use super::Glob;

    // Expected values were produced by Python 3.13.1 `fnmatch.fnmatchcase(name, pattern)`.
    #[test]
    fn matches_python_fnmatchcase() {
        let cases: &[(&str, &str, bool)] = &[
            ("a*", "abc", true),
            ("*.wav", "x.WAV", false),
            ("[a-c]?", "bz", true),
            ("[!a-c]?", "bz", false),
            ("[]a]", "]", true),
            ("[]a]", "a", true),
            ("[", "[", true),
            ("a[", "a[", true),
            ("[!]", "!", false),
            ("[a-]", "-", true),
            ("[c-a]", "b", false),
            ("[a-c-e]", "d", false),
            ("[a-c-e]", "-", true),
            ("[^a]", "^", true),
            ("[^a]", "a", true),
            ("[[]", "[", true),
            ("*", "\n", true),
            ("?", "\n", true),
            ("[a-b-c]", "-", true),
            ("a?*", "a", false),
            ("**a**", "bab", true),
            ("[a\\]x", "\\", false),
            ("[a\\]x", "x", false),
            ("é*", "é1", true),
            ("[!a]", "é", true),
            ("[--0]", ".", true),
            ("[]-a]", "^", true),
        ];
        for (pattern, name, expected) in cases {
            assert_eq!(
                Glob::new(pattern).is_match(name),
                *expected,
                "pattern {pattern:?} name {name:?}"
            );
        }
    }

    #[test]
    fn star_backtracks() {
        assert!(Glob::new("*a*b").is_match("aaab"));
        assert!(!Glob::new("*a*b").is_match("aaba"));
        assert!(Glob::new("").is_match(""));
        assert!(!Glob::new("").is_match("x"));
    }
}
