use std::collections::HashMap;
use crate::fecha::Fecha;

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

pub struct StreamingRust {
    pub usuarios: Vec<Usuario>,
    pub historial: Vec<HistorialContrato>,
}

impl StreamingRust {
    pub fn new() -> Self {
        Self {
            usuarios: Vec::new(),
            historial: Vec::new(),
        }
    }

    pub fn crear_usuario(&mut self, nombre: String, tipo_sub: Option<TipoSuscripcion>, medio: MedioPago, fecha: Fecha) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_rust() {
        let mut platform = StreamingRust::new();
        let f = Fecha::new(1, 1, 2026);

        platform.crear_usuario("User1".to_string(), Some(TipoSuscripcion::Basic), MedioPago::Efectivo, f);
        platform.crear_usuario("User2".to_string(), Some(TipoSuscripcion::Classic), MedioPago::Cripto("wallet_addr".to_string()), f);
        platform.crear_usuario("User3".to_string(), Some(TipoSuscripcion::Basic), MedioPago::Cripto("wallet_addr2".to_string()), f);
        platform.crear_usuario("User4".to_string(), Some(TipoSuscripcion::Basic), MedioPago::Cripto("wallet_addr3".to_string()), f);

        assert_eq!(platform.medio_pago_mas_utilizado_activas(), Some("Cripto"));
        assert_eq!(platform.suscripcion_mas_contratada_activas(), Some(TipoSuscripcion::Basic));

        platform.upgrade_suscripcion(0); // User1 Basic -> Classic
        assert_eq!(platform.usuarios[0].suscripcion.as_ref().unwrap().tipo, TipoSuscripcion::Classic);

        platform.downgrade_suscripcion(2); // User3 Basic -> Cancelled (None)
        assert!(platform.usuarios[2].suscripcion.is_none());

        assert_eq!(platform.medio_pago_mas_utilizado_historico(), Some("Cripto"));
        assert_eq!(platform.suscripcion_mas_contratada_historico(), Some(TipoSuscripcion::Basic));
    }
}
