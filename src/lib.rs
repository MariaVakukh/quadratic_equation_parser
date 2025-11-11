use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Parse Error: {source}")]
    ParseNumber {
        input: String,
        source: std::num::ParseFloatError,
    },
    #[error("Invalid term format: '{term}'")]
    InvalidTermFormat { term: String },
}

pub fn extract_coeff(pairs: pest::iterators::Pairs<Rule>) -> Result<(f64, f64, f64), ParserError> {
    let mut coeff = (0.0, 0.0, 0.0);

    for pair in pairs {
        process_pair(pair, &mut coeff)?;
    }

    Ok(coeff)
}

fn process_pair(
    pair: pest::iterators::Pair<Rule>,
    coeff: &mut (f64, f64, f64),
) -> Result<(), ParserError> {
    match pair.as_rule() {
        Rule::quad_term => coeff.0 += parse_term(pair.as_str(), "x^2")?,
        Rule::linear_term => coeff.1 += parse_term(pair.as_str(), "x")?,
        Rule::constant_term => coeff.2 += parse_const(pair.as_str())?,
        _ => {
            let inner = pair.into_inner();
            if inner.clone().count() > 0 {
                let (a, b, c) = extract_coeff(inner)?;
                coeff.0 += a;
                coeff.1 += b;
                coeff.2 += c;
            }
        }
    }
    Ok(())
}

fn parse_term(s: &str, suffix: &str) -> Result<f64, ParserError> {
    let s = s.replace(" ", "");
    let (sign, content) = match s.chars().next() {
        Some('+') => (1.0, &s[1..]),
        Some('-') => (-1.0, &s[1..]),
        _ => (1.0, s.as_str()),
    };

    let num_part = if let Some(stripped) = content.strip_suffix(&format!("*{}", suffix)) {
        stripped
    } else if let Some(stripped) = content.strip_suffix(suffix) {
        stripped
    } else {
        content
    };

    if num_part.is_empty() {
        Ok(sign * 1.0)
    } else {
        let parsed = match num_part.parse::<f64>() {
            Ok(n) => n,
            Err(source) => {
                return Err(ParserError::ParseNumber {
                    input: num_part.to_string(),
                    source,
                });
            }
        };
        Ok(sign * parsed)
    }
}

fn parse_const(s: &str) -> Result<f64, ParserError> {
    let cleaned = s.replace(" ", "");
    let parsed = match cleaned.parse::<f64>() {
        Ok(val) => val,
        Err(source) => {
            return Err(ParserError::ParseNumber {
                input: cleaned,
                source,
            });
        }
    };
    Ok(parsed)
}

pub fn solve(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    if a == 0.0 {
        return None;
    }
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        None
    } else {
        let sqrt_d = d.sqrt();
        Some(((-b + sqrt_d) / (2.0 * a), (-b - sqrt_d) / (2.0 * a)))
    }
}
