struct User {
    name: String,
    email: String,
    age: u8,
    salary: f32,
    is_active: bool,
    user_role: UserRole,
    website: Website,
}

// Basic Enum
enum UserRole {
    BASIC,
    PRO,
    PREMIUM,
    ADMIN,
}

// Custom Enum
enum Website {
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String),
}

fn main() {
    let user:User = User {
        name: String::from("Rafael"),
        email: String::from("Rafael@gmail.com"),
        age: 51,
        salary: 68957.45,
        is_active: true,
        user_role: UserRole::ADMIN,
        website: Website::LINKEDIN(String::from("https://www.linkedin.com/in/rafaelhernamperez/")),
    };

    print_user(&user);
    print_line();
}

fn print_line() {
    println!("----------");
}

// This function prints the user fields
fn print_user(user: &User) {
    println!("Name: {}", user.name);
    println!("Email: {}", user.email);
    println!("Age: {}", user.age);
    println!("Salary: {}", user.salary);
    println!("Is_Active: {}", user.is_active);
    println!("User role: {}", get_role(&user.user_role));
    println!("Website: {}", get_website(&user.website));
}

// This function evaluates the role and returns the corresponding role string
fn get_role(user_role: &UserRole) -> String {
    match user_role {
        UserRole::ADMIN => String::from("Admin"),
        UserRole::BASIC => String::from("Basic"),
        UserRole::PRO => String::from("Professional"),
        UserRole::PREMIUM => String::from("Premium"),
    }
}

// This function evaluates the role and returns the corresponding role string
fn get_website(website: &Website) -> String {
    match website {
        Website::URL(i) => format!("URL: {}", i),
        Website::INSTAGRAM(i) => format!("Instagram: {}", i),
        Website::LINKEDIN(i) => format!("LinkedIn: {}", i),
        Website::FACEBOOK(i) => format!("Facebook: {}", i),
    }
}
