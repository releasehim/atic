use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Fecha {
    pub dia: u32,
    pub mes: u32,
    pub anio: i32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, anio: i32) -> Self {
        Self { dia, mes, anio }
    }

    pub fn es_bisiesto_anio(anio: i32) -> bool {
        (anio % 4 == 0 && anio % 100 != 0) || anio % 400 == 0
    }

    pub fn es_bisiesto(&self) -> bool {
        Self::es_bisiesto_anio(self.anio)
    }

    pub fn dias_en_mes(mes: u32, anio: i32) -> u32 {
        if mes == 1 || mes == 3 || mes == 5 || mes == 7 || mes == 8 || mes == 10 || mes == 12 {
            31
        } else if mes == 4 || mes == 6 || mes == 9 || mes == 11 {
            30
        } else if mes == 2 {
            if Self::es_bisiesto_anio(anio) { 29 } else { 28 }
        } else {
            0
        }
    }

    pub fn es_fecha_valida(&self) -> bool {
        if self.mes < 1 || self.mes > 12 {
            return false;
        }
        let max_dias = Self::dias_en_mes(self.mes, self.anio);
        self.dia >= 1 && self.dia <= max_dias
    }

    pub fn sumar_dias(&mut self, mut dias: u32) {
        if !self.es_fecha_valida() {
            return;
        }
        while dias > 0 {
            let max_dias = Self::dias_en_mes(self.mes, self.anio);
            let dias_disponibles = max_dias - self.dia;
            if dias <= dias_disponibles {
                self.dia += dias;
                dias = 0;
            } else {
                dias -= dias_disponibles + 1;
                self.dia = 1;
                if self.mes == 12 {
                    self.mes = 1;
                    self.anio += 1;
                } else {
                    self.mes += 1;
                }
            }
        }
    }

    pub fn restar_dias(&mut self, mut dias: u32) {
        if !self.es_fecha_valida() {
            return;
        }
        while dias > 0 {
            if dias < self.dia {
                self.dia -= dias;
                dias = 0;
            } else {
                dias -= self.dia;
                if self.mes == 1 {
                    self.mes = 12;
                    self.anio -= 1;
                } else {
                    self.mes -= 1;
                }
                self.dia = Self::dias_en_mes(self.mes, self.anio);
            }
        }
    }

    pub fn es_mayor(&self, una_fecha: &Fecha) -> bool {
        if self.anio != una_fecha.anio {
            return self.anio > una_fecha.anio;
        }
        if self.mes != una_fecha.mes {
            return self.mes > una_fecha.mes;
        }
        self.dia > una_fecha.dia
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoSuscripcion {
    Basic,
    Classic,
    Super,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MedioPago {
    Efectivo,
    MercadoPago(String),
    TarjetaCredito(String, String),
    TransferenciaBancaria(String),
    Cripto(String),
}

#[derive(Debug, Clone)]
pub struct Suscripcion {
    pub tipo: TipoSuscripcion,
    pub costo_mensual: f64,
    pub meses: u32,
    pub fecha_inicio: Fecha,
}

#[derive(Debug, Clone)]
pub struct Usuario {
    pub nombre: String,
    pub suscripcion: Option<Suscripcion>,
    pub medio_pago: MedioPago,
}

#[derive(Debug, Clone)]
pub struct HistorialContrato {
    pub tipo_suscripcion: TipoSuscripcion,
    pub medio_pago: MedioPago,
}

/*

CONSIGNA DEL ENTREGABLE:

estacion_con_mas_suscripciones(&self) -> Option<EstacionTop>

Retorna la estación del año con mayor cantidad de suscripciones activas,
junto con su cantidad, el mes más contratado dentro de esa estación
y la cantidad de ese mes

*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NombreEstacion {
    Otonio,
    Invierno,
    Primavera,
    Verano,
}

pub struct Estacion {
    pub nombre: NombreEstacion,
    pub fecha_inicio: Fecha,
    pub fecha_fin: Fecha,
}

impl Estacion {
    pub fn new(nombre: NombreEstacion, fecha_inicio: Fecha, fecha_fin: Fecha) -> Self {
        Self {
            nombre,
            fecha_inicio,
            fecha_fin,
        }
    }

    pub fn determinar_estacion_segun_fecha(fecha: Fecha) -> Option<NombreEstacion> {
        let mes = fecha.mes;
        let dia = fecha.dia;

        if (mes == 12 && dia >= 21) || mes == 1 || mes == 2 || (mes == 3 && dia <= 20) {
            Some(NombreEstacion::Verano)
        } else if (mes == 3 && dia >= 21) || mes == 4 || mes == 5 || (mes == 6 && dia <= 20) {
            Some(NombreEstacion::Otonio)
        } else if (mes == 6 && dia >= 21) || mes == 7 || mes == 8 || (mes == 9 && dia <= 20) {
            Some(NombreEstacion::Invierno)
        } else if (mes == 9 && dia >= 21) || mes == 10 || mes == 11 || (mes == 12 && dia <= 20) {
            Some(NombreEstacion::Primavera)
        } else {
            None
        }
    }

    pub fn from_nombre(nombre: NombreEstacion) -> Self {
        match nombre {
            NombreEstacion::Verano => {
                Self::new(nombre, Fecha::new(21, 12, 0), Fecha::new(20, 3, 0))
            }
            NombreEstacion::Otonio => Self::new(nombre, Fecha::new(21, 3, 0), Fecha::new(20, 6, 0)),
            NombreEstacion::Invierno => {
                Self::new(nombre, Fecha::new(21, 6, 0), Fecha::new(20, 9, 0))
            }
            NombreEstacion::Primavera => {
                Self::new(nombre, Fecha::new(21, 9, 0), Fecha::new(20, 12, 0))
            }
        }
    }
}

pub struct EstacionTop {
    pub estacion: Estacion,
    pub total_suscripciones_estacion: u32,
    pub mes_top: u32,
    pub cantidad_suscripciones_mes_top: u32,
}

impl EstacionTop {
    pub fn new(
        estacion: Estacion,
        total_suscripciones_estacion: u32,
        mes_top: u32,
        cantidad_suscripciones_mes_top: u32,
    ) -> Self {
        Self {
            estacion,
            total_suscripciones_estacion,
            mes_top,
            cantidad_suscripciones_mes_top,
        }
    }
}

pub struct StreamingRust {
    pub usuarios: Vec<Usuario>,
    pub historial: Vec<HistorialContrato>,
}

impl Default for StreamingRust {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamingRust {
    pub fn new() -> Self {
        Self {
            usuarios: Vec::new(),
            historial: Vec::new(),
        }
    }

    pub fn crear_usuario(
        &mut self,
        nombre: String,
        tipo_sub: Option<TipoSuscripcion>,
        medio: MedioPago,
        fecha: Fecha,
    ) {
        let sub = match tipo_sub {
            Some(t) => {
                self.historial.push(HistorialContrato {
                    tipo_suscripcion: t,
                    medio_pago: medio.clone(),
                });
                let costo = match t {
                    TipoSuscripcion::Basic => 100.0,
                    TipoSuscripcion::Classic => 200.0,
                    TipoSuscripcion::Super => 300.0,
                };
                Some(Suscripcion {
                    tipo: t,
                    costo_mensual: costo,
                    meses: 1,
                    fecha_inicio: fecha,
                })
            }
            None => None,
        };
        self.usuarios.push(Usuario {
            nombre,
            suscripcion: sub,
            medio_pago: medio,
        });
    }

    pub fn upgrade_suscripcion(&mut self, index: usize) {
        if index >= self.usuarios.len() {
            return;
        }
        let u = &mut self.usuarios[index];
        if let Some(ref mut s) = u.suscripcion {
            let nuevo_tipo = match s.tipo {
                TipoSuscripcion::Basic => Some(TipoSuscripcion::Classic),
                TipoSuscripcion::Classic => Some(TipoSuscripcion::Super),
                TipoSuscripcion::Super => None,
            };
            if let Some(t) = nuevo_tipo {
                s.tipo = t;
                s.costo_mensual = match t {
                    TipoSuscripcion::Basic => 100.0,
                    TipoSuscripcion::Classic => 200.0,
                    TipoSuscripcion::Super => 300.0,
                };
                self.historial.push(HistorialContrato {
                    tipo_suscripcion: t,
                    medio_pago: u.medio_pago.clone(),
                });
            }
        }
    }

    pub fn downgrade_suscripcion(&mut self, index: usize) {
        if index >= self.usuarios.len() {
            return;
        }
        let u = &mut self.usuarios[index];
        let mut cancelar = false;
        if let Some(ref mut s) = u.suscripcion {
            let nuevo_tipo = match s.tipo {
                TipoSuscripcion::Super => Some(TipoSuscripcion::Classic),
                TipoSuscripcion::Classic => Some(TipoSuscripcion::Basic),
                TipoSuscripcion::Basic => {
                    cancelar = true;
                    None
                }
            };
            if let Some(t) = nuevo_tipo {
                s.tipo = t;
                s.costo_mensual = match t {
                    TipoSuscripcion::Basic => 100.0,
                    TipoSuscripcion::Classic => 200.0,
                    TipoSuscripcion::Super => 300.0,
                };
                self.historial.push(HistorialContrato {
                    tipo_suscripcion: t,
                    medio_pago: u.medio_pago.clone(),
                });
            }
        }
        if cancelar {
            u.suscripcion = None;
        }
    }

    pub fn cancelar_suscripcion(&mut self, index: usize) {
        if index < self.usuarios.len() {
            self.usuarios[index].suscripcion = None;
        }
    }

    pub fn nombre_medio(mp: &MedioPago) -> &'static str {
        match mp {
            MedioPago::Efectivo => "Efectivo",
            MedioPago::MercadoPago(_) => "MercadoPago",
            MedioPago::TarjetaCredito(_, _) => "TarjetaCredito",
            MedioPago::TransferenciaBancaria(_) => "TransferenciaBancaria",
            MedioPago::Cripto(_) => "Cripto",
        }
    }

    pub fn medio_pago_mas_utilizado_activas(&self) -> Option<&'static str> {
        let mut counts = HashMap::new();
        for u in self.usuarios.iter().filter(|u| u.suscripcion.is_some()) {
            let name = Self::nombre_medio(&u.medio_pago);
            *counts.entry(name).or_insert(0) += 1;
        }
        let mut max_name = None;
        let mut max_count = 0;
        for (name, count) in counts {
            if count > max_count {
                max_count = count;
                max_name = Some(name);
            }
        }
        max_name
    }

    pub fn suscripcion_mas_contratada_activas(&self) -> Option<TipoSuscripcion> {
        let mut counts = HashMap::new();
        for u in self.usuarios.iter().filter_map(|u| u.suscripcion.as_ref()) {
            *counts.entry(u.tipo).or_insert(0) += 1;
        }
        let mut max_tipo = None;
        let mut max_count = 0;
        for (tipo, count) in counts {
            if count > max_count {
                max_count = count;
                max_tipo = Some(tipo);
            }
        }
        max_tipo
    }

    pub fn medio_pago_mas_utilizado_historico(&self) -> Option<&'static str> {
        let mut counts = HashMap::new();
        for h in &self.historial {
            let name = Self::nombre_medio(&h.medio_pago);
            *counts.entry(name).or_insert(0) += 1;
        }
        let mut max_name = None;
        let mut max_count = 0;
        for (name, count) in counts {
            if count > max_count {
                max_count = count;
                max_name = Some(name);
            }
        }
        max_name
    }

    pub fn suscripcion_mas_contratada_historico(&self) -> Option<TipoSuscripcion> {
        let mut counts = HashMap::new();
        for h in &self.historial {
            *counts.entry(h.tipo_suscripcion).or_insert(0) += 1;
        }
        let mut max_tipo = None;
        let mut max_count = 0;
        for (tipo, count) in counts {
            if count > max_count {
                max_count = count;
                max_tipo = Some(tipo);
            }
        }
        max_tipo
    }

    pub fn estacion_con_mas_suscripciones(&self) -> Option<EstacionTop> {
        let mut conteo_por_estacion: HashMap<NombreEstacion, u32> = HashMap::new();
        let mut conteo_por_mes: HashMap<(NombreEstacion, u32), u32> = HashMap::new();

        for u in &self.usuarios {
            if let Some(s) = &u.suscripcion
                && let Some(estacion) = Estacion::determinar_estacion_segun_fecha(s.fecha_inicio)
            {
                *conteo_por_estacion.entry(estacion).or_insert(0) += 1;
                *conteo_por_mes
                    .entry((estacion, s.fecha_inicio.mes))
                    .or_insert(0) += 1;
            }
        }

        let mut estacion_top = None;
        let mut total_suscripciones_estacion = 0;
        for (estacion, cantidad) in &conteo_por_estacion {
            if *cantidad > total_suscripciones_estacion {
                total_suscripciones_estacion = *cantidad;
                estacion_top = Some(*estacion);
            }
        }
        let estacion_top = estacion_top?;

        let mut mes_top = 0;
        let mut cantidad_suscripciones_mes_top = 0;
        for ((estacion, mes), cantidad) in &conteo_por_mes {
            if *estacion == estacion_top && *cantidad > cantidad_suscripciones_mes_top {
                cantidad_suscripciones_mes_top = *cantidad;
                mes_top = *mes;
            }
        }

        Some(EstacionTop::new(
            Estacion::from_nombre(estacion_top),
            total_suscripciones_estacion,
            mes_top,
            cantidad_suscripciones_mes_top,
        ))
    }
}

fn main() {
    println!("Hola, entregable 2!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_rust() {
        let mut platform = StreamingRust::new();
        let f = Fecha::new(1, 1, 2026);

        platform.crear_usuario(
            "User1".to_string(),
            Some(TipoSuscripcion::Basic),
            MedioPago::Efectivo,
            f,
        );
        platform.crear_usuario(
            "User2".to_string(),
            Some(TipoSuscripcion::Classic),
            MedioPago::Cripto("wallet_addr".to_string()),
            f,
        );
        platform.crear_usuario(
            "User3".to_string(),
            Some(TipoSuscripcion::Basic),
            MedioPago::Cripto("wallet_addr2".to_string()),
            f,
        );
        platform.crear_usuario(
            "User4".to_string(),
            Some(TipoSuscripcion::Basic),
            MedioPago::Cripto("wallet_addr3".to_string()),
            f,
        );

        assert_eq!(platform.medio_pago_mas_utilizado_activas(), Some("Cripto"));
        assert_eq!(
            platform.suscripcion_mas_contratada_activas(),
            Some(TipoSuscripcion::Basic)
        );

        platform.upgrade_suscripcion(0);
        assert_eq!(
            platform.usuarios[0].suscripcion.as_ref().unwrap().tipo,
            TipoSuscripcion::Classic
        );

        platform.downgrade_suscripcion(2);
        assert!(platform.usuarios[2].suscripcion.is_none());

        assert_eq!(
            platform.medio_pago_mas_utilizado_historico(),
            Some("Cripto")
        );
        assert_eq!(
            platform.suscripcion_mas_contratada_historico(),
            Some(TipoSuscripcion::Basic)
        );
    }

    #[test]
    fn test_estacion_con_mas_suscripciones() {
        let mut platform = StreamingRust::new();

        platform.crear_usuario(
            "A".to_string(),
            Some(TipoSuscripcion::Basic),
            MedioPago::Efectivo,
            Fecha::new(5, 1, 2026),
        );
        platform.crear_usuario(
            "B".to_string(),
            Some(TipoSuscripcion::Basic),
            MedioPago::Efectivo,
            Fecha::new(10, 1, 2026),
        );
        platform.crear_usuario(
            "C".to_string(),
            Some(TipoSuscripcion::Basic),
            MedioPago::Efectivo,
            Fecha::new(15, 2, 2026),
        );

        platform.crear_usuario(
            "D".to_string(),
            Some(TipoSuscripcion::Basic),
            MedioPago::Efectivo,
            Fecha::new(1, 4, 2026),
        );

        let top = platform
            .estacion_con_mas_suscripciones()
            .expect("debería haber un resultado");
        assert_eq!(top.total_suscripciones_estacion, 3);
        assert_eq!(top.mes_top, 1);
        assert_eq!(top.cantidad_suscripciones_mes_top, 2);
    }

    #[test]
    fn test_determinar_estacion_segun_fecha() {
        assert_eq!(
            Estacion::determinar_estacion_segun_fecha(Fecha::new(21, 12, 2026)),
            Some(NombreEstacion::Verano)
        );
        assert_eq!(
            Estacion::determinar_estacion_segun_fecha(Fecha::new(20, 3, 2026)),
            Some(NombreEstacion::Verano)
        );
        assert_eq!(
            Estacion::determinar_estacion_segun_fecha(Fecha::new(21, 3, 2026)),
            Some(NombreEstacion::Otonio)
        );
        assert_eq!(
            Estacion::determinar_estacion_segun_fecha(Fecha::new(20, 6, 2026)),
            Some(NombreEstacion::Otonio)
        );
        assert_eq!(
            Estacion::determinar_estacion_segun_fecha(Fecha::new(21, 6, 2026)),
            Some(NombreEstacion::Invierno)
        );
        assert_eq!(
            Estacion::determinar_estacion_segun_fecha(Fecha::new(20, 9, 2026)),
            Some(NombreEstacion::Invierno)
        );
        assert_eq!(
            Estacion::determinar_estacion_segun_fecha(Fecha::new(21, 9, 2026)),
            Some(NombreEstacion::Primavera)
        );
        assert_eq!(
            Estacion::determinar_estacion_segun_fecha(Fecha::new(20, 12, 2026)),
            Some(NombreEstacion::Primavera)
        );
    }

    #[test]
    fn test_mes_invalido_no_pertenece_a_ninguna_estacion() {
        assert_eq!(
            Estacion::determinar_estacion_segun_fecha(Fecha::new(1, 13, 2026)),
            None
        );
    }

    #[test]
    fn test_sin_suscripciones_activas_devuelve_none() {
        let mut platform = StreamingRust::new();
        platform.crear_usuario(
            "A".to_string(),
            None,
            MedioPago::Efectivo,
            Fecha::new(1, 1, 2026),
        );
        platform.crear_usuario(
            "B".to_string(),
            Some(TipoSuscripcion::Basic),
            MedioPago::Efectivo,
            Fecha::new(5, 1, 2026),
        );
        platform.cancelar_suscripcion(1);

        assert!(platform.estacion_con_mas_suscripciones().is_none());
    }
}
