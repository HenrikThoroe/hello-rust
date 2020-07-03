pub fn pow(lhs: &i32, rhs: &i32) -> i32 {
    let mut res = *lhs;

    for _ in 1..*rhs {
        res *= lhs;
    }

    res
}

pub fn mult(lhs: &i32, rhs: &i32) -> i32 {
    let mut res = *lhs;

    for _ in 1..*rhs {
        res += lhs;
    }

    res
}