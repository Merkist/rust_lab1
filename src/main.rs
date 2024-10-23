use std::io::{self, Write};
use regex::Regex;

fn tokenize(input: &str) -> Vec<&str> {
    let re = Regex::new(r"(\d+\.?\d*|[+\-*/])").unwrap();
    re.find_iter(input)
        .filter_map(|mat| Some(mat.as_str()))
        .collect()
}

struct Calculator {
    last_result: f64,
}

impl Calculator {
    fn new() -> Self {
        Calculator { last_result: 0.0 }
    }

    fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        let mut result = self.last_result;

        // Розділяємо вираз на токени
        let tokens: Vec<&str> = tokenize(expression);
        if tokens.is_empty() {
            return Err("Введіть вираз.".to_string());
        }

        let mut iter = tokens.into_iter();
        while let Some(token) = iter.next() {
            match token.parse::<f64>() {
                Ok(num) => result = num,
                Err(_) => match token {
                    "+" => result += self.get_next_number(&mut iter)?,
                    "-" => result -= self.get_next_number(&mut iter)?,
                    "*" => result *= self.get_next_number(&mut iter)?,
                    "/" => {
                        let divisor = self.get_next_number(&mut iter)?;
                        if divisor == 0.0 {
                            return Err("Помилка: ділення на 0!".to_string());
                        }
                        result /= divisor;
                    }
                    _ => return Err(format!("Невідомий оператор або число: {}", token)),
                },
            }
        }

        self.last_result = result;
        Ok(result)
    }

    fn get_next_number<'a, I>(&self, iter: &mut I) -> Result<f64, String>
    where
        I: Iterator<Item = &'a str>,
    {
        match iter.next() {
            Some(num_str) => match num_str.parse::<f64>() {
                Ok(num) => Ok(num),
                Err(_) => Err(format!("Невірне число: {}", num_str)),
            },
            None => Err("Вираз неповний.".to_string()),
        }
    }
}

fn main() {
    let mut calculator = Calculator::new();

    loop {
        print!("Введіть вираз або 'q' для виходу: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Помилка при зчитуванні вводу.");

        let input = input.trim();

        if input == "q" {
            break;
        }

        match calculator.evaluate(input) {
            Ok(result) => println!("Результат: {}", result),
            Err(err) => println!("Помилка: {}", err),
        }
    }
}

