extern crate csv as csv_crate;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

use std::collections::HashMap;
use std::fmt;

mod errors {
    error_chain!{}
}

use errors::*;

mod csv;
mod quantity;

use quantity::Quantity;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
struct Department(String);

impl fmt::Display for Department {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
struct Product {
    name: String,
    department: Department,
}

#[derive(Debug, Clone)]
struct Recipe {
    name: String,
    ingredients: HashMap<Product, Quantity>,
}

#[derive(Debug, Deserialize)]
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Deserialize)]
enum Meal {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

#[derive(Debug)]
struct PlannedMeal {
    day: Day,
    meal: Meal,
    recipe: Recipe,
}

fn sum_ingredients(planned_meals: &Vec<PlannedMeal>) -> HashMap<Product, Quantity> {
    let mut planned_ingredients: HashMap<Product, Quantity> = HashMap::new();
    for planned_meal in planned_meals {
        for (product, quantity) in &planned_meal.recipe.ingredients {
            planned_ingredients
                .entry(product.clone())
                .and_modify(|q| *q = q.clone() + quantity)
                .or_insert(quantity.clone());
        }
    }
    planned_ingredients
}

fn subtract_ingredients(
    // these argument names are maybe a little too cute...
    subtrahend_ingredients: &HashMap<Product, Quantity>,
    minuend_ingredients: &HashMap<Product, Quantity>,
) -> HashMap<Product, Quantity> {
    let mut difference: HashMap<Product, Quantity> = HashMap::new();
    for (product, subtrahend_quantity) in subtrahend_ingredients.iter() {
        let quantity_difference = match minuend_ingredients.get(product) {
            Some(minuend_quantity) => subtrahend_quantity.clone() - minuend_quantity,
            None => subtrahend_quantity.clone(),
        };
        difference.insert(product.clone(), quantity_difference);
    }
    difference
}

pub fn run() -> Result<()> {
    let departments: Vec<Department> = csv::load_departments("inputs/departments.csv")?;
    let products: Vec<Product> = csv::load_products(&departments, "inputs/products.csv")?;
    let recipes: Vec<Recipe> =
        csv::load_recipes(&products, "inputs/recipes.csv", "inputs/recipes")?;
    let inventory: HashMap<Product, Quantity> =
        csv::load_ingredients(&products, "inputs/inventory.csv")?;
    let planned_meals: Vec<PlannedMeal> = csv::load_planned_meals(&recipes, "inputs/plan.csv")?;

    let planned_ingredients = sum_ingredients(&planned_meals);

    let ingredients_to_buy = subtract_ingredients(&planned_ingredients, &inventory);

    println!("products: {:?}", products);
    println!("departments: {:?}", departments);
    println!("recipes: {:?}", recipes);
    println!("inventory: {:?}", inventory);
    println!("planned_meals: {:?}", planned_meals);
    println!("planned_ingredients: {:?}", planned_ingredients);
    println!("ingredients_to_buy: {:?}", ingredients_to_buy);

    Ok(())
}
