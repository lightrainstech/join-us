/// "The real stuff" Q2 — takes a number and returns its digits in an array.
pub fn digits(n: i64) -> Vec<u32> {
    let s = n.abs().to_string();
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

/// "The real stuff" Q3 — remove duplicates, keeping first occurrences in order.
pub fn unique<T: PartialEq + Clone>(items: &[T]) -> Vec<T> {
    let mut out: Vec<T> = Vec::with_capacity(items.len());
    for item in items {
        if !out.contains(item) {
            out.push(item.clone());
        }
    }
    out
}

/// "The real stuff" Q5 — rotate a list left by `k` elements, in place.
/// Uses the three-reversal trick: no copy of the list is created and no
/// extra memory is needed, only `n` swap operations in total.
/// [1,2,3,4,5,6] rotated by 2 becomes [3,4,5,6,1,2].
pub fn rotate<T>(items: &mut [T], k: usize) {
    let len = items.len();
    if len == 0 {
        return;
    }
    let k = k % len; // rotating by len (or a multiple) changes nothing
    if k == 0 {
        return;
    }
    // Reverse the whole slice, then the two halves around the split point.
    reverse(items, 0, len - 1);
    reverse(items, 0, len - k - 1);
    reverse(items, len - k, len - 1);
}

fn reverse<T>(items: &mut [T], start: usize, end: usize) {
    let mut i = start;
    let mut j = end;
    while i < j {
        items.swap(i, j);
        i += 1;
        j -= 1;
    }
}

/// "The real stuff" Q4 — translate a text to Pig Latin.
/// Rule: move the first letter of every word to the end and add "ay",
/// e.g. "The quick brown fox" -> "Hetay uickqay rownbay oxfay".
/// Punctuation attached to a word is kept in place.
pub fn to_pig_latin(text: &str) -> String {
    translate(text, true)
}

/// "The real stuff" Q4 (second half) — translate Pig Latin back to English.
pub fn from_pig_latin(text: &str) -> String {
    translate(text, false)
}

fn translate(text: &str, to_pig: bool) -> String {
    text.split(' ')
        .map(|word| translate_word(word, to_pig))
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_word(word: &str, to_pig: bool) -> String {
    // Preserve leading/trailing punctuation like the full stop in "fox."
    let trimmed = word.trim_matches(|c: char| !c.is_ascii_alphabetic());
    let (prefix, core, suffix) = split_around(word, trimmed);
    if core.is_empty() {
        return word.to_string();
    }
    if to_pig {
        // Keep the original case pattern: "The" -> "Hetay".
        let mut chars = core.chars();
        let first = chars.next().unwrap();
        let rest: String = chars.collect();
        let cased = match (first.is_uppercase(), rest.chars().next()) {
            (true, Some(_)) => {
                // Capitalize the new first letter, lowercase the moved one.
                let recased = rest
                    .chars()
                    .enumerate()
                    .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                    .collect::<String>();
                format!("{}{}", recased, first.to_ascii_lowercase())
            }
            (true, None) => first.to_string(), // single letter like "I" keeps its case
            _ => format!("{}{}", rest, first.to_ascii_lowercase()),
        };
        format!("{}{}ay{}", prefix, cased, suffix)
    } else {
        // Strip "ay" and move the last letter back to the front: "Hetay" -> "The".
        let stem = core.strip_suffix("ay").unwrap_or(core);
        let count = stem.chars().count();
        if count == 0 {
            return word.to_string();
        }
        if count == 1 {
            // Words like "Iay" were single letters to begin with.
            return format!("{}{}{}", prefix, stem, suffix);
        }
        let moved = stem.chars().last().unwrap();
        let head: String = stem.chars().take(count - 1).collect();
        let cased = if core.chars().next().unwrap().is_uppercase() {
            // The moved letter was the original capitalized first letter.
            let lowered_head = head
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_lowercase() } else { c })
                .collect::<String>();
            format!("{}{}", moved.to_ascii_uppercase(), lowered_head)
        } else {
            format!("{}{}", moved, head)
        };
        format!("{}{}{}", prefix, cased, suffix)
    }
}

fn split_around<'a>(word: &'a str, core: &'a str) -> (&'a str, &'a str, &'a str) {
    match word.find(core) {
        Some(i) => (&word[..i], core, &word[i + core.len()..]),
        None => ("", word, ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_of_positive_number() {
        assert_eq!(digits(12345), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn digits_of_negative_number() {
        assert_eq!(digits(-402), vec![4, 0, 2]);
    }

    #[test]
    fn digits_of_single_digit() {
        assert_eq!(digits(7), vec![7]);
    }

    #[test]
    fn digits_of_zero() {
        assert_eq!(digits(0), vec![0]);
    }

    #[test]
    fn unique_removes_duplicates_and_keeps_order() {
        assert_eq!(unique(&[1, 2, 2, 3, 1, 4]), vec![1, 2, 3, 4]);
        assert_eq!(unique(&["a", "b", "a"]), vec!["a", "b"]);
        assert_eq!(unique::<i32>(&[]), Vec::<i32>::new());
    }

    #[test]
    fn pig_latin_example_from_the_question() {
        assert_eq!(
            to_pig_latin("The quick brown fox"),
            "Hetay uickqay rownbay oxfay"
        );
    }

    #[test]
    fn pig_latin_round_trip() {
        let english = "The quick brown fox jumps over the lazy dog";
        assert_eq!(from_pig_latin(&to_pig_latin(english)), english);
    }

    #[test]
    fn pig_latin_handles_punctuation_and_single_letters() {
        assert_eq!(to_pig_latin("I am here."), "Iay maay erehay.");
        assert_eq!(from_pig_latin("Iay maay erehay."), "I am here.");
    }

    #[test]
    fn rotate_works_with_the_example() {
        let mut list = vec![1, 2, 3, 4, 5, 6];
        rotate(&mut list, 2);
        assert_eq!(list, vec![3, 4, 5, 6, 1, 2]);
    }

    #[test]
    fn rotate_by_k_is_equivalent_to_k_mod_len() {
        let mut list = vec![1, 2, 3, 4, 5, 6];
        rotate(&mut list, 8); // 8 mod 6 == 2
        assert_eq!(list, vec![3, 4, 5, 6, 1, 2]);
    }

    #[test]
    fn rotate_handles_edge_cases() {
        let mut a = vec![1, 2, 3];
        rotate(&mut a, 0);
        assert_eq!(a, vec![1, 2, 3]);

        let mut b = vec![1, 2, 3];
        rotate(&mut b, 3); // full cycle
        assert_eq!(b, vec![1, 2, 3]);

        let mut empty: Vec<i32> = vec![];
        rotate(&mut empty, 4);
        assert_eq!(empty, Vec::<i32>::new());
    }
}
