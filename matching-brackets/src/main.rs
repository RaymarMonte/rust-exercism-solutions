fn main() {
    let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
        \\end{array}\\right)";
    let mut brackets_only = String::from(input);
    brackets_only.retain(|c| r#"{}[]()"#.contains(c));
    println!("{}", brackets_only.as_str().chars().nth(4).unwrap());
}