pub struct UniverseBuilder {}

impl UniverseBuilder {
    pub fn build(&self) -> Universe {
        Universe {}
    }
}

pub struct Universe {}

impl Universe {
    pub fn from_args<I: Iterator<Item = String>>(args: Option<I>) -> Self {
        println!("Hello from eMPIRe!");

        if let Some(args) = args {
            println!("Args are:");
            for arg in args {
                println!("- {}", arg);
            }
        } else {
            println!("Args were not specified")
        }

        Self {}
    }
}

impl Drop for Universe {
    fn drop(&mut self) {
        println!("Buh-bye from eMPIRe!");
    }
}
