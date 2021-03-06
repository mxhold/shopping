extern crate csv as csv_crate;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate num_rational;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fmt;

mod errors {
    error_chain!{}
}

use errors::*;

mod csv;
mod quantity;

use quantity::RQuantity;

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

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
struct Recipe {
    name: String,
    ingredients: HashMap<Product, RQuantity>,
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

fn sum_ingredients(planned_meals: &Vec<PlannedMeal>) -> HashMap<Product, RQuantity> {
    let mut planned_ingredients: HashMap<Product, RQuantity> = HashMap::new();
    for planned_meal in planned_meals {
        for (product, quantity) in &planned_meal.recipe.ingredients {
            planned_ingredients
                .entry(product.clone())
                .and_modify(|q| *q = *q + quantity)
                .or_insert(*quantity);
        }
    }
    planned_ingredients
}

fn subtract_ingredients(
    subtrahend_ingredients: &HashMap<Product, RQuantity>,
    minuend_ingredients: &HashMap<Product, RQuantity>,
) -> HashMap<Product, RQuantity> {
    let mut difference: HashMap<Product, RQuantity> = HashMap::new();
    for (product, subtrahend_quantity) in subtrahend_ingredients.iter() {
        let quantity_difference = match minuend_ingredients.get(product) {
            Some(minuend_quantity) => *subtrahend_quantity - minuend_quantity,
            None => *subtrahend_quantity,
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
    let inventory: HashMap<Product, RQuantity> =
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

    println!("=========================================");

    let mut ingredients_to_buy_by_department: HashMap<Department, Vec<(Product, RQuantity)>> =
        HashMap::new();
    for (product, quantity) in ingredients_to_buy {
        let entry = ingredients_to_buy_by_department.entry(product.department.clone());
        entry.or_insert(Vec::new()).push((product, quantity));
    }

    let mut sorted_departments: Vec<Department> =
        ingredients_to_buy_by_department.keys().cloned().collect();
    sorted_departments
        .sort_unstable_by_key(|sd| departments.iter().position(|ref d| d.0 == sd.0).unwrap());

    for department in sorted_departments {
        println!("## {}", department);
        let ingredients: &Vec<(Product, RQuantity)> =
            ingredients_to_buy_by_department.get(&department).unwrap();
        for (product, quantity) in ingredients {
            if quantity.is_positive() {
                println!("{} {}", quantity, product);
            }
        }
    }

    //    for (department, ingredients) in ingredients_to_buy_by_department {
    //        println!("## {}", department);
    //        for (product, quantity) in ingredients {
    //            if quantity.is_positive() {
    //                println!("{} {}", quantity, product);
    //            }
    //        }
    //    }

    Ok(())
}
