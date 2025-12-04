// ----------------------------- ExprC -----------------------------

#[derive(Debug, Clone)]
pub enum ExprC {
    NumC(NumC),
    IdC(IdC),
    StringC(StringC),
    IfC(IfC),
}

#[derive(Debug, Clone)]
pub struct NumC {
    pub n: f64,
}

#[derive(Debug, Clone)]
pub struct IdC {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct StringC {
    pub s: String,
}

#[derive(Debug, Clone)]
pub struct IfC {
    pub condition: Box<ExprC>,
    pub then_: Box<ExprC>,
    pub else_: Box<ExprC>,
}

// Keywords list
pub static KEYWORDS: &[&str] = &["if", "let", "in", "end", ":", "="];

// ----------------------------- Value -----------------------------

#[derive(Debug, Clone)]
pub enum Value {
    NumV(NumV),
    BoolV(BoolV),
    StringV(StringV),
}

#[derive(Debug, Clone)]
pub struct NumV {
    pub n: f64,
}

#[derive(Debug, Clone)]
pub struct BoolV {
    pub b: bool,
}

#[derive(Debug, Clone)]
pub struct StringV {
    pub s: String,
}

pub type Environment = Vec<(String, Value)>;

fn lookup(env: &Environment, name: &str) -> Value {
    env.iter()
        .rev()
        .find(|(n, _)| n == name)
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| panic!("Unbound identifier {name}"))
}

fn interp(expr: ExprC) -> Value {
    interp_with_env(expr, &vec![])
}

fn interp_with_env(expr: ExprC, env: &Environment) -> Value {
    match expr {
        ExprC::NumC(nc) => Value::NumV(NumV { n: nc.n }),
        ExprC::IdC(idc) => lookup(env, &idc.name),
        ExprC::StringC(sc) => Value::StringV(StringV { s: sc.s }),
        ExprC::IfC(if_c) => match interp_with_env(*if_c.condition, env) {
            Value::BoolV(bv) => {
                if bv.b {
                    interp_with_env(*if_c.then_, env)
                } else {
                    interp_with_env(*if_c.else_, env)
                }
            }
            other => panic!("If expected boolean, got {:?}", other),
        },
    }
}

fn serialize(val: Value) -> String {
    match val {
        Value::NumV(nv) => nv.n.to_string(),
        Value::BoolV(bv) => bv.b.to_string(),
        Value::StringV(sv) => sv.s.to_string(),
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

    #[test]
    fn test_idc_interp_lookup() {
        let env = vec![("x".to_string(), Value::NumV(NumV { n: 42.0 }))];
        let expr = ExprC::IdC(IdC {
            name: "x".to_string(),
        });

        let val = interp_with_env(expr, &env);

        match val {
            Value::NumV(nv) => assert_eq!(nv.n, 42.0),
            other => panic!("Unexpected value for IdC: {:?}", other),
        }
    }

    #[test]
    fn test_ifc_branches() {
        let env = vec![
            ("cond".to_string(), Value::BoolV(BoolV { b: true })),
            ("other".to_string(), Value::NumV(NumV { n: 99.0 })),
        ];

        let expr = ExprC::IfC(IfC {
            condition: Box::new(ExprC::IdC(IdC {
                name: "cond".to_string(),
            })),
            then_: Box::new(ExprC::NumC(NumC { n: 1.0 })),
            else_: Box::new(ExprC::IdC(IdC {
                name: "other".to_string(),
            })),
        });

        let val = interp_with_env(expr, &env);

        match val {
            Value::NumV(nv) => assert_eq!(nv.n, 1.0),
            other => panic!("Unexpected value for IfC: {:?}", other),
        }
    }

}
