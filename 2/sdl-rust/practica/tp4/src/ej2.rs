#[derive(Debug, PartialEq)]
pub struct Persona<'a> {
    pub nombre: &'a str,
    pub apellido: &'a str,
    pub direccion: &'a str,
    pub ciudad: &'a str,
    pub salario: f64,
    pub edad: u8,
}

pub fn personas_salario_mayor<'a>(personas: &'a [Persona<'a>], min_salario: f64) -> Vec<&'a Persona<'a>> {
    let mut res = Vec::new();
    for p in personas.iter().filter(|p| p.salario > min_salario) {
        res.push(p);
    }
    res
}

pub fn personas_edad_ciudad<'a>(personas: &'a [Persona<'a>], edad_limite: u8, ciudad: &str) -> Vec<&'a Persona<'a>> {
    let mut res = Vec::new();
    for p in personas.iter().filter(|p| p.edad > edad_limite && p.ciudad == ciudad) {
        res.push(p);
    }
    res
}

pub fn todas_viven_en_ciudad(personas: &[Persona], ciudad: &str) -> bool {
    personas.iter().all(|p| p.ciudad == ciudad)
}

pub fn al_menos_una_en_ciudad(personas: &[Persona], ciudad: &str) -> bool {
    personas.iter().any(|p| p.ciudad == ciudad)
}

pub fn existe_persona(personas: &[Persona], persona: &Persona) -> bool {
    personas.iter().any(|p| p == persona)
}

pub fn obtener_edades(personas: &[Persona]) -> Vec<u8> {
    let mut res = Vec::new();
    for edad in personas.iter().filter_map(|p| Some(p.edad)) {
        res.push(edad);
    }
    res
}

pub fn obtener_min_max_salario<'a>(personas: &'a [Persona<'a>]) -> (Option<&'a Persona<'a>>, Option<&'a Persona<'a>>) {
    if personas.is_empty() {
        return (None, None);
    }
    let mut min_p = &personas[0];
    let mut max_p = &personas[0];

    let es_menor = |a: &Persona, b: &Persona| -> bool {
        if a.salario != b.salario {
            a.salario < b.salario
        } else {
            a.edad > b.edad
        }
    };

    let es_mayor = |a: &Persona, b: &Persona| -> bool {
        if a.salario != b.salario {
            a.salario > b.salario
        } else {
            a.edad > b.edad
        }
    };

    for p in personas.iter() {
        if es_menor(p, min_p) {
            min_p = p;
        }
        if es_mayor(p, max_p) {
            max_p = p;
        }
    }

    (Some(min_p), Some(max_p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personas_filtrado() {
        let p1 = Persona { nombre: "Juan", apellido: "Perez", direccion: "C1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
        let p2 = Persona { nombre: "Ana", apellido: "Gomez", direccion: "C2", ciudad: "La Plata", salario: 1500.0, edad: 25 };
        let p3 = Persona { nombre: "Pedro", apellido: "Lopez", direccion: "C3", ciudad: "CABA", salario: 800.0, edad: 40 };
        let p4 = Persona { nombre: "Maria", apellido: "Rodriguez", direccion: "C4", ciudad: "La Plata", salario: 1500.0, edad: 35 };

        let lista = vec![p1, p2, p3, p4];

        let mayor_1000 = personas_salario_mayor(&lista, 1000.0);
        assert_eq!(mayor_1000.len(), 2);
        assert_eq!(mayor_1000[0].nombre, "Ana");
        assert_eq!(mayor_1000[1].nombre, "Maria");

        let lp_mayores_28 = personas_edad_ciudad(&lista, 28, "La Plata");
        assert_eq!(lp_mayores_28.len(), 2);
        assert_eq!(lp_mayores_28[0].nombre, "Juan");
        assert_eq!(lp_mayores_28[1].nombre, "Maria");

        assert_eq!(todas_viven_en_ciudad(&lista, "La Plata"), false);
        assert_eq!(al_menos_una_en_ciudad(&lista, "CABA"), true);

        let test_p = Persona { nombre: "Juan", apellido: "Perez", direccion: "C1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
        assert_eq!(existe_persona(&lista, &test_p), true);

        assert_eq!(obtener_edades(&lista), vec![30, 25, 40, 35]);

        let (min, max) = obtener_min_max_salario(&lista);
        assert_eq!(min.unwrap().nombre, "Pedro");
        assert_eq!(max.unwrap().nombre, "Maria"); // desempate por edad (35 vs 25 de Ana)
    }
}
