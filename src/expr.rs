use std::{fmt::Display, string::ParseError};

#[forbid(clippy::style, clippy::str_to_string, clippy::string_to_string)]
#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
    Str(&'a str),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprOperator<'a> {
    Add(Value<'a>, Value<'a>),
    Sub(Value<'a>, Value<'a>),
    Mul(Value<'a>, Value<'a>),
    Div(Value<'a>, Value<'a>),
    Pow(Value<'a>, Value<'a>),
}

macro_rules! expr_identity {
    ($e:expr) => {
        $e
    };
}

#[macro_export]
macro_rules! _v {
    ($t:ident) => {
        println!("$type")
    };
}

#[macro_export]
macro_rules! _val_from {
    ($t:ident,$($type:ident),+#$v:expr) => {
        {
            let x = _val_from!($t # $v);
            if x == Value::None {
                _val_from!($($type),+#$v)
            } else {
                x
            }
        }
    };
    ($type:ident # $val:expr) => {
        // TODO: Implement negative numbers
        if $val.contains('+') || $val.contains('-') || $val.contains('*') || $val.contains('/') {
            $val.to_string().parse()
        } else if "0123456789".contains($val.trim().chars().nth(0).unwrap()) {
            $val.trim().parse()
        } else if $val.trim().to_lowercase().starts_with(stringify!($type).to_lowercase().as_str()) {
            $val.trim()[(stringify!($type).len()+1)..].trim().parse()
        } else {
            Value::None
        }
    }
}

impl From<String> for Value {
    fn from(val: String) -> Value {
        let x = _val_from!(Str, I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, Char # val);
        if x == Value::None {
            parse_expr(val)
        } else {
            x
        }
    }
}

impl From<&str> for Value<'a> {
    fn from(val: &str) -> Result<Value, _> {
        Ok(Value::from(val.to_string()))
    }
}

impl<'a> ToString for Value<'a> {
    fn to_string(&self) -> String {
        use Value::*;
        match self {
            Str(a) => a.clone().to_owned(),
            Char(a) => a.to_string(),
            I8(a) => a.to_string(),
            I16(a) => a.to_string(),
            I32(a) => a.to_string(),
            I64(a) => a.to_string(),
            I128(a) => a.to_string(),
            U8(a) => a.to_string(),
            U16(a) => a.to_string(),
            U32(a) => a.to_string(),
            U64(a) => a.to_string(),
            U128(a) => a.to_string(),
            None => "None".to_string(),
        }
    }
}

macro_rules! _avi {
    ($t:tt,$j:expr,$x:expr,$y:ident,$a:ty,$b:expr,$c:ident,$($d:ident),+$(,)?) => {
        if let $c(b) = $x {
            let f = b as $a;
            return $y(expr_identity!($b $t f))
        } else {
            _avi!($t,$j,$x,$y,$a,$b,$($d),+)
        }
    };

    ($t:tt,$j:expr,$x:expr,$y:ident,$a:ty,$b:expr,$c:ident$(,)?) => {
        if let $c(b) = $x {
            let f = b as $a;
            return $y(expr_identity!($b $t f))
        } else {
            return expr_identity!($x $t $j)
        }
    }
}

macro_rules! op {
    ($name:ident,$fname:ident,$op:tt,$str:expr) => {
        impl<'a> std::ops::$name for Value<'a> {
            type Output = Value<'a>;
            fn $fname(self, _rhs: Value) -> Value {
                use Value::*;
                match self {
                    Str(a) => $str(a, _rhs),
                    I8(a) => { _avi!($op, self, _rhs, I8, i8, a, I8, U8) },
                    I16(a) => { _avi!($op, self, _rhs, I16, i16, a, I8, I16, U8, U16) },
                    I32(a) => { _avi!($op, self, _rhs, I32, i32, a, I8, I16, I32, U8, U16, U32) },
                    I64(a) => { _avi!($op, self,_rhs,I64,i64,a,I8,I16,I32,I64,U8,U16,U32,U64) },
                    I128(a) => { _avi!($op, self,_rhs,I128,i128,a,I8,I16,I32,I64,U8,U16,U32,U64) },
                    U8(a) => { _avi!($op, self,_rhs,U8,u8,a,U8,U8) },
                    U16(a) => { _avi!($op, self,_rhs,U16,u16,a,U8,U16) },
                    U32(a) => { _avi!($op, self,_rhs,U32,u32,a,U8,U16,U32) },
                    U64(a) => { _avi!($op, self,_rhs,U64,u64,a,U8,U16,U32,U64) },
                    U128(a) => { _avi!($op, self,_rhs,U128,u128,a,U8,U16,U32,U64,U128) },
                    None => None,
                    Char(a) => { Str(a.to_string().as_str()) $op _rhs },
                }
            }
        }
    }
}

op!(Add, add, +, |a: &str, _rhs: Value| {
    let mut x = a.to_string();
    x.push_str(_rhs.to_string().as_str());
    Str(x.as_str())
});

op!(Sub, sub, -, |_a: &str, _rhs: Value| {
    todo!("Subtract strings")
});

op!(Mul, mul, *, |_a: &str, _rhs: Value| {
    todo!("Multiply strings");
});

op!(Div, div, /, |_a: &str, _rhs: Value| {
    todo!("Divide strings");
});

enum ValueParseError<'a> {
    InvalidRHS(&'a str),
}

// TODO: Allow using variables in expressions
impl<'a> std::str::FromStr for Value<'a> {
    type Err = ValueParseError<'a>;

    fn from_str(
        val: &str, /*, vars: &mut Vec<(String, Value)>*/
    ) -> std::result::Result<Value<'a>, ValueParseError<'a>> {
        if val.trim().to_lowercase() == "none" {
            return Ok(Value::None);
        }

        let mut x: Value = Value::None;

        let mut a: Vec<&str> = val.split('+').collect();
        if a.len() > 1 {
            let v: Vec<Value> = a.iter().map(|a| Value::from(*a)).collect();
            x = v[0].clone();
            for i in v[1..].iter() {
                x = x + i.clone();
            }
        } else {
            a = val.split('-').collect();
            if a.len() > 1 {
                let v: Vec<Value> = a.iter().map(|a| Value::from(*a)).collect();
                x = v[0].clone();
                for i in v[1..].iter() {
                    x = x - i.clone();
                }
            } else {
                a = val.split('*').collect();
                if a.len() > 1 {
                    let v: Vec<Value> = a.iter().map(|a| Value::from(*a)).collect();
                    x = v[0].clone();
                    for i in v[1..].iter() {
                        x = x * i.clone();
                    }
                } else {
                    a = val.split('/').collect();
                    if a.len() > 1 {
                        let v: Vec<Value> = a.iter().map(|a| Value::from(*a)).collect();
                        x = v[0].clone();
                        for i in v[1..].iter() {
                            x = x / i.clone();
                        }
                    } else {
                        // ...
                    }
                }
            }
        }

        if x == Value::None {}
        Ok(x)
    }
}
