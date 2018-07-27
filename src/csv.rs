use std::fmt;
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;

use ::errors::*;

use ::{Result, Department, Product, Recipe, Quantity, PlannedMeal};
use ::{UnresolvedIngredient, UnresolvedRecipe, UnresolvedPlannedMeal};

fn reader<P: AsRef<Path> + fmt::Debug>(path: P) -> Result<::csv_crate::Reader<File>> {
    ::csv_crate::Reader::from_path(&path).chain_err(|| format!("unable to open file #{:?}", &path))
}

pub(super) fn load_departments(path: &str) -> Result<Vec<Department>> {
    let mut departments: Vec<Department> = Vec::new();
    for result in reader(&path)?.deserialize() {
        let department: Department = result.chain_err(|| "unable to parse department")?;
        departments.push(department);
    }
    Ok(departments)
}

pub(super) fn load_products(departments: &Vec<Department>, path: &str) -> Result<Vec<Product>> {
    let mut products: Vec<Product> = Vec::new();
    for result in reader(&path)?.deserialize() {
        let product: Product = result.chain_err(|| "unable to parse product")?;

        if departments.contains(&product.department) {
            products.push(product);
        } else {
            bail!("unrecognized department \"{}\"", &product.department);
        }
    }
    Ok(products)
}

pub(super) fn load_recipes(products: &Vec<Product>, path: &str) -> Result<Vec<Recipe>> {
    let mut recipes: Vec<Recipe> = Vec::new();
    for result in reader(&path)?.deserialize() {
        let unresolved_recipe: UnresolvedRecipe = result.chain_err(|| "unable to parse recipe")?;
        let recipe = unresolved_recipe
            .resolve(products)
            .chain_err(|| "unable to resolve recipe")?;

        recipes.push(recipe);
    }
    Ok(recipes)
}


pub(super) fn load_ingredients<P: AsRef<Path> + fmt::Debug>(
    products: &Vec<Product>,
    path: P,
) -> Result<HashMap<Product, Quantity>> {
    let mut ingredients: HashMap<Product, Quantity> = HashMap::new();

    for result in reader(&path)?.deserialize() {
        let ingredient: UnresolvedIngredient = result.chain_err(|| "unable to parse ingredient")?;
        let product = products.iter().find(|p| p.name == ingredient.ingredient);
        match product {
            Some(product) => {
                if ingredients.contains_key(&product) {
                    bail!("encountered duplicate ingredient `{:?}`", product)
                }
                ingredients.insert(product.clone(), ingredient.quantity);
            }
            None => bail!("unrecognized ingredient \"{}\"", ingredient.ingredient),
        }
    }

    Ok(ingredients)
}

pub(super) fn load_planned_meals(recipes: &Vec<Recipe>, path: &str) -> Result<Vec<PlannedMeal>> {
    let mut planned_meals: Vec<PlannedMeal> = Vec::new();
    for result in reader(&path)?.deserialize() {
        let unresolved_planned_meal: UnresolvedPlannedMeal =
            result.chain_err(|| "unable to parse plan")?;
        let planned_meal: PlannedMeal = unresolved_planned_meal
            .resolve(recipes)
            .chain_err(|| "unable to resolve plan")?;

        planned_meals.push(planned_meal);
    }
    Ok(planned_meals)
}
