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

#[derive(Debug, Deserialize, PartialEq)]
struct Quantity(String);

#[derive(Debug, Deserialize)]
struct UnresolvedRecipe {
    name: String,
    filename: String,
}

#[derive(Debug, Deserialize)]
struct UnresolvedIngredient {
    ingredient: String,
    quantity: Quantity,
}

#[derive(Debug)]
struct Recipe<> {
    name: String,
    ingredients: HashMap<Product, Quantity>,
}

impl UnresolvedRecipe {
    fn resolve(self, products: &Vec<Product>) -> Result<Recipe> {
        let mut ingredients: HashMap<Product, Quantity> = HashMap::new();
        let filepath = Path::new("inputs/recipes").join(self.filename);
        let file = File::open(filepath).chain_err(|| "unable to open recipe")?;
        let mut reader = csv::Reader::from_reader(file);

        for result in reader.deserialize() {
            let ingredient: UnresolvedIngredient = result.chain_err(|| "unable to parse recipe ingredient")?;
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

        Ok(Recipe {
            name: self.name,
            ingredients: ingredients,
        })
    }
}


fn load_departments(path: &str) -> Result<Vec<Department>>  {
    let file = File::open(path).chain_err(|| "unable to open departments file")?;
    let mut reader = csv::Reader::from_reader(file);
    let mut departments: Vec<Department> = Vec::new();
    for result in reader.deserialize() {
        let department: Department = result.chain_err(|| "unable to parse department")?;
        departments.push(department);
    }
    Ok(departments)
}

fn load_products(departments: &Vec<Department>, path: &str) -> Result<Vec<Product>> {
    let file = File::open(path).chain_err(|| "unable to open products file")?;
    let mut reader = csv::Reader::from_reader(file);
    let mut products: Vec<Product> = Vec::new();
    for result in reader.deserialize() {
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
    let file = File::open(path).chain_err(|| "unable to open recipes file")?;
    let mut reader = csv::Reader::from_reader(file);
    let mut recipes: Vec<Recipe> = Vec::new();
    for result in reader.deserialize() {
        let unresolved_recipe: UnresolvedRecipe = result.chain_err(|| "unable to parse recipe")?;
        let recipe = unresolved_recipe.resolve(products).chain_err(|| "unable to resolve recipe")?;

        recipes.push(recipe);
    }
    Ok(recipes)
}

fn run() -> Result<()> {
    let departments: Vec<Department> = load_departments("inputs/departments.csv")?;
    let products: Vec<Product> = load_products(&departments, "inputs/products.csv")?;

    let recipes: Vec<Recipe> = load_recipes(&products, "inputs/recipes.csv")?;

    println!("{:?}", products);
    println!("{:?}", departments);
    println!("{:?}", recipes);



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
                        if file.display().to_string()[0..3] == "src".to_string(){
                            println!("{}:{}", file.display().to_string(), lineno);
                        }
                    }
                }
            }
        }

        ::std::process::exit(1);
    }
}
