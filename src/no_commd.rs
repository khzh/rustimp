use std::collections::HashMap;

/* Evaluating arithmetic and boolean expressions only -- doesn't require borrowing environments */

/* represents an arithmetic expression */
#[derive(Debug)]
pub enum Arith {
    Num(u64),
    Var(String),
    Add(Box<Arith>, Box<Arith>),
    Mul(Box<Arith>, Box<Arith>)
}

impl Arith {
    /* evaluates an arith */
    fn eval(arith : Arith, environment : HashMap<String, u64>) -> u64 {
        match arith {
            Arith::Num(n) => n,
            Arith::Var(v) => 
                match environment.get(&v){
                    Some(&num) => num.clone(),
                    _ => 1
                },
            Arith::Add(a, b) => (Arith::eval(*a, environment.clone()) + Arith::eval(*b, environment.clone())),
            Arith::Mul(a, b) => (Arith::eval(*a, environment.clone()) * Arith::eval(*b, environment.clone()))
        }
    }
}

/* represents a boolean expression */
#[derive(Debug)]
pub enum Booln {
   True,
   False,
   LessThan(Box<Arith>, Box<Arith>)
}

impl Booln{
    /* evaluates a booln */
    fn eval(booln : Booln, environment : HashMap<String, u64>) -> bool {
        match booln {
            Booln::True => true,
            Booln::False => false,
            Booln::LessThan(a1, a2) => (Arith::eval(*a1, environment.clone()) < Arith::eval(*a2, environment.clone()))
        }
    }
}

/*
/* represents a command */
#[derive(Debug)]
pub enum Commd{
    Skip,
    Assgn{ name: String, expr: Box<Arith> },
    Seq(Box<Commd>, Box<Commd>),
    If(Box<Booln>, Box<Commd>, Box<Commd>),
    While( Box<Booln>, Box<Commd>)
}


impl Commd{
    /* simplifies construction of Commd::Assgn construction */
    fn assgn(name : String, arith: Arith) -> Self {
        Commd::Assgn { name, expr: Box::new(arith) }
    }

    /* evaluates a Commd */
    fn eval(commd : Commd, environment : HashMap<String, u64>) -> HashMap<String, u64> {
        match commd {
            Commd::Skip => environment,
            Commd::Assgn{name, expr} => environment, //environment.insert(name, expr), //TODO
            Commd::Seq(c1, c2) => Commd::eval(*c2, Commd::eval(*c1, environment)),
            Commd::If(b, c1, c2) => //let eb = Booln::eval(b, environment);
                if Booln::eval(*b, environment) {
                    Commd::eval(*c1, environment)
                }
                else {
                    Commd::eval(*c2, environment)
                },
            Commd::While(b, c) =>
                if Booln::eval(*b, environment) {
                    let environment1 = Commd::eval(*c, environment);
                    let iter = Commd::While(b, c);
                    Commd::eval(iter, environment1)
                }
                else {
                    environment
                }
        }
    }
}
*/

fn main() {
    println!("Hello world!");
}

/* Tests below */

/* tests for arith */
#[test]
fn arith_num() {
    let env = HashMap::new();
    let test = Arith::Num(21);
    assert_eq!(Arith::eval(test, env), 21);
 }

#[test]
fn arith_var_v() {
    let mut env = HashMap::new();
    env.insert("test".to_string(), 99);
    let test = Arith::Var("test".to_string());
    assert_eq!(Arith::eval(test, env), 99);
 }

#[test]
fn arith_add_v() {
    let env = HashMap::new();
    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let addexp = Arith::Add(Box::new(arg1), Box::new(arg2));
    assert_eq!(Arith::eval(addexp, env), 9);
 }

#[test]
fn arith_mul_v() {
    let env = HashMap::new();
    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let mulexp = Arith::Mul(Box::new(arg1), Box::new(arg2));
    assert_eq!(Arith::eval(mulexp, env), 20);
 }

#[test]
fn arith_add_v_aexp() {
    let env = HashMap::new();
    let arg = Arith::Num(3);

    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let mulexp = Arith::Mul(Box::new(arg1), Box::new(arg2));

    let addexp = Arith::Add(Box::new(arg), Box::new(mulexp));
    assert_eq!(Arith::eval(addexp, env), 23);
 }

#[test]
fn arith_add_aexp_v() {
    let env = HashMap::new();
    let arg = Arith::Num(3);
    
    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let mulexp = Arith::Mul(Box::new(arg1), Box::new(arg2));

    let addexp = Arith::Add(Box::new(mulexp), Box::new(arg));
    assert_eq!(Arith::eval(addexp, env), 23);
 }

#[test]
fn arith_add_aexp_aexp() {
    let env = HashMap::new();
    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let mulexp1 = Arith::Add(Box::new(arg1), Box::new(arg2));

    let arg3 = Arith::Num(2);
    let arg4 = Arith::Num(3);
    let mulexp2 = Arith::Mul(Box::new(arg3), Box::new(arg4));

    let addexp = Arith::Add(Box::new(mulexp1), Box::new(mulexp2));
    assert_eq!(Arith::eval(addexp, env), 15);
 }


/* tests for booln */
#[test]
fn booln_true() {
    let env = HashMap::new();
    let test = Booln::True;
    assert!(Booln::eval(test, env));
 }

#[test]
fn booln_false() {
    let env = HashMap::new();
    let test = Booln::False;
    assert_eq!(Booln::eval(test, env), false);
 }

#[test]
fn booln_gt_v(){
    let env = HashMap::new();
    let test = Arith::Num(21);
    let test2 = Arith::Num(20);
    let ltexp = Booln::LessThan(Box::new(test), Box::new(test2));
    assert_eq!(Booln::eval(ltexp, env), false);
}

#[test]
fn booln_eq_v(){
    let env = HashMap::new();
    let test = Arith::Num(21);
    let test2 = Arith::Num(21);
    let ltexp = Booln::LessThan(Box::new(test), Box::new(test2));
    assert_eq!(Booln::eval(ltexp, env), false);
}

#[test]
fn booln_lt_v(){
    let env = HashMap::new();
    let test = Arith::Num(20);
    let test2 = Arith::Num(21);
    let ltexp = Booln::LessThan(Box::new(test), Box::new(test2));
    assert!(Booln::eval(ltexp, env));
}