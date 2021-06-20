use nom::{IResult, branch::alt, bytes::complete::{tag, take_while}, character::{complete::{one_of, space1}, is_alphabetic, streaming::alpha0}, combinator::opt, many1, multi::{separated_list1, many0, many1, }, named, sequence::{delimited, preceded, separated_pair, terminated, tuple}};

use crate::expressions::ExprE;

#[derive(Debug)]
pub enum SExpr {
    SSym(String),
    SNum(f64),
    SList(Box<Vec<SExpr>>)
}

static OP_LIST: [char; 6] = ['+', '-', '*', '>', '<', '|'];
impl SExpr {
    fn new_sym(s: &str) -> Self {
        SExpr::SSym(s.to_string())
    }

    fn new_num(n: &str) -> Self {
        let convert_to_f64 = n.parse::<f64>().unwrap();
        SExpr::SNum(convert_to_f64)
    }

    fn new_slist (v: Vec<SExpr>) -> Self {
        SExpr::SList(Box::new(v))
    }
}

// Obviously not complete for all types of floats (yet)
fn parse_num(input: &str) -> IResult<&str, SExpr> {
    let (cont, res) = nom::combinator::recognize(
      many1(
        terminated(one_of("0123456789"), many0(one_of("_")))
      )
    )(input)?;
    Ok((cont, SExpr::new_num(res)))
}

fn parse_symbol(input: &str) -> IResult<&str, SExpr> {
    let (cont, res) = take_while(|c:char| c.is_alphabetic() || OP_LIST.contains(&c))(input)?;
    Ok((cont, SExpr::new_sym(res)))
}

fn parse_str(input: &str) -> IResult<&str, SExpr> {
    let (cont, res) = take_while(|c:char| c.is_alphabetic())(input)?;
    Ok((cont, SExpr::new_sym(res)))
}

fn parse_bare_string(input: &str) -> IResult<&str, SExpr> {
    delimited(tag("\""), parse_str, tag("\""))(input)
}

// Do i need to remove white space for last symbol? rn (function 3 4)
fn parse_slist(input: &str) -> IResult<&str, SExpr> {
    //let (c, _) = nom::bytes::complete::tag("(")(input)?;
    let (cont, _) = nom::bytes::complete::tag("(")(input)?;
    let (c, ret) =  separated_list1(space1, alt((parse_slist, parse_num, parse_symbol)))(cont)?;
    println!("kurwamac, {}", c);
    let (rest,_) = nom::bytes::complete::tag(")")(c)?;
    Ok((rest, SExpr::new_slist(ret)))
}
pub fn parse_sexpr(input: &str ) -> IResult<&str, SExpr> {
    alt((
        parse_num, parse_bare_string, parse_slist
    ))(input)
}


// gotta adjust parser.... :(
// (let (x, y) ())
// Parses slists
fn parse_helper(inp: Box<Vec<SExpr>>) -> ExprE {
   let mut v = inp.into_iter();
   let curr = v.next().unwrap();
   match curr {
       // Here we parse functions and primitive operators
        SExpr::SSym(s ) => {
            match s.as_str() {
                ">"  => {
                    let a = parse(v.next().unwrap());
                    let b = parse(v.next().unwrap());

                    if v.next().is_some() {
                        panic!("Invalid number of arguments")
                    }
                    ExprE::LT(Box::new(a), Box::new(b))
                }
                "<"  => {
                    let a = parse(v.next().unwrap());
                    let b = parse(v.next().unwrap());

                    if v.next().is_some() {
                        panic!("Invalid number of arguments")
                    }
                    ExprE::GT(Box::new(a), Box::new(b))
                }
                "Eq"  => {
                    let a = parse(v.next().unwrap());
                    let b = parse(v.next().unwrap());
                    println!("a: {:?} b: {:?}", a, b);
                    if v.next().is_some() {
                        panic!("Invalid number of arguments")
                    }
                    ExprE::GT(Box::new(a), Box::new(b))
                }
                "if"  => {
                    let cond = parse(v.next().unwrap());
                    let t = parse(v.next().unwrap());
                    let f = parse(v.next().unwrap());
                    ExprE::IfC(Box::new(cond), Box::new(t), Box::new(f))
                }



                "+"   => {
                    let a = parse(v.next().unwrap());
                    let b = parse(v.next().unwrap());

                    if v.next().is_some() {
                        panic!("Invalid number of arguments")
                    }

                    ExprE::Plus(Box::new(a),Box::new(b))
                }
                "*"   => {
                    let a = parse(v.next().unwrap());
                    let b = parse(v.next().unwrap());

                    if v.next().is_some()  {
                        panic!("Invalid number of arguments")
                    }

                    ExprE::Mult(Box::new(a),Box::new(b))
                }
 
                "-"   => {
                    let a = parse(v.next().unwrap());
                    let b = parse(v.next().unwrap());

                    if v.next().is_some()  {
                        panic!("Invalid number of arguments")
                    }

                    ExprE::Minus(Box::new(a),Box::new(b))
                }
                _   => ExprE::Prim(-99999.0)
            }
        },
        _ => ExprE::Prim(-99999.0)
   }
}

//let (_, xexpr) = parse_sexpr(inp).unwrap(); 
pub fn parse(inp: SExpr) -> ExprE {
    // safe to unwrap at this point
    match inp {
        SExpr::SSym(s)    => {
            match s.as_str() {
                "true" => ExprE::TrueC,
                "false" => ExprE::FalseC,
                _       => ExprE::Id(s)
            }
        } 
        SExpr::SNum(x)   => ExprE::Prim(x),
        SExpr::SList(xs) => parse_helper(xs) 
    }
}




// TODO(fixes) 
// (+ 2 2      ) will put "" in the symlist if the spaces are uneven :/
// let's try to get a primitive repl going now -- 1 expr
pub fn testing() {
    println!("THINGS THAT DEFINITELY SHOULDN'T FAIL:");
    println!("{:?}", parse_sexpr("(|x| (|y| (x + y)))").unwrap());
    let (_, s)= parse_sexpr("(if true 5 3)").unwrap();
    println!("{:?}", s);
    println!("{:?}", parse(s))

}

/*
#[cfg(test)]

#[test]

 How do i implement the equality stuff ;/
fn s_list_parser() {
    let (s, v) = parse_slist("(+ 2 2)").unwrap();
    let it = v.iter();
    assert_eq!(it.next(), S)
}
*/
