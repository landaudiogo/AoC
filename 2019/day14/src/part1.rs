use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::BufRead,
};

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

    let mut need = BTreeMap::new();
    need.insert(String::from("FUEL"), 1);
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

    println!("{ores}");
}
