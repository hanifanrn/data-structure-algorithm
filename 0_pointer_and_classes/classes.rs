// there is no clases in rust, but i try to build it using struct and impl instead
struct Cookie {
    color: String,
}

impl Cookie {
    pub fn new(color: &str) -> Cookie {
        Cookie {
            color: color.to_string(),
        }
    }

    pub fn get_color(&self) -> String {
        return self.color.clone();
    }

    pub fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
    }
}

fn main() {
    let mut cookie_one = Cookie::new("green");
    let cookie_two = Cookie::new("blue");

    cookie_one.set_color("yellow");

    println!("{}", cookie_one.get_color());
    println!("{}", cookie_two.get_color());
}
