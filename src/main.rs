use std::collections::HashMap;

/* Evaluating arithmetic and boolean expressions only -- requires borrowing environments 
(environments are not modified from pure arithmetic and boolean expression evaluation) */

/* represents an arithmetic expression */
#[derive(Debug, Clone)]
pub enum Arith {
    Num(u64),
    Var(String),
    Add(Box<Arith>, Box<Arith>),
    Mul(Box<Arith>, Box<Arith>)
}

impl Arith {
    /* evaluates an arith */
    fn eval(arith : Arith, environment : &HashMap<String, u64>) -> u64 {
        match arith {
            Arith::Num(n) => n,
            Arith::Var(v) => 
                match environment.get(&v){
                    Some(&num) => num.clone(),
                    _ => panic!("Undefined variable!") //not sure what to do here...
                },
            Arith::Add(a, b) => (Arith::eval(*a, &environment) + Arith::eval(*b, &environment)),
            Arith::Mul(a, b) => (Arith::eval(*a, &environment) * Arith::eval(*b, &environment))
        }
    }
}

/* represents a boolean expression */
#[derive(Debug, Clone)]
pub enum Booln {
   True,
   False,
   LessThan(Box<Arith>, Box<Arith>),
   Not(Box<Booln>) //not yet tested
}

impl Booln{
    /* evaluates a booln */
    fn eval(booln : Booln, environment : &HashMap<String, u64>) -> bool {
        match booln {
            Booln::True => true,
            Booln::False => false,
            Booln::LessThan(a1, a2) => (Arith::eval(*a1, &environment) < Arith::eval(*a2, &environment)),
            Booln::Not(b) => !(Booln::eval(*b, &environment))
        }
    }
}

/* represents a command */
#[derive(Debug, Clone)]
pub enum Commd{
    Skip,
    Assgn{ name: String, expr: Box<Arith> },
    Seq(Box<Commd>, Box<Commd>),
    If(Box<Booln>, Box<Commd>, Box<Commd>),
    While(Box<Booln>, Box<Commd>)
}

impl Commd{
    /* simplifies construction of Commd::Assgn construction */
    fn assgn(name : String, arith: Arith) -> Self {
        Commd::Assgn { name, expr: Box::new(arith) }
    }

    /* evaluates a Commd */
    fn eval(commd : Commd, mut environment : HashMap<String, u64>) -> HashMap<String, u64> {
        match commd {
            Commd::Skip => environment,
            Commd::Assgn{name, expr} =>
                match environment.insert(name, Arith::eval(*expr, &environment)) {
                    _ => environment
                },
            Commd::Seq(c1, c2) => Commd::eval(*c2, Commd::eval(*c1, environment)),
            Commd::If(b, c1, c2) =>
                if Booln::eval(*b, &environment) {
                    Commd::eval(*c1, environment)
                }
                else {
                    Commd::eval(*c2, environment)
                },
            Commd::While(ref b, ref c) =>
                if Booln::eval(*(b.clone()), &environment) {
                    let environment1 = Commd::eval(*(c.clone()), environment);
                    Commd::eval(commd.clone(), environment1)
                }
                else {
                    environment
                }
        }
    }
}

fn main() {
    println!("Hello world!");
}

/* Tests below */

/* tests for arith */

#[test]
fn arith_num() {
    let env = HashMap::new();
    let test = Arith::Num(21);
    assert_eq!(Arith::eval(test, &env), 21);
 }

#[test]
fn arith_var_v() {
    let mut env = HashMap::new();
    env.insert("test".to_string(), 99);
    let test = Arith::Var("test".to_string());
    assert_eq!(Arith::eval(test, &env), 99);
 }

#[test]
fn arith_add_v() {
    let env = HashMap::new();
    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let addexp = Arith::Add(Box::new(arg1), Box::new(arg2));
    assert_eq!(Arith::eval(addexp, &env), 9);
 }

#[test]
fn arith_mul_v() {
    let env = HashMap::new();
    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let mulexp = Arith::Mul(Box::new(arg1), Box::new(arg2));
    assert_eq!(Arith::eval(mulexp, &env), 20);
 }

#[test]
fn arith_add_v_aexp() {
    let env = HashMap::new();
    let arg = Arith::Num(3);

    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let mulexp = Arith::Mul(Box::new(arg1), Box::new(arg2));

    let addexp = Arith::Add(Box::new(arg), Box::new(mulexp));
    assert_eq!(Arith::eval(addexp, &env), 23);
 }

#[test]
fn arith_add_aexp_v() {
    let env = HashMap::new();
    let arg = Arith::Num(3);
    
    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let mulexp = Arith::Mul(Box::new(arg1), Box::new(arg2));

    let addexp = Arith::Add(Box::new(mulexp), Box::new(arg));
    assert_eq!(Arith::eval(addexp, &env), 23);
 }

#[test]
fn arith_add_aexp_aexp() {
    let env = HashMap::new();
    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let addexp1 = Arith::Add(Box::new(arg1), Box::new(arg2));

    let arg3 = Arith::Num(2);
    let arg4 = Arith::Num(3);
    let mulexp2 = Arith::Mul(Box::new(arg3), Box::new(arg4));

    let addexp = Arith::Add(Box::new(addexp1), Box::new(mulexp2));
    assert_eq!(Arith::eval(addexp, &env), 15);
 }

#[test]
fn arith_add_var(){
    let mut env = HashMap::new();
    env.insert("x".to_string(), 10);
    let x = Arith::Var("x".to_string());

    let n = Arith::Num(42);

    let addexp = Arith::Add(Box::new(x), Box::new(n));

    assert_eq!(Arith::eval(addexp, &env), 52);
}


