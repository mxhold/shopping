extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::fmt;
use std::collections::HashMap;
use std::path::Path;

mod errors{
    error_chain! { }
}

use errors::*;

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

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct Quantity(String);

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

#[derive(Debug, Deserialize)]
struct UnresolvedPlannedMeal {
    day: Day,
    meal: Meal,
    recipe: String,
}

#[derive(Debug)]
struct PlannedMeal {
    day: Day,
    meal: Meal,
    recipe: Recipe,
}

fn load_departments(path: &str) -> Result<Vec<Department>>  {
    let file = File::open(&path).chain_err(|| "unable to open departments file")?;
    let mut reader = csv::Reader::from_reader(file);
    let mut departments: Vec<Department> = Vec::new();
    for result in reader.deserialize() {
        let department: Department = result.chain_err(|| "unable to parse department")?;
        departments.push(department);
    }
    Ok(departments)
}

fn load_products(departments: &Vec<Department>, path: &str) -> Result<Vec<Product>> {
    let mut products: Vec<Product> = Vec::new();
    for result in csv_reader(&path)?.deserialize() {
        let product: Product = result.chain_err(|| "unable to parse product")?;

        if departments.contains(&product.department) {
            products.push(product);
        } else {
            bail!("unrecognized department \"{}\"", &product.department);
        }
    }
    Ok(products)
}

fn load_recipes(products: &Vec<Product>, path: &str) -> Result<Vec<Recipe>> {
    let mut recipes: Vec<Recipe> = Vec::new();
    for result in csv_reader(&path)?.deserialize() {
        let unresolved_recipe: UnresolvedRecipe = result.chain_err(|| "unable to parse recipe")?;
        let recipe = unresolved_recipe.resolve(products).chain_err(|| "unable to resolve recipe")?;

        recipes.push(recipe);
    }
    Ok(recipes)
}

impl UnresolvedRecipe {
    fn resolve(self, products: &Vec<Product>) -> Result<Recipe> {
        let filepath = Path::new("inputs/recipes").join(self.filename);
        let ingredients: HashMap<Product, Quantity> = load_ingredients(&products, filepath)?;

        Ok(Recipe {
            name: self.name,
            ingredients,
        })
    }
}

fn load_ingredients<P: AsRef<Path> + fmt::Debug>(products: &Vec<Product>, path: P) -> Result<HashMap<Product, Quantity>> {
    let mut ingredients: HashMap<Product, Quantity> = HashMap::new();

    for result in csv_reader(&path)?.deserialize() {
        let ingredient: UnresolvedIngredient = result.chain_err(|| "unable to parse ingredient")?;
        let product = products.iter().find(|p| p.name == ingredient.ingredient);
        match product {
            Some(product) => {
                ingredients.insert(product.clone(), ingredient.quantity);
            }
            None => {
                bail!("unrecognized ingredient \"{}\"", ingredient.ingredient)
            }
        }
    }

    Ok(ingredients)
}

fn csv_reader<P: AsRef<Path> + fmt::Debug>(path: P) -> Result<csv::Reader<File>> {
    csv::Reader::from_path(&path).chain_err(|| format!("unable to open file #{:?}", &path))
}

impl UnresolvedPlannedMeal {
    fn resolve(self, recipes: &Vec<Recipe>) -> Result<PlannedMeal> {

        let recipe = recipes.iter().find(|r| r.name == self.recipe);

        if recipe.is_none() {
            bail!("unrecognized recipe \"{}\"", self.recipe);
        }

        Ok(PlannedMeal {
            day: self.day,
            meal: self.meal,
            recipe: recipe.unwrap().clone(),
        })
    }
}

fn load_planned_meals(recipes: &Vec<Recipe>, path: &str) -> Result<Vec<PlannedMeal>> {
    let mut planned_meals: Vec<PlannedMeal> = Vec::new();
    for result in csv_reader(&path)?.deserialize() {
        let unresolved_planned_meal: UnresolvedPlannedMeal = result.chain_err(|| "unable to parse plan")?;
        let planned_meal: PlannedMeal = unresolved_planned_meal.resolve(recipes).chain_err(|| "unable to resolve plan")?;

        planned_meals.push(planned_meal);
    }
    Ok(planned_meals)
}

fn run() -> Result<()> {
    let departments: Vec<Department> = load_departments("inputs/departments.csv")?;
    let products: Vec<Product> = load_products(&departments, "inputs/products.csv")?;
    let recipes: Vec<Recipe> = load_recipes(&products, "inputs/recipes.csv")?;
    let inventory: HashMap<Product, Quantity> = load_ingredients(&products, "inputs/inventory.csv")?;
    let planned_meals: Vec<PlannedMeal> = load_planned_meals(&recipes, "inputs/plan.csv")?;

    println!("products: {:?}", products);
    println!("departments: {:?}", departments);
    println!("recipes: {:?}", recipes);
    println!("inventory: {:?}", inventory);
    println!("planned_meals: {:?}", planned_meals);
    
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // Print backtrace, but only files/lines in this project
        if let Some(backtrace) = e.backtrace() {
            let frames = backtrace.frames();
            for frame in frames.iter() {
                for symbol in frame.symbols().iter() {
                    if let (Some(file), Some(lineno)) = (symbol.filename(), symbol.lineno()) {
                        if file.display().to_string()[0..3] == "src".to_string() {
                            println!("{}:{}", file.display().to_string(), lineno);
                        }
                    }
                }
            }
        }

        ::std::process::exit(1);
    }
}
