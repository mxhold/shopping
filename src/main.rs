extern crate shopping;

use shopping::run;

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
