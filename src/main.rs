use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub langgrammer); // synthesized by LALRPOP

fn main() {
    match langgrammer::TermParser::new().parse("22") {
        Ok(v) => {
            assert_eq!(v, 22);
            println!("Success");
        }
        Err(err) => {
            println!("Err: {}", err);
        }
    }
    
    assert!(langgrammer::TermParser::new().parse("22").is_ok());
    assert!(langgrammer::TermParser::new().parse("(22)").is_ok());
    assert!(langgrammer::TermParser::new().parse("((((22))))").is_ok());
    assert!(langgrammer::TermParser::new().parse("((22)").is_err());
}