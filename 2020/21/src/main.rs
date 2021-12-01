use std::collections::{HashMap, HashSet};

fn main() {
    let contents = include_str!("../input");
    let mut recipes = contents.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(read_recipe)
        .collect::<Option<Vec<(Vec<&str>, Vec<&str>)>>>()
        .expect("Error reading recipes");

    let mut ingredients: Vec<(&str, usize)> = Vec::new();
    let mut allergens: HashMap<&str, Vec<usize>> = HashMap::new();
    let mut not_allergen: HashMap<&str, Vec<&str>> = HashMap::new();

    for (i, recipe) in recipes.iter().enumerate() {
        for ingredient in recipe.0.iter() {
            match ingredients.iter().position(|(x,_)| x.eq(ingredient)) {
                Some(index) => ingredients[index].1 += 1,
                None        => ingredients.push((ingredient, 1)),
            }
        }
        for allergen in recipe.1.iter() {
            if allergens.contains_key(allergen) {
                allergens.entry(allergen).and_modify(|x| x.push(i));
            } else {
                allergens.insert(allergen, vec![i]);
            }
        }
    }

    for (allergen, is) in allergens.iter() {
        let mut possible = recipes[is[0]].0.clone();
        for &i in &is[1..] {
            possible.retain(|ingredient| recipes[i].0.contains(ingredient));
        }
        not_allergen.insert(allergen, ingredients.iter()
            .map(|&(ingredient,_)| ingredient)
            .filter(|ingredient| !possible.contains(ingredient))
            .collect());
    }

    let mut safe = Vec::new();
    let mut n = 0;
    for (ingredient, uses) in ingredients {
        if not_allergen.iter().all(|(_, is)| is.contains(&ingredient)) {
            safe.push(ingredient);
            n += uses;
        }
    }

    println!("Part 1: {}", n);

    for (recipe_ingredients, _) in recipes.iter_mut() {
        recipe_ingredients.retain(|ingredient| !safe.contains(ingredient));
    }

    let mut identified: HashMap<&str, &str> = HashMap::new();
    let mut allergenic: HashSet<&str> = HashSet::new();

    loop {
        let mut changed = false;
        for (&allergen, is) in allergens.iter() {
            if identified.contains_key(allergen) {
                continue;
            }
            let mut possible = recipes[is[0]].0.iter()
                .copied()
                .filter(|&i| !allergenic.contains(i))
                .collect::<Vec<&str>>();
            for &i in &is[1..] {
                possible.retain(|ingredient| recipes[i].0.contains(ingredient));
            }
            if possible.len() == 1 {
                changed = true;
                let ingredient = possible[0];
                allergenic.insert(ingredient);
                identified.insert(allergen, ingredient);
            }
        }
        if !changed {
            break;
        }
    }

    let mut allergens = identified.keys().copied().collect::<Vec<&str>>();
    allergens.sort();

    let part2 = allergens.iter()
        .map(|allergen| identified[allergen])
        .collect::<Vec<&str>>()
        .join(",");
    println!("Part 2: {}", part2);
}

fn read_recipe(l: &str) -> Option<(Vec<&str>, Vec<&str>)> {
    let parts = l.splitn(2, "(contains").collect::<Vec<&str>>();
    if parts.len() != 2 {
        return None;
    }
    let ingredients = parts[0].trim()
        .split_whitespace()
        .collect::<Vec<&str>>();
    let allergens = parts[1].strip_suffix(")")?
        .trim()
        .split(",")
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    Some((ingredients, allergens))
}
