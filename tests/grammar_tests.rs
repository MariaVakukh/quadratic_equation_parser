use anyhow::anyhow;
use pest::Parser;
use quadratic_equation_parser::*;


#[test]
fn test_term() -> anyhow::Result<()> {
    for input in ["x^2", "-3x^2", "2x", "5", "-1.5"] 
    {
        let pair = Grammar::parse(Rule::term, input)?
            .next()
            .ok_or_else(|| anyhow!("no pair!"))?;
            
        assert_eq!(pair.as_str(), input);
    }
    Ok(())
}


#[test]
fn test_equation_valid() -> anyhow::Result<()> {
    let valid = [
        "x^2+3x+2=0",
        "-2x^2+x-7=0",
        "0.5x^2 - 1.2x + 3 = 0",
        "x^2 - 4 = 0",
    ];

    for input in valid {
        let pair = Grammar::parse(Rule::equation, input)?
            .next()
            .ok_or_else(|| anyhow!("no pair!"))?;

        assert_eq!(pair.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_file() -> anyhow::Result<()> {

    let pair = Grammar::parse(Rule::file, "x^2+3x+2=0\n-2x^2+x-7=0\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair!"))?;

    assert_eq!(pair.as_str(), "x^2+3x+2=0\n-2x^2+x-7=0\n");

    Ok(())
}


#[test]
fn test_equation_fails() -> anyhow::Result<()> {
    let invalid = [
        "x^2+3x+2=5",
        "x^3+3x+2=0",  
        "2y^2+3y+1=0",  
        "x^2++3x=0",    
        "x^2+3x=",      
    ];

    for input in invalid {
        assert!(Grammar::parse(Rule::equation, input).is_err());
    }
    Ok(())
}


