use font_search::find_best_match;
use fontconfig::{list_fonts, Fontconfig, Pattern};
use std::collections::HashMap;

fn process_path(path: &str) -> &str {
    let len = path.len();
    let start = path.rfind('/').unwrap_or(len);
    let offset = std::cmp::max(start, path.rfind('.').unwrap_or(len));
    &path[start..offset]
}

fn main() {
    // FIXME: faster collecting/mapping of data
    // so I don't have to copy everything around
    let fc = Fontconfig::new().unwrap();
    let all_fonts = list_fonts(&Pattern::new(&fc), None);
    let all_fonts = all_fonts.iter().collect::<Vec<_>>();
    let all_fonts = all_fonts
        .iter()
        .filter_map(|p| {
            let identifying_str = if let Some(name) = p.name() {
                name
            } else if let Some(path) = p.filename() {
                process_path(path)
            } else {
                return None;
            };
            Some((identifying_str, p))
        })
        .collect::<HashMap<_, _>>();

    let filter = "caskaydia mono";

    println!("searching for: {:?}", filter);

    let (best_match, score) = find_best_match(filter, &mut all_fonts.keys());
    let best_match = all_fonts.get(best_match);

    if let Some(best_match) = best_match {
        print!("best match: ");
        if let Some(name) = best_match.name() {
            print!("{} ", name);
        }
        if let Some(filename) = best_match.filename() {
            print!("at {}", filename);
        }
        println!(" with a score of {}", score);
    }
}
