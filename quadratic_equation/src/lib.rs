use float_cmp::approx_eq;
use simple_error::SimpleError;

pub fn solve(a: f64, b: f64, c: f64) -> Result<Vec<f64>, SimpleError> {
    // Не валидные параметры
    if !a.is_finite() || !b.is_finite() || !c.is_finite() {
        return Err(SimpleError::new("Invalid params"));
    }
    // Линейное уровнение, не имеет решения
    if approx_eq!(f64, a, 0.0) {
        return Ok(vec![]);
    }

    let d = b.powi(2) - 4.0 * a * c;
    println!("{:?}", d);
    // Решений не имеет
    if d < 0.0 {
        return Ok(vec![]);
    }

    if approx_eq!(f64, d, 0.0) {
        return Ok(vec![-b / (2.0 * a), (-b / (2.0 * a))]);
    }

    return Ok(vec![
        ((-b + d.sqrt()) / 2.0 * a),
        ((-b - d.sqrt()) / (2.0 * a)),
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //  x^2+1 = 0 корней нет
    fn testcase1() {
        assert_eq!(Vec::<f64>::new(), solve(1.0, 0.0, 1.0).unwrap());
    }

    #[test]
    //  x^2-1 = 0, x1=1, x2=-1
    fn testcase2() {
        assert_eq!(vec![1.0, -1.0], solve(1.0, 0.0, -1.0).unwrap());
    }

    #[test]
    //  x^2+2x+1 = 0 x1=-1, x2=-1
    fn testcase3() {
        assert_eq!(vec![-1.0, -1.0], solve(1.0, 2.0, 1.0).unwrap());
    }

    #[test]
    //  x^2+2x+1 = 0 x1=-0, x2=-0  D == 0.0
    fn testcase3_1() {
        assert_eq!(
            vec![-0.0, -0.0],
            solve(1.00000001, 0.0, -0.000000000000000000001).unwrap()
        );
    }

    #[test]
    //  Коэфициенты по 0
    fn testcase4() {
        assert_eq!(Vec::<f64>::new(), solve(0.0, 0.0, 0.0).unwrap());
    }

    #[test]
    //  non f64 type
    fn testcase5() {
        assert_eq!(
            SimpleError::new("Invalid params"),
            solve(f64::NAN, 0.0, 0.0).unwrap_err()
        );
    }

    #[test]
    //  second is nan
    fn testcase6() {
        assert_eq!(
            SimpleError::new("Invalid params"),
            solve(0.0, f64::NAN, 0.0).unwrap_err()
        );
    }

    #[test]
    //  third is inf
    fn testcase7() {
        assert_eq!(
            SimpleError::new("Invalid params"),
            solve(0.0, 0.0, f64::INFINITY).unwrap_err()
        );
    }
}
