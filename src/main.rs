use std::io::{self, Read};

fn main() {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .expect("failed to read input");

    let text = text.trim_end_matches(|character| character == '\n' || character == '\r');
    println!("{}", longest_palindromic_substring(text));
}

fn longest_palindromic_substring(text: &str) -> &str {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    if n == 0 {
        return text;
    }

    let mut d1 = vec![0usize; n];
    let mut left = 0;
    let mut right = 0;
    for i in 0..n {
        let mut k = if i < right {
            d1[left + right - 1 - i].min(right - i)
        } else {
            1
        };
        while i >= k && i + k < n && chars[i - k] == chars[i + k] {
            k += 1;
        }
        d1[i] = k;
        if i + k > right {
            left = i + 1 - k;
            right = i + k;
        }
    }

    let mut d2 = vec![0usize; n];
    left = 0;
    right = 0;
    for i in 0..n {
        let mut k = if i < right {
            d2[left + right - i].min(right - i)
        } else {
            0
        };
        while i >= k + 1 && i + k < n && chars[i - k - 1] == chars[i + k] {
            k += 1;
        }
        d2[i] = k;
        if i + k > right {
            left = i - k;
            right = i + k;
        }
    }

    let mut best_start = 0usize;
    let mut best_len = 0usize;

    for (i, &radius) in d1.iter().enumerate() {
        let len = 2 * radius - 1;
        if len > best_len {
            best_len = len;
            best_start = i + 1 - radius;
        }
    }
    for (i, &radius) in d2.iter().enumerate() {
        let len = 2 * radius;
        if len > best_len {
            best_len = len;
            best_start = i - radius;
        }
    }

    let start_byte = char_to_byte_index(text, best_start);
    let end_byte = char_to_byte_index(text, best_start + best_len);
    &text[start_byte..end_byte]
}

fn char_to_byte_index(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map(|(byte_index, _)| byte_index)
        .unwrap_or(text.len())
}

#[cfg(test)]
#[path = "../tests/unit/palindrome.rs"]
mod palindrome_tests;
