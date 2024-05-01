/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Value(v) => Ok(v),
        Expression::Op { op, left, right } => {
            let left = match eval(*left) {
                Ok(v) => v,
                e @ Err(_) => return e, // or `Err(e) => return Err(e),`
            };

            let right = match eval(*right) {
                Ok(v) => v,
                e @ Err(_) => return e, // or `Err(e) => return Err(e),`
            };

            Ok(match op {
                Operation::Add => left + right,
                Operation::Sub => left - right,
                Operation::Mul => left * right,
                Operation::Div => {
                    if right == 0 {
                        return Err(String::from("division by zero"));
                    } else {
                        left / right
                    }
                }
            })
        }
    }


    // Solution 1
    // match e {
    //     Expression::Value(v) => Ok(v),
    //     Expression::Op { op, left, right } => {
    //         let left = match *left {
    //             Expression::Value(v) => Ok(v),
    //             Expression::Op { op, left, right } => eval(Expression::Op { op: op, left: left, right: right })
    //         };

    //         let right = match *right {
    //             Expression::Value(v) => Ok(v),
    //             Expression::Op { op, left, right } => eval(Expression::Op { op: op, left: left, right: right })
    //         };

    //         match op {
    //             Operation::Add => Ok(left.unwrap() + right.unwrap()),
    //             Operation::Sub => Ok(left.unwrap() - right.unwrap()),
    //             Operation::Mul => Ok(left.unwrap() * right.unwrap()),
    //             Operation::Div => {
    //                 let right = right.unwrap();

    //                 if right == 0 {
    //                     return Err(String::from("division by zero"));
    //                 }

    //                 Ok(left.unwrap() / right)
    //             }
    //         }
    //     }
    // }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}


fn main() {
    let result = eval(Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(20)),
    });

    println!("10 + 20 = {:?}", result);
}
