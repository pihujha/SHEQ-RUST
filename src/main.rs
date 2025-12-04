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
    pub s: String,
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

fn interp(expr: ExprC) -> Value {
    match expr {
        ExprC::NumC(nc) => Value::NumV(NumV { n: nc.n }),
        ExprC::IdC(idc) => unimplemented!("IdC not implemented for {}", idc.name),
        ExprC::StringC(sc) => Value::StringV(StringV { s: sc.s }),
        ExprC::IfC(if_c) => unimplemented!("IfC not implemented for {:?}", if_c.condition), // :? => print a developer-friendly debug representation of a value
        ExprC::LambdaC(_) => unimplemented!("LambdaC not implemented"),
        ExprC::AppC(_) => unimplemented!("AppC not implemented"),
    }
}

fn serialize(val: Value) -> String {
    match val {
        Value::NumV(nv) => nv.n.to_string(),
        Value::BoolV(bv) => bv.b.to_string(),
        Value::StringV(sv) => sv.s.to_string(),
        Value::CloV(_) => "#<procedure>".to_string(),
        Value::PrimV(_) => "#<primop>".to_string(),
    }
}

fn main() {
    let expr: ExprC = ExprC::NumC(NumC { n: 10.0 });
    let val = interp(expr);
    serialize(val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numc_interp_serialize() {
        let expr: ExprC = ExprC::NumC(NumC { n: 10.0 });
        let val = interp(expr);
        let output = serialize(val);
        assert_eq!(output, "10");
    }

    #[test]
    fn test_stringc_interp_serialize() {
        let expr: ExprC = ExprC::StringC(StringC {
            s: "Hello world!".to_string(),
        });
        let val = interp(expr);
        let output = serialize(val);
        assert_eq!(output, "Hello world!");
    }
}