/* tests for booln */
#[test]
fn booln_true() {
    let env = HashMap::new();
    let test = Booln::True;
    assert!(Booln::eval(test, &env));
 }

#[test]
fn booln_false() {
    let env = HashMap::new();
    let test = Booln::False;
    assert_eq!(Booln::eval(test, &env), false);
 }

#[test]
fn booln_gt_v(){
    let env = HashMap::new();
    let test = Arith::Num(21);
    let test2 = Arith::Num(20);
    let ltexp = Booln::LessThan(Box::new(test), Box::new(test2));
    assert_eq!(Booln::eval(ltexp, &env), false);
}

#[test]
fn booln_eq_v(){
    let env = HashMap::new();
    let test = Arith::Num(21);
    let test2 = Arith::Num(21);
    let ltexp = Booln::LessThan(Box::new(test), Box::new(test2));
    assert_eq!(Booln::eval(ltexp, &env), false);
}

#[test]
fn booln_lt_v(){
    let env = HashMap::new();
    let test = Arith::Num(20);
    let test2 = Arith::Num(21);
    let ltexp = Booln::LessThan(Box::new(test), Box::new(test2));
    assert!(Booln::eval(ltexp, &env));
}

/* tests for commd */
#[test]
fn commd_skip(){
    let empty = HashMap::new();
    let env = HashMap::new();
    let sk = Commd::Skip;
    assert_eq!(Commd::eval(sk, empty.clone()), env);
}

#[test]
fn commd_assgn_v(){
    let empty = HashMap::new();

    let mut expected = HashMap::new();
    expected.insert("temp".to_string(), 999);

    let ag = Commd::assgn("temp".to_string(), Arith::Num(999));
    assert_eq!(Commd::eval(ag, empty), expected);
}

#[test]
fn commd_assgn_aexp(){
    let empty = HashMap::new();

    let mut expected = HashMap::new();
    expected.insert("temp".to_string(), 9);

    let arg1 = Arith::Num(4);
    let arg2 = Arith::Num(5);
    let aexp = Arith::Add(Box::new(arg1), Box::new(arg2));
    let ag = Commd::assgn("temp".to_string(), aexp);

    assert_eq!(Commd::eval(ag, empty), expected);
}

#[test]
fn commd_seq_skip_skip(){
    let empty = HashMap::new();

    let expected = HashMap::new();
    let sk1 = Commd::Skip;
    let sk2 = Commd::Skip;

    let seq = Commd::Seq(Box::new(sk1), Box::new(sk2));

    assert_eq!(Commd::eval(seq, empty), expected);
}

#[test]
fn commd_seq_assgn_assgn(){
    let empty = HashMap::new();

    let mut expected = HashMap::new();
    expected.insert("var1".to_string(), 52);
    expected.insert("var2".to_string(), 823);

    let ass1 = Commd::assgn("var1".to_string(), Arith::Num(52));
    let ass2 = Commd::assgn("var2".to_string(), Arith::Num(823));

    let seq = Commd::Seq(Box::new(ass1), Box::new(ass2));

    assert_eq!(Commd::eval(seq, empty), expected);
}

#[test]
fn commd_if_true(){
    let empty = HashMap::new();

    let mut expected = HashMap::new();
    expected.insert("var".to_string(), 99);

    let b = Booln::True;
    let c1 = Commd::assgn("var".to_string(), Arith::Num(99));
    let c2 = Commd::assgn("var".to_string(), Arith::Num(42));

    let iftrue = Commd::If(Box::new(b), Box::new(c1), Box::new(c2));

    assert_eq!(Commd::eval(iftrue, empty), expected);
}

#[test]
fn commd_if_false(){
    let empty = HashMap::new();

    let mut expected = HashMap::new();
    expected.insert("var".to_string(), 42);

    let b = Booln::False;
    let c1 = Commd::assgn("var".to_string(), Arith::Num(99));
    let c2 = Commd::assgn("var".to_string(), Arith::Num(42));

    let iftrue = Commd::If(Box::new(b), Box::new(c1), Box::new(c2));

    assert_eq!(Commd::eval(iftrue, empty), expected);
}

#[test]
fn commd_if_arith(){
    let empty = HashMap::new();

    let mut expected = HashMap::new();
    expected.insert("var".to_string(), 99);

    let a1 = Arith::Num(42);
    let a2 = Arith::Num(99);
    let b = Booln::LessThan(Box::new(a1), Box::new(a2));
    let c1 = Commd::assgn("var".to_string(), Arith::Num(99));
    let c2 = Commd::assgn("var".to_string(), Arith::Num(42));

    let iftrue = Commd::If(Box::new(b), Box::new(c1), Box::new(c2));

    assert_eq!(Commd::eval(iftrue, empty), expected);
}

#[test]
fn commd_while_false(){
    let empty = HashMap::new();

    let expected = HashMap::new();
    
    let c = Commd::assgn("ok".to_string(), Arith::Num(42));
    let whilst = Commd::While(Box::new(Booln::False), Box::new(c));

    assert_eq!(Commd::eval(whilst, empty), expected);
}

#[test]
fn commd_while_add(){
    let mut init = HashMap::new();
    init.insert("i".to_string(), 0);

    let mut expected = HashMap::new();
    expected.insert("i".to_string(), 5);

    //"while i < 5"
    let aiter = Arith::Var("i".to_string());
    let an = Arith::Num(5);
    let b = Booln::LessThan(Box::new(aiter), Box::new(an));

    //"i = i + 1;"
    let addexp = Arith::Add(Box::new(Arith::Var("i".to_string())), Box::new(Arith::Num(1)));
    let c = Commd::assgn("i".to_string(), addexp);

    let whilst = Commd::While(Box::new(b), Box::new(c));

    assert_eq!(Commd::eval(whilst, init), expected);
}