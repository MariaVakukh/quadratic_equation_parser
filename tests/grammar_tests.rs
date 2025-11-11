use anyhow::anyhow;
use pest::Parser;
use quadratic_equation_parser::*;

#[test]
fn test_number() -> anyhow::Result<()> {
    let valid = ["123", "-456", "3.14", "-0.5", ".25", "-.75"];
    for input in valid {
        let pair = Grammar::parse(Rule::number, input)?
            .next()
            .ok_or_else(|| anyhow!("no pair!"))?;
        assert_eq!(pair.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_number_fail() {
    let invalid = ["4-.з4", "2..45", "8--e3", "3.4.5", "--2"];
    for input in invalid {
        assert!(Grammar::parse(Rule::number, input).is_err());
    }
}

#[test]
fn test_quad_term() -> anyhow::Result<()> {
    let valid = [
        "x^2",
        "-x^2",
        "2*x^2",
        "-3.5*x^2",
        "1*x^2",
        "-3*x^2",
        "  4  * x^2",
        "+5*x^2",
    ];
    for input in valid {
        let pair = Grammar::parse(Rule::quad_term, input)?
            .next()
            .ok_or_else(|| anyhow!("no pair!"))?;
        assert_eq!(pair.as_str().replace(" ", ""), input.replace(" ", ""));
    }
    Ok(())
}

#[test]
fn test_quad_term_fail() -> anyhow::Result<()> {
    let invalid = ["x^3", "2x", "3", "x^^2", "**x^2", "x^"];
    for input in invalid {
        assert!(Grammar::parse(Rule::quad_term, input).is_err());
    }
    Ok(())
}

#[test]
fn test_linear_term() -> anyhow::Result<()> {
    let valid = [
        "x", "-x", "2*x", "-3.5*x", "1*x", "-3*x", "  4  * x", "+5*x",
    ];
    for input in valid {
        let pair = Grammar::parse(Rule::linear_term, input)?
            .next()
            .ok_or_else(|| anyhow!("no pair!"))?;
        assert_eq!(pair.as_str().replace(" ", ""), input.replace(" ", ""));
    }
    Ok(())
}

#[test]
fn test_linear_term_fail() -> anyhow::Result<()> {
    let invalid = ["x^2", "2", "**x", "x^^2", "3*x^3"];
    for input in invalid {
        assert!(Grammar::parse(Rule::linear_term, input).is_err());
    }
    Ok(())
}

#[test]
fn test_constant_term() -> anyhow::Result<()> {
    let valid = ["0", "-5", "3.14", "-0.25", "  7  ", "+12"];
    for input in valid {
        let pair = Grammar::parse(Rule::constant_term, input)?
            .next()
            .ok_or_else(|| anyhow!("no pair!"))?;
        assert_eq!(pair.as_str().replace(" ", ""), input.replace(" ", ""));
    }
    Ok(())
}

#[test]
fn test_equation() -> anyhow::Result<()> {
    let valid = [
        "x^2+3*x+2=0",
        "x-2*x^2-7=0",
        "0.5*x^2 - 1.2*x + 3 = 0",
        "x^2 - 4 = 0",
        " 2 * x^2 + 3*x + 1 = 0",
        "+x^2 + x + 1 = 0",
    ];
    for input in valid {
        let pair = Grammar::parse(Rule::equation, input)?
            .next()
            .ok_or_else(|| anyhow!("no pair!"))?;
        assert_eq!(pair.as_str().replace(" ", ""), input.replace(" ", ""));
    }
    Ok(())
}

#[test]
fn test_equation_fail() -> anyhow::Result<()> {
    let invalid = [
        "x^3+3*x+2=0",
        "2*y^2+3*y+1=0",
        "x^2++3*x=0",
        "x^2+3*x=",
        "5=9",
        "3*x+2=1",
        "2*x^2 = 2*x + 3",
        "x^^2+1=0",
        "*x^2+1=0",
    ];
    for input in invalid {
        assert!(Grammar::parse(Rule::equation, input).is_err());
    }
    Ok(())
}

#[test]
fn test_file() -> anyhow::Result<()> {
    let input = "x^2+3*x+2=0\n-2*x^2+x-7=0\n0.5*x^2 - 1.2*x + 3 = 0\n";
    let pair = Grammar::parse(Rule::file, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair!"))?;
    assert_eq!(pair.as_str().replace(" ", ""), input.replace(" ", ""));
    Ok(())
}

#[test]
fn test_file_fail() -> anyhow::Result<()> {
    let invalid = "x^2+3*x+2=0\n5=9\n";
    assert!(Grammar::parse(Rule::file, invalid).is_err());
    Ok(())
}
