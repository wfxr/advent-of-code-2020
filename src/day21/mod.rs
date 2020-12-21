use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let mut m: HashMap<&str, HashSet<&str>> = HashMap::new(); // allergen -> possible ingredients
    input.iter().for_each(|(ingredients, allergens)| {
        allergens.iter().for_each(|allergen| {
            let entry = m.entry(allergen).or_insert_with(|| ingredients.clone());
            *entry = entry.intersection(ingredients).cloned().collect();
        });
    });

    let mut mapping = HashMap::new(); // ingredient -> allergen
    while let Some((&k, v)) = m.iter().filter(|(_, v)| v.len() == 1).next() {
        let allergen = *v.iter().next().unwrap();
        mapping.insert(allergen, k);
        m.remove(k);
        m.iter_mut().for_each(|(_, v)| {
            v.remove(allergen);
        });
    }
    assert!(m.is_empty(), "no easy solution!");
    input
        .iter()
        .flat_map(|(ingredients, _)| ingredients)
        .filter(|&ingredient| !mapping.contains_key(ingredient))
        .count()
}

fn part2(input: &str) -> usize {
    unimplemented!()
}

fn parse_input(input: &str) -> Vec<(HashSet<&str>, HashSet<&str>)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(|c: char| !c.is_ascii_alphabetic()).filter(|s| !s.is_empty());
            let ingredients = it.by_ref().take_while(|&s| s != "contains").collect();
            let allergens = it.collect();
            (ingredients, allergens)
        })
        .collect()
}
crate::solution!(part1 => 2098, part2 => 0);
