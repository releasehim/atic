pub trait EsPrimo {
    fn es_primo(&self) -> bool;
}

impl EsPrimo for i32 {
    fn es_primo(&self) -> bool {
        if *self <= 1 {
            return false;
        }
        let mut i = 2;
        while i * i <= *self {
            if *self % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }
}

pub fn cantidad_primos(v: &Vec<i32>) -> usize {
    let mut count = 0;
    for _ in v.iter().filter(|&&x| x.es_primo()) {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantidad_primos() {
        let v = vec![1, 2, 3, 4, 5, 11, 12, 13];
        assert_eq!(cantidad_primos(&v), 5);
    }
}
