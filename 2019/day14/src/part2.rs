use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::BufRead,
};

fn required_ores(recipes: &HashMap<String, (u64, Vec<(String, u64)>)>, fuel: u64) -> u64 {
    let mut need = BTreeMap::new();
    need.insert(String::from("FUEL"), fuel);
    let mut leftovers = HashMap::new();
    let mut ores = 0;

    while let Some((chem, mut qnt)) = need.pop_first() {
        if chem == "ORE" {
            ores += qnt;
            continue;
        }

        if let Some(leftover) = leftovers.get_mut(&chem) {
            let used = if *leftover > qnt { qnt } else { *leftover };
            *leftover -= used;
            qnt -= used
        }

        if qnt == 0 {
            continue;
        }

        let (recipe_qnt, ingredients) = recipes.get(&chem).unwrap();
        let mul = (qnt as u64).div_ceil(*recipe_qnt);
        *leftovers.entry(chem.clone()).or_insert(0) += mul * recipe_qnt;
        for (ingredient, ingredient_qnt) in ingredients {
            *need.entry(ingredient.clone()).or_insert(0) += mul * (*ingredient_qnt);
        }

        need.insert(chem, qnt);
    }
    ores
}

fn binary_search(max_ores: u64, recipes: &HashMap<String, (u64, Vec<(String, u64)>)>) -> u64 {
    let mut lower = 0;
    let mut upper = max_ores;
    while lower != upper {
        let fuel = lower + ((upper - lower) as u64).div_ceil(2);
        let ores = required_ores(recipes, fuel);
        if ores > max_ores {
            upper = fuel - 1;
        } else {
            lower = fuel;
        }
    }
    lower
}

pub fn run<B: BufRead>(mut buf: B) {
    let mut recipes: HashMap<String, (u64, Vec<(String, u64)>)> = HashMap::new();
    let mut line = String::new();
    while let Ok(len) = buf.read_line(&mut line) {
        if len == 0 {
            break;
        }

        let mut eq = line.trim().split("=>");
        let ingredients = eq.next().unwrap();
        let mut ingredients = ingredients.split(",");
        let ingredients: Vec<(String, u64)> = ingredients
            .map(|ingredient| {
                let ingredient = ingredient.trim();
                let mut iter = ingredient.split(" ");
                let qnt = iter.next().unwrap().parse().unwrap();
                let chem = iter.next().unwrap().into();
                (chem, qnt)
            })
            .collect();

        let result = eq.next().unwrap().trim();
        let mut iter = result.split(" ");
        let qnt = iter.next().unwrap().parse().unwrap();
        let chem = iter.next().unwrap().into();
        recipes.insert(chem, (qnt, ingredients));

        line.truncate(0);
    }

    let fuel = binary_search(1_000_000_000_000, &recipes);

    println!("{fuel}");
}
