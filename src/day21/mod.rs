use std::collections::{HashMap, HashSet};

fn solve_mapping<'a>(input: &'a [(HashSet<&str>, HashSet<&str>)]) -> HashMap<&'a str, &'a str> {
    let mut m = HashMap::new(); // allergen -> possible ingredients
    input.iter().for_each(|(ingrs, alles)| {
        alles.iter().for_each(|alle| {
            let entry = m.entry(alle).or_insert_with(|| ingrs.clone());
            *entry = entry.intersection(ingrs).cloned().collect();
        });
    });

    let mut mapping = HashMap::new(); // ingredient -> allergen
    while let Some((&ingr, alles)) = m.iter().find(|(_, alles)| alles.len() == 1) {
        let alle = *alles.iter().next().unwrap();
        mapping.insert(alle, *ingr);
        m.remove(ingr);
        m.iter_mut().for_each(|(_, alles)| {
            alles.remove(alle);
        });
    }
    assert!(m.is_empty(), "does not work!");
    mapping
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let mapping = solve_mapping(&input);
    input
        .iter()
        .flat_map(|(ingredients, _)| ingredients)
        .filter(|&ingredient| !mapping.contains_key(ingredient))
        .count()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);
    let mut ingredients: Vec<_> = solve_mapping(&input).into_iter().collect();
    ingredients.sort_unstable_by_key(|&(_, v)| v);
    ingredients.iter().map(|&(k, _)| k).collect::<Vec<&str>>().join(",")
}

fn parse_input(input: &str) -> Vec<(HashSet<&str>, HashSet<&str>)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(|c: char| !c.is_ascii_alphabetic()).filter(|s| !s.is_empty());
            let ingrs = it.by_ref().take_while(|&s| s != "contains").collect();
            let alles = it.collect();
            (ingrs, alles)
        })
        .collect()
}
crate::solution!(part1 => 2098, part2 => "ppdplc,gkcplx,ktlh,msfmt,dqsbql,mvqkdj,ggsz,hbhsx");
