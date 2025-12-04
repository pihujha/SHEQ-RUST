// ----------------------------- ExprC -----------------------------

#[derive(Debug)]
pub enum ExprC {
    NumC(NumC),
    IdC(IdC),
    StringC(StringC),
    IfC(IfC),
    LambdaC(LambdaC),
    AppC(AppC),
}

#[derive(Debug)]
pub struct NumC {
    pub n: f64,
}

#[derive(Debug)]
pub struct IdC {
    pub name: String,
}

#[derive(Debug)]
pub struct StringC {
    pub str_: String,
}

#[derive(Debug)]
pub struct IfC {
    pub condition: Box<ExprC>,
    pub then_: Box<ExprC>,
    pub else_: Box<ExprC>,
}

#[derive(Debug)]
pub struct LambdaC {
    pub args: Vec<String>,
    pub body: Box<ExprC>,
}

#[derive(Debug)]
pub struct AppC {
    pub fun: Box<ExprC>,
    pub args: Vec<ExprC>,
}


// Keywords list
pub static KEYWORDS: &[&str] = &["if", "let", "in", "end", "lambda", ":", "="];

// ----------------------------- Value -----------------------------

pub enum Value {
    NumV(NumV),
    BoolV(BoolV),
    StringV(StringV),
    CloV(CloV),
    PrimV(PrimV),
}

pub struct NumV {
    pub n: f64,
}

pub struct BoolV {
    pub b: bool,
}

pub struct StringV {
    pub s: String,
}

pub struct CloV {
    pub params: Vec<String>,
    pub body: ExprC,
    pub env: Environment,
}

pub struct PrimV {
    pub name: String,
    pub func: fn(Vec<Value>) -> Value,
}

pub type Environment = Vec<(String, Value)>; 

fn main() {
    // interp function?
    let expr = ExprC::NumC(NumC { n: 10.0 });
    println!("{:?}", expr)
}

