pub mod a {
pub mod series {
pub mod of {
pub fn nested_modules() {}
}
}
}

fn main() {
a::series::of::nested_modules();
}

pub mod a {
    pub mod series {
    pub mod of {
    pub fn nested_modules() {}
    }
    }
    }
    use a::series::of::nested_modules;
fn main() {
nested_modules();
}

