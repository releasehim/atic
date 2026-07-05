fn es_par(n: i32) -> bool {
    n % 2 == 0
}

fn es_primo(n: u32) -> bool {
    let mut i = 2;
    let mut primo = true;
    while i * i <= n {
        if n % i == 0 {
            primo = false;
        }
        i += 1;
    }
    primo
}

fn suma_pares(arreglo: [i32; 5]) -> i32 {
    let mut suma = 0;
    for elemento in arreglo {
        if elemento % 2 == 0 {
            suma += elemento;
        }
    }
    suma
}

fn cantidad_impares(arreglo: [i32; 5]) -> i32 {
    let mut cant = 0;
    for elemento in arreglo {
        if elemento % 2 != 0 {
            cant += 1;
        }
    }
    cant
}

fn duplicar_valores(arreglo: [f32; 5]) -> [f32; 5] {
    let mut nuevo = [0.0, 0.0, 0.0, 0.0, 0.0];
    let mut i = 0;
    while i < 5 {
        nuevo[i] = arreglo[i] * 2.0;
        i += 1;
    }
    nuevo
}

fn longitud_de_cadenas(arreglo: [String; 5]) -> [usize; 5] {
    let mut resultado = [0, 0, 0, 0, 0];
    let mut i = 0;
    while i < 5 {
        resultado[i] = arreglo[i].len();
        i += 1;
    }
    resultado
}

fn cantidad_de_mayores(arreglo: [i32; 5], limite: i32) -> i32 {
    let mut cant = 0;
    for elemento in arreglo {
        if elemento > limite {
            cant += 1;
        }
    }
    cant
}

fn sumar_arreglos(arr1: [f32; 5], arr2: [f32; 5]) -> [f32; 5] {
    let mut resultado = [0.0, 0.0, 0.0, 0.0, 0.0];
    let mut i = 0;
    while i < 5 {
        resultado[i] = arr1[i] + arr2[i];
        i += 1;
    }
    resultado
}

fn cantidad_en_rango(arreglo: [i32; 5], inferior: i32, superior: i32) -> i32 {
    let mut cant = 0;
    for elemento in arreglo {
        if elemento >= inferior && elemento <= superior {
            cant += 1;
        }
    }
    cant
}

fn cantidad_de_cadenas_mayor_a(arreglo: [String; 5], limite: usize) -> i32 {
    let mut cant = 0;
    for elemento in arreglo {
        if elemento.len() > limite {
            cant += 1;
        }
    }
    cant
}

fn multiplicar_valores(arreglo: &mut [i32; 5], factor: i32) {
    let mut i = 0;
    while i < 5 {
        arreglo[i] = arreglo[i] * factor;
        i += 1;
    }
}

fn reemplazar_pares(arreglo: &mut [i32; 5]) {
    let mut i = 0;
    while i < 5 {
        if arreglo[i] % 2 == 0 {
            arreglo[i] = -1;
        }
        i += 1;
    }
}

fn ordenar_nombres(arreglo: &mut [String; 5]) {
    let mut i = 0;
    while i < 5 {
        let mut j = 0;
        while j < 4 {
            if arreglo[j] > arreglo[j + 1] {
                let temp = arreglo[j].to_string();
                arreglo[j] = arreglo[j + 1].to_string();
                arreglo[j + 1] = temp;
            }
            j += 1;
        }
        i += 1;
    }
}

fn incrementar(valor: &mut f32) {
    *valor += 1.0;
}

fn serie_geometrica(tamano: usize) -> [u32; 5] {
    let mut resultado = [0, 0, 0, 0, 0];
    let mut i = 0;
    let mut valor = 1;
    while i < 5 {
        if i < tamano {
            resultado[i] = valor;
            valor *= 2;
        }
        i += 1;
    }
    resultado
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_par() {
        assert_eq!(es_par(4), true);
        assert_eq!(es_par(7), false);
        assert_eq!(es_par(0), true);
    }

    #[test]
    fn test_es_primo() {
        assert_eq!(es_primo(2), true);
        assert_eq!(es_primo(3), true);
        assert_eq!(es_primo(4), false);
        assert_eq!(es_primo(11), true);
    }

    #[test]
    fn test_suma_pares() {
        assert_eq!(suma_pares([1, 2, 3, 4, 5]), 6);
        assert_eq!(suma_pares([1, 3, 5, 7, 9]), 0);
    }

    #[test]
    fn test_cantidad_impares() {
        assert_eq!(cantidad_impares([1, 2, 3, 4, 5]), 3);
        assert_eq!(cantidad_impares([2, 4, 6, 8, 10]), 0);
    }

    #[test]
    fn test_duplicar_valores() {
        assert_eq!(duplicar_valores([1.0, 2.0, 3.5, 4.0, 5.0]), [2.0, 4.0, 7.0, 8.0, 10.0]);
    }

    #[test]
    fn test_longitud_de_cadenas() {
        let arr = [
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "abcd".to_string(),
            "abcde".to_string(),
        ];
        assert_eq!(longitud_de_cadenas(arr), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_cantidad_de_mayores() {
        assert_eq!(cantidad_de_mayores([1, 10, 3, 15, 5], 5), 2);
        assert_eq!(cantidad_de_mayores([1, 2, 3, 4, 5], 10), 0);
    }

    #[test]
    fn test_sumar_arreglos() {
        let a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let b = [10.0, 20.0, 30.0, 40.0, 50.0];
        assert_eq!(sumar_arreglos(a, b), [11.0, 22.0, 33.0, 44.0, 55.0]);
    }

    #[test]
    fn test_cantidad_en_rango() {
        assert_eq!(cantidad_en_rango([1, 5, 10, 15, 20], 5, 15), 3);
        assert_eq!(cantidad_en_rango([1, 2, 3, 4, 5], 10, 20), 0);
    }

    #[test]
    fn test_cantidad_de_cadenas_mayor_a() {
        let arr = [
            "uno".to_string(),
            "dos".to_string(),
            "tres".to_string(),
            "cuatro".to_string(),
            "cinco".to_string(),
        ];
        assert_eq!(cantidad_de_cadenas_mayor_a(arr, 4), 2);
    }

    #[test]
    fn test_multiplicar_valores() {
        let mut arr = [1, 2, 3, 4, 5];
        multiplicar_valores(&mut arr, 3);
        assert_eq!(arr, [3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_reemplazar_pares() {
        let mut arr = [1, 2, 3, 4, 5];
        reemplazar_pares(&mut arr);
        assert_eq!(arr, [1, -1, 3, -1, 5]);
    }

    #[test]
    fn test_ordenar_nombres() {
        let mut arr = [
            "banana".to_string(),
            "apple".to_string(),
            "pear".to_string(),
            "orange".to_string(),
            "grape".to_string(),
        ];
        ordenar_nombres(&mut arr);
        assert_eq!(
            arr,
            [
                "apple".to_string(),
                "banana".to_string(),
                "grape".to_string(),
                "orange".to_string(),
                "pear".to_string()
            ]
        );
    }

    #[test]
    fn test_incrementar() {
        let mut val = 4.5;
        incrementar(&mut val);
        assert_eq!(val, 5.5);
    }

    #[test]
    fn test_serie_geometrica() {
        assert_eq!(serie_geometrica(3), [1, 2, 4, 0, 0]);
        assert_eq!(serie_geometrica(5), [1, 2, 4, 8, 16]);
    }
}
