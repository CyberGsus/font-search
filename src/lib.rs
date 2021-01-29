#![feature(generators, generator_trait)]
use levenshtein::levenshtein;
use itertools::Itertools;

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

pub struct IterGenerator<G>
where
    G: Generator<Return = ()> + Unpin,
{
    generator: G,
}

impl<G> From<G> for IterGenerator<G>
where
    G: Generator<Return = ()> + Unpin,
{
    fn from(gen: G) -> Self {
        Self { generator: gen }
    }
}

impl<G> Iterator for IterGenerator<G>
where
    G: Generator<Return = ()> + Unpin,
{
    type Item = G::Yield;
    fn next(&mut self) -> Option<Self::Item> {
        match Pin::new(&mut self.generator).resume(()) {
            GeneratorState::Complete(_) => None,
            GeneratorState::Yielded(i) => Some(i),
        }
    }
}

pub fn unword(string: &str) -> IterGenerator<impl Generator<Return = (), Yield = &str> + '_> {
    IterGenerator::from(move || {
        let mut start = 0;
        let mut offset = 0;
        let mut is_alpha = true;
        let mut is_upper = false;
        let mut changed_to_alpha = false;
        const DELIMETERS: &str = " \t\n\r~!@#$%^&*()_+=-{{[}}]\\;',./<>?:\"";
        for (i, c) in string.char_indices() {
            if changed_to_alpha {
                is_upper = c.is_uppercase();
            }
            if DELIMETERS.contains(c) {
                if offset > start {
                    yield &string[start..offset];
                }
                offset = i + 1;
                start = offset;
                continue;
            }
            let is_a = c.is_alphabetic();
            if is_a ^ is_alpha {
                // if changed from alpha to non-alpha
                // then it's a word boundary
                is_alpha = is_a;
                if offset > start {
                    yield &string[start..offset];
                }
                start = offset;
                changed_to_alpha = is_a;
            } else if is_a {
                let is_u = c.is_uppercase();
                if !is_upper && is_u {
                    // if changed from lowercase to uppercase
                    // which applies only for alphabetic characters
                    if offset > start {
                        yield &string[start..offset];
                    }
                    start = offset;
                }
                is_upper = is_u;
            }
            offset = i + 1;
        }
        if offset > start {
            yield &string[start..offset];
        }
    })
}

pub fn find_best_match<'a>(
    filter: &str,
    it: &mut dyn Iterator<Item = &'a &'a str>,
) -> (&'a str, f32) {
    let sorter = sort_search(filter);
    it.fold((&"", 0f32), |acc, next| {
        let (_, best_value) = acc;
        let next_value = sorter(&mut unword(next));
        if best_value < next_value {
            (next, next_value)
        } else {
            acc
        }
    })
}

pub fn sort_search(string: &str) -> impl Fn(&mut dyn Iterator<Item = &str>) -> f32 + '_ {
    println!("search: {}", string);
    let search_terms = unword(&string)
        .map(|x| x.to_lowercase())
        .collect::<Vec<_>>();

    move |x| {
        x.cartesian_product(search_terms.iter().enumerate())
            .map(|(ref a, (i, ref b))| {
                let i = i as f32 + 1.0;
                let result = ratio(a, b);
                result / i
            })
            .sum()
    }
}
fn ratio(a: &str, b: &str) -> f32 {
    let distance = levenshtein(a, b) as f32;
    let len_sum = a.len() as f32 + b.len() as f32;
    (len_sum - distance) / len_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unword {
        use super::*;

        fn test_unword(test_case: &str, values: Vec<&str>) {
            assert_eq!(unword(test_case).collect::<Vec<_>>(), values);
        }

        #[test]
        fn delimeter_boundaries() {
            test_unword(
                "hi. i am gsus. lolool wow ? yes. i am gsus.",
                vec![
                    "hi", "i", "am", "gsus", "lolool", "wow", "yes", "i", "am", "gsus",
                ],
            )
        }

        #[test]
        fn number_boundaries() {
            test_unword("/hello1234world", vec!["hello", "1234", "world"])
        }

        #[test]
        fn case_boundaries() {
            test_unword(
                "HELLO, World! My name is Gsus",
                vec!["HELLO", "World", "My", "name", "is", "Gsus"],
            );
        }

        #[test]
        fn case_number_boundaries() {
            test_unword("12AM", vec!["12", "AM"])
        }
    }

    #[test]
    fn sort() {
        let words = vec!["Fira Code", "Caskaydia Mono Nerd Font", "Sauce Code Pro", ];
        let mut it = words.iter();
        assert_eq!(find_best_match("caskaydia", &mut it).0, words[1]);
    }
}
