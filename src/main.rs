extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::fmt;

mod errors{
    error_chain! { }
}

use errors::*;

#[derive(Debug, Deserialize, PartialEq)]
struct Department(String);

impl fmt::Display for Department {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
struct Product {
    name: String,
    department: Department,
}

//struct Quantity(String);
//
//struct Recipe {
//    name: String,
//    ingredients: HashMap<Product, Quantity>,
//}

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

fn run() -> Result<()> {
    let departments: Vec<Department> = load_departments("inputs/departments.csv")?;
    let products: Vec<Product> = load_products(&departments, "inputs/products.csv")?;

    println!("{:?}", products);
    println!("{:?}", departments);


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
