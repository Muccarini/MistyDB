// MistyDB Sample Program
// This demonstrates all parser features

// Variable declarations
let pi = 3.14159;
let name = "MistyDB";
let version = 1.0;
let is_active = true;

// Arithmetic expressions
let result = (10 + 5) * 2 - 8 / 4;
let complex = pi * 2 + version;

// Function definitions
func greet(name) {
    let message = "Hello, " + name;
    message;
}

func calculate_area(width, height) {
    let area = width * height;
    area;
}

func is_valid(age, premium) {
    let adult = age >= 18;
    let valid = adult && premium || age >= 65;
    valid;
}

// Conditional logic
where age > 18 && status == "active";
where (balance > 0 && premium == true) || trial_days > 0;

// Boolean expressions
let can_access = is_admin || (is_member && subscription_active);
let needs_update = version < 2.0 && last_update > 30;

// Function calls
greet("Alice");
calculate_area(10, 20);
is_valid(25, true);

// Complex nested expressions
let final_score = (base_score * multiplier + bonus) / total_items;
let comparison = x > 0 && x < 100 && y > 0 && y < 100;

// Nested function calls
let max_value = max(min(a, b), min(c, d));
let computed = sqrt(16) + pow(2, 3) - abs(-5);
