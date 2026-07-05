pub mod fecha;
pub mod ej1;
pub mod ej2;
pub mod ej3;
pub mod ej4;
pub mod ej5;

fn main() {
    let v_primos = vec![2, 3, 4, 5, 6, 7];
    println!("Cantidad primos: {}", ej1::cantidad_primos(&v_primos));

    let p = ej2::Persona {
        nombre: "Ana",
        apellido: "Gomez",
        direccion: "C2",
        ciudad: "La Plata",
        salario: 1500.0,
        edad: 25,
    };
    println!("Persona existe: {}", ej2::existe_persona(&[p], &ej2::Persona {
        nombre: "Ana",
        apellido: "Gomez",
        direccion: "C2",
        ciudad: "La Plata",
        salario: 1500.0,
        edad: 25,
    }));

    let mut platform = ej3::StreamingRust::new();
    let f = fecha::Fecha::new(1, 1, 2026);
    platform.crear_usuario("User".to_string(), Some(ej3::TipoSuscripcion::Basic), ej3::MedioPago::Efectivo, f);
    println!("Suscripcion activa mas contratada: {:?}", platform.suscripcion_mas_contratada_activas());

    let sistema = ej4::SistemaVentas::new(10.0);
    println!("Descuento newsletter: {}", sistema.descuento_newsletter);

    let xyz = ej5::XYZPlataforma::new();
    println!("XYZ transacciones: {}", xyz.transacciones.len());
}
