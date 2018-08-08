use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::path::Path;

use errors::*;

use {Day, Department, Meal, PlannedMeal, Product, Quantity, RQuantity, Recipe, Result};

#[derive(Debug, Deserialize)]
struct UnresolvedIngredient {
    ingredient: String,
    quantity: Quantity,
}

#[derive(Debug, Deserialize)]
struct UnresolvedRecipe {
    name: String,
    filename: String,
}

#[derive(Debug, Deserialize)]
struct UnresolvedPlannedMeal {
    day: Day,
    meal: Meal,
    recipe: String,
}

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

pub(super) fn load_recipes(
    products: &Vec<Product>,
    path: &str,
    recipes_dir: &str,
) -> Result<Vec<Recipe>> {
    let mut recipes: Vec<Recipe> = Vec::new();
    for result in reader(&path)?.deserialize() {
        let unresolved_recipe: UnresolvedRecipe = result.chain_err(|| "unable to parse recipe")?;
        let filepath = Path::new(&recipes_dir).join(unresolved_recipe.filename);
        let ingredients: HashMap<Product, RQuantity> =
            load_ingredients(&products, filepath).chain_err(|| "unable to resolve recipe")?;

        let recipe = Recipe {
            name: unresolved_recipe.name,
            ingredients: ingredients,
        };

        recipes.push(recipe);
    }
    Ok(recipes)
}

pub(super) fn load_ingredients<P: AsRef<Path> + fmt::Debug>(
    products: &Vec<Product>,
    path: P,
) -> Result<HashMap<Product, RQuantity>> {
    let mut ingredients: HashMap<Product, RQuantity> = HashMap::new();

    for result in reader(&path)?.deserialize() {
        let ingredient: UnresolvedIngredient = result.chain_err(|| "unable to parse ingredient")?;
        let product = products.iter().find(|p| p.name == ingredient.ingredient);
        match product {
            Some(product) => {
                if ingredients.contains_key(&product) {
                    bail!("encountered duplicate ingredient `{:?}`", product)
                }
                ingredients.insert(product.clone(), RQuantity::new(&ingredient.quantity.0));
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

        let recipe = recipes
            .iter()
            .find(|r| r.name == unresolved_planned_meal.recipe);

        if recipe.is_none() {
            bail!("unrecognized recipe \"{}\"", unresolved_planned_meal.recipe);
        }

        let planned_meal = PlannedMeal {
            day: unresolved_planned_meal.day,
            meal: unresolved_planned_meal.meal,
            recipe: recipe.unwrap().clone(),
        };

        planned_meals.push(planned_meal);
    }
    Ok(planned_meals)
}
