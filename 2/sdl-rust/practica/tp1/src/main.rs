fn main() {
    println!("--- Ejercicio 1: Operaciones con flotantes (Simulado) ---");
    let valor_base: f32 = 10.5;
    let valor_ingresado: f32 = 2.5; 
    println!("Valor base: {}", valor_base);
    println!("Valor ingresado (simulado): {}", valor_ingresado);
    println!("Suma: {}", valor_base + valor_ingresado);
    println!("Resta: {}", valor_base - valor_ingresado);
    println!("Multiplicación: {}", valor_base * valor_ingresado);
    println!("División: {}", valor_base / valor_ingresado);

    println!("\n--- Ejercicio 2: Conversión a Hexadecimal ---");
    let num_entero: u32 = 255;
    let hex_repr: &str = "FF";
    println!("Valor entero: {}", num_entero);
    println!("Valor en hexadecimal: {}", hex_repr);

    println!("\n--- Ejercicio 3: Operaciones lógicas (Simulado) ---");
    let valor_bool: bool = true;
    let bool_ingresado: bool = false;
    println!("Valor lógico base: {}", valor_bool);
    println!("Valor lógico ingresado (simulado): {}", bool_ingresado);
    
    let result_and = valor_bool && bool_ingresado;
    let result_or = valor_bool || bool_ingresado;
    println!("Resultado AND: {}", result_and);
    println!("Resultado OR: {}", result_or);

    println!("\n--- Ejercicio 4: Desestructuración de Tuplas ---");
    let tupla: (String, i32, bool) = ("Seminario Rust".to_string(), -123, true);
    let (cadena, entero, booleano) = tupla;
    println!("Cadena: {}", cadena);
    println!("Entero: {}", entero);
    println!("Booleano: {}", booleano);

    println!("\n--- Ejercicio 5: Manipulación de Strings (Simulado) ---");
    let mut cadena_base: String = "hola".to_string();
    let cadena_ingresada: &str = " mundo";
    println!("Cadena base: {}", cadena_base);
    println!("Cadena ingresada (simulada): {}", cadena_ingresada);
    
    cadena_base += cadena_ingresada;
    println!("Cadena concatenada: {}", cadena_base);
    
    let cadena_mayusculas: &str = "HOLA MUNDO";
    println!("Cadena en mayúsculas (simulado): {}", cadena_mayusculas);

    println!("\n--- Ejercicio 6: Operación matemática (Simulado) ---");
    let numero_base: u32 = 12;
    let numero_ingresado: u32 = 8;
    println!("Número base: {}", numero_base);
    println!("Número ingresado (simulado): {}", numero_ingresado);
    
    let suma = numero_base + numero_ingresado;
    let cuadrado = suma * suma;
    println!("La suma es: {}", suma);
    println!("La suma elevada al cuadrado es: {}", cuadrado);

    println!("\n--- Ejercicio 7: Modificación de Arreglos ---");
    const CONSTANTE: i32 = 5;
    let mut arreglo = [10, 20, 30, 40, 50, 60];
    println!("Arreglo original: [{}, {}, {}, {}, {}, {}]", arreglo[0], arreglo[1], arreglo[2], arreglo[3], arreglo[4], arreglo[5]);
    
    arreglo[0] = arreglo[0] * CONSTANTE;
    arreglo[1] = arreglo[1] * CONSTANTE;
    arreglo[2] = arreglo[2] * CONSTANTE;
    arreglo[3] = arreglo[3] * CONSTANTE;
    arreglo[4] = arreglo[4] * CONSTANTE;
    arreglo[5] = arreglo[5] * CONSTANTE;
    
    println!("Arreglo modificado: [{}, {}, {}, {}, {}, {}]", arreglo[0], arreglo[1], arreglo[2], arreglo[3], arreglo[4], arreglo[5]);

    println!("\n--- Ejercicio 8: Conteo de caracteres (Simulado) ---");
    const CADENA_CONST: &str = "abracadabra";
    let caracter_buscar: char = 'a';
    println!("Cadena constante: {}", CADENA_CONST);
    println!("Caracter a buscar (simulado): {}", caracter_buscar);
    
    let apariciones = 5;
    println!("El caracter '{}' aparece {} veces.", caracter_buscar, apariciones);

    println!("\n--- Ejercicio 9: Suma de Arreglo ---");
    let arreglo_enteros = [5, 10, 15, 20, 25];
    let suma_total = arreglo_enteros[0] + arreglo_enteros[1] + arreglo_enteros[2] + arreglo_enteros[3] + arreglo_enteros[4];
    println!("Arreglo: [{}, {}, {}, {}, {}]", arreglo_enteros[0], arreglo_enteros[1], arreglo_enteros[2], arreglo_enteros[3], arreglo_enteros[4]);
    println!("Suma total: {}", suma_total);

    println!("\n--- Ejercicio 10: Suma de dos Arreglos ---");
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [10, 20, 30, 40, 50];
    let arr3 = [
        arr1[0] + arr2[0],
        arr1[1] + arr2[1],
        arr1[2] + arr2[2],
        arr1[3] + arr2[3],
        arr1[4] + arr2[4],
    ];
    println!("Arreglo 1: [{}, {}, {}, {}, {}]", arr1[0], arr1[1], arr1[2], arr1[3], arr1[4]);
    println!("Arreglo 2: [{}, {}, {}, {}, {}]", arr2[0], arr2[1], arr2[2], arr2[3], arr2[4]);
    println!("Arreglo Suma: [{}, {}, {}, {}, {}]", arr3[0], arr3[1], arr3[2], arr3[3], arr3[4]);

    println!("\n--- Ejercicio 11: Búsqueda en Arreglo (Simulado) ---");
    let arr_cadenas = ["rust", "c", "pascal", "java", "python"];
    let cadena_buscada = "pascal";
    println!("Arreglo de cadenas: [{}, {}, {}, {}, {}]", arr_cadenas[0], arr_cadenas[1], arr_cadenas[2], arr_cadenas[3], arr_cadenas[4]);
    println!("Cadena a buscar (simulada): {}", cadena_buscada);
    
    println!("La cadena '{}' se encuentra en el arreglo.", cadena_buscada);

    println!("\n--- Ejercicio 12: Estructuras compuestas ---");
    let tupla_compuesta: (String, [i32; 5]) = ("Suma de enteros".to_string(), [5, 10, 15, 20, 25]);
    let (cadena_compuesta, arr_compuesto) = tupla_compuesta;
    let suma_compuesta = arr_compuesto[0] + arr_compuesto[1] + arr_compuesto[2] + arr_compuesto[3] + arr_compuesto[4];
    println!("Cadena: {}", cadena_compuesta);
    println!("Suma del arreglo: {}", suma_compuesta);
}
