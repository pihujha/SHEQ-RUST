// ----------------------------- ExprC -----------------------------

#[derive(Debug, Clone)]
pub enum ExprC {
    NumC(NumC),
    IdC(IdC),
    StringC(StringC),
    IfC(IfC),
    AppC(AppC),
    LamC(LamC),
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
pub struct AppC {
    pub fun: Box<ExprC>,
    pub args: Vec<ExprC>,
}

#[derive(Debug, Clone)]
pub struct LamC {
    pub args: Vec<String>,
    pub body: Box<ExprC>,
}

#[derive(Debug, Clone)]
pub struct IfC {
    pub condition: Box<ExprC>,
    pub then_: Box<ExprC>,
    pub else_: Box<ExprC>,
}

// Keywords list
pub static KEYWORDS: &[&str] = &["if", "let", "in", "end", ":", "="];

// ----------------------------- Env -----------------------------

pub type Environment = Vec<(String, Value)>;

fn top_env() -> Environment {
    vec![
        ("+".to_string(), Value::PrimV(PrimV { s: "+".to_string() })),
        ("-".to_string(), Value::PrimV(PrimV { s: "-".to_string() })),
        ("*".to_string(), Value::PrimV(PrimV { s: "*".to_string() })),
        ("/".to_string(), Value::PrimV(PrimV { s: "/".to_string() })),
        (
            "<=".to_string(),
            Value::PrimV(PrimV {
                s: "<=".to_string(),
            }),
        ),
        (
            "substring".to_string(),
            Value::PrimV(PrimV {
                s: "substring".to_string(),
            }),
        ),
        (
            "strlen".to_string(),
            Value::PrimV(PrimV {
                s: "strlen".to_string(),
            }),
        ),
        (
            "equal?".to_string(),
            Value::PrimV(PrimV {
                s: "equal?".to_string(),
            }),
        ),
        (
            "error".to_string(),
            Value::PrimV(PrimV {
                s: "error".to_string(),
            }),
        ),
        ("true".to_string(), Value::BoolV(BoolV { b: true })),
        ("false".to_string(), Value::BoolV(BoolV { b: false })),
    ]
}
// ----------------------------- Value -----------------------------

#[derive(Debug, Clone)]
pub enum Value {
    NumV(NumV),
    BoolV(BoolV),
    StringV(StringV),
    CloV(CloV),
    PrimV(PrimV),
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

#[derive(Debug, Clone)]
pub struct CloV {
    pub params: Vec<String>,
    pub body: Box<ExprC>,
    pub env: Environment,
}

#[derive(Debug, Clone)]
pub struct PrimV {
    pub s: String,
}

fn lookup(env: &Environment, name: &str) -> Value {
    env.iter()
        .rev()
        .find(|(n, _)| n == name)
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| panic!("Unbound identifier {name}"))
}

// extend-env will take in a list of params and a list of argvals and return a new environment with
fn extend_env(params: Vec<String>, argvals: Vec<Value>, mut env: Environment) -> Environment {
    if params.len() != argvals.len() {
        panic!("diff size lengths")
    }
    for i in 0..params.len() {
        env.push((params[i].clone(), argvals[i].clone()));
    }

    return env.to_vec();
}

fn top_interp(expr: ExprC) -> Value {
    interp(expr, &top_env())
}

// interp interprets the given AST, outputting the result of running the given expressions & functions
fn interp(expr: ExprC, env: &Environment) -> Value {
    match expr {
        ExprC::NumC(nc) => Value::NumV(NumV { n: nc.n }),
        ExprC::IdC(idc) => lookup(env, &idc.name),
        ExprC::StringC(sc) => Value::StringV(StringV { s: sc.s }),
        ExprC::IfC(if_c) => match interp(*if_c.condition, env) {
            Value::BoolV(bv) => {
                if bv.b {
                    interp(*if_c.then_, env)
                } else {
                    interp(*if_c.else_, env)
                }
            }
            other => panic!("If expected boolean, got {:?}", other),
        },
        ExprC::LamC(lamc) => Value::CloV(CloV {
            params: lamc.args,
            body: lamc.body,
            env: env.to_vec(),
        }),
        ExprC::AppC(appc) => {
            let funval = interp(*appc.fun, env);
            let argval: Vec<Value> = appc.args.iter().map(|a| interp(a.clone(), env)).collect();
            match funval {
                Value::PrimV(pv) => interp_prim(&pv.s, argval),
                Value::CloV(cv) => {
                    let ext_env = extend_env(cv.params, argval, cv.env);
                    interp(*cv.body, &ext_env)
                }
                other => panic!("Not a PrimV or CloV  {:?}", other),
            }
        }
    }
}

//Serialize the sheq4 values, Outputting the result as a string
fn serialize(val: Value) -> String {
    match val {
        Value::NumV(nv) => nv.n.to_string(),
        Value::BoolV(bv) => bv.b.to_string(),
        Value::StringV(sv) => sv.s.to_string(),
        Value::CloV(_) => "#<procedure>".to_string(),
        Value::PrimV(_) => "#<primop>".to_string(),
    }
}

//TODO
pub fn interp_prim(op: &str, args: Vec<Value>) -> Value {
    match (op, args.as_slice()) {
        ("+", [Value::NumV(a), Value::NumV(b)]) =>
            Value::NumV(NumV { n: a.n + b.n }),
        ("-", [Value::NumV(a), Value::NumV(b)]) =>
            Value::NumV(NumV { n: a.n - b.n}),
        ("*", [Value::NumV(a), Value::NumV(b)]) =>
            Value::NumV(NumV { n: a.n * b.n }),
        ("/", [Value::NumV(a), Value::NumV(b)]) =>
            Value::NumV(NumV { n: a.n / b.n }),
        ("<=", [Value::NumV(a), Value::NumV(b)]) =>
            Value::BoolV(BoolV { b: a.n <= b.n }),
        _ => Value::StringV(StringV {s : "Not implemented".to_string()}),
    }
}

fn main() {
    let expr: ExprC = ExprC::NumC(NumC { n: 10.0 });
    let val = top_interp(expr);
    serialize(val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numc_interp_serialize() {
        let expr: ExprC = ExprC::NumC(NumC { n: 10.0 });
        let val = top_interp(expr);
        let output = serialize(val);
        assert_eq!(output, "10");
    }

    #[test]
    fn test_stringc_interp_serialize() {
        let expr: ExprC = ExprC::StringC(StringC {
            s: "Hello world!".to_string(),
        });
        let val = top_interp(expr);
        let output = serialize(val);
        assert_eq!(output, "Hello world!");
    }

    #[test]
    fn test_idc_interp_lookup() {
        let env = vec![("x".to_string(), Value::NumV(NumV { n: 42.0 }))];
        let expr = ExprC::IdC(IdC {
            name: "x".to_string(),
        });

        let val = interp(expr, &env);

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

        let val = interp(expr, &env);

        match val {
            Value::NumV(nv) => assert_eq!(nv.n, 1.0),
            other => panic!("Unexpected value for IfC: {:?}", other),
        }
    }

    #[test]
    fn test_lamc_interp_serialize() {
        let expr: ExprC = ExprC::LamC(LamC {
            args: vec!["f".to_string()],
            body: Box::new(ExprC::NumC(NumC { n: 5.0 })),
        });
        let val = top_interp(expr);
        let output = serialize(val);
        assert_eq!(output, "#<procedure>");
    }

    #[test]
    fn test_arith_prims() {
        // Shape:
        // (AppC (IdC '*') 
        //       (AppC (IdC '+') (list (NumC 10.0) (NumC 5.0)))
        // (NumC 8.0))
        let expr = ExprC::AppC(AppC {
            fun: Box::new(ExprC::IdC(IdC {name: "*".to_string()})),
            args: vec![
                ExprC::AppC(AppC {
                    fun: Box::new(ExprC::IdC(IdC {name: "-".to_string()})),
                    args: vec![
                        ExprC::NumC(NumC { n: 10.0}),
                        ExprC::NumC(NumC { n: 5.0})
                    ],
                }),
                ExprC::NumC(NumC { n: 8.0}),
            ],
        });
        let val = top_interp(expr);
        let output = serialize(val);
        assert_eq!(output, "40");
    }

    #[test]
    fn test_leq() {
        let expr = ExprC::IfC(IfC {
            condition: Box::new(ExprC::AppC(AppC {
                fun: Box::new(ExprC::IdC(IdC {name: "<=".to_string()})),
                args: vec![
                    ExprC::NumC(NumC { n: 5.0}),
                    ExprC::NumC(NumC { n: 10.0}),
                ],
            })),
            then_: Box::new(ExprC::NumC(NumC { n: 1.0})),
            else_: Box::new(ExprC::NumC(NumC { n: 2.0})),
        });
        let val = top_interp(expr);
        let output = serialize(val);
        assert_eq!(output, "1");

    }


    #[test]
    fn test_appc_lam_add() {
        //Delete this before we submit or show code this is jsut what the test looks like
        // (AppC (LamC '(x)
        //              (AppC (IdC '+) (list (NumC 1) (IdC 'x))))
        //       (list (NumC 15)))

        let expr: ExprC = ExprC::AppC(AppC {
            fun: Box::new(ExprC::LamC(LamC {
                args: vec!["x".to_string()],
                body: Box::new(ExprC::AppC(AppC {
                    fun: Box::new(ExprC::IdC(IdC {
                        name: "+".to_string(),
                    })),
                    args: vec![
                        ExprC::NumC(NumC { n: 1.0 }),
                        ExprC::IdC(IdC {
                            name: "x".to_string(),
                        }),
                    ],
                })),
            })),
            args: vec![ExprC::NumC(NumC { n: 15.0 })],
        });
        let val = top_interp(expr);
        let output = serialize(val);
        assert_eq!(output, "16");
    }
}
