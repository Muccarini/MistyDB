use misty_db::interpreter::tokenizer::Tokenizer;
use misty_db::interpreter::parser::Parser;

fn test(name: &str, source: &str) {
    println!("=== {} ===", name);
    println!("Source: {}", source);
    match Tokenizer::tokenize(source.to_string()) {
        Ok(tokens) => {
            match Parser::new(tokens).parse() {
                Ok(ast) => println!("✓ Success: {:#?}\n", ast),
                Err(e) => println!("✗ Parse Error: {}\n", e),
            }
        }
        Err(e) => println!("✗ Tokenize Error: {}\n", e),
    }
}

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║  MistyDB Parser Complete Test Suite   ║");
    println!("╚════════════════════════════════════════╝\n");

    // Basic arithmetic
    test("Simple addition", "2 + 3;");
    test("Complex arithmetic", "10 + 20 * 3 - 5 / 2;");
    test("Parentheses", "(10 + 5) * 2;");

    // Comparisons
    test("Equality", "x == 42;");
    test("Greater than", "a > b;");
    test("Less or equal", "count <= 100;");

    // Logical operators
    test("Logical AND", "x > 5 && y < 10;");
    test("Logical OR", "a == 0 || b == 1;");
    test("Complex boolean", "(x > 0 && x < 100) || y == 50;");
    test("AND/OR precedence", "a || b && c || d;");

    // Let statements
    test("Let with number", "let x = 42;");
    test("Let with string", "let name = \"MistyDB\";");
    test("Let with expression", "let result = (10 + 5) * 2;");
    test("Let with boolean", "let flag = x > 0 && y < 100;");

    // Where statements
    test("Where simple", "where age > 18;");
    test("Where complex", "where (status == \"active\" && balance > 0) || premium == true;");

    // Function calls
    test("Function call no args", "print();");
    test("Function call one arg", "sqrt(16);");
    test("Function call multiple args", "add(10, 20, 30);");
    test("Nested function calls", "max(min(a, b), c);");
    test("Function in expression", "x + sqrt(25) * 2;");

    // Function definitions
    test("Function no params", 
        "func hello() { print(\"Hello\"); }");
    
    test("Function one param", 
        "func double(x) { x * 2; }");
    
    test("Function multiple params", 
        "func add(a, b) { a + b; }");
    
    test("Function with body",
        "func calculate(x, y) {
            let sum = x + y;
            let product = x * y;
            sum + product;
        }");

    // Multiple statements
    test("Multiple statements",
        "let x = 10;
         let y = 20;
         x + y;");

    // Mixed types
    test("Mixed literals",
        "let num = 42;
         let text = \"hello\";
         let bool = true;");

    // Complex expressions
    test("Everything combined",
        "func validate(age, name, premium) {
            let is_adult = age >= 18;
            let has_name = name != \"\";
            where is_adult && has_name || premium;
        }");

    test("Nested parentheses",
        "((1 + 2) * (3 + 4)) / ((5 - 1) * 2);");

    test("Chain comparisons with logic",
        "x > 0 && x < 100 && y > 0 && y < 100 || z == 0;");

    println!("╔════════════════════════════════════════╗");
    println!("║         All Tests Completed!           ║");
    println!("╚════════════════════════════════════════╝");
}
