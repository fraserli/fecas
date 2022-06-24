use crate::parser::{Atom, Expression, Operator::*};

use num::{traits::Pow, BigRational, ToPrimitive};

impl Expression {
    pub fn compute(self) -> BigRational {
        let mut total = self.lhs.compute();

        for (op, expr) in self.operations {
            total = match op {
                Addition => total + expr.compute(),
                Subtraction => total - expr.compute(),
                Multiplication => total * expr.compute(),
                Division => total / expr.compute(),
                Power => {
                    let index = expr.compute();
                    if index.is_integer() {
                        total.pow(index.to_integer())
                    } else {
                        let base = total.to_f64().unwrap();
                        BigRational::from_float(base.pow(index.to_f64().unwrap())).unwrap()
                    }
                }
            };
        }

        total
    }
}

impl Atom {
    pub fn compute(self) -> BigRational {
        match self {
            Self::Number(number) => number,
            Self::Expr(expression) => expression.compute(),
        }
    }
}
