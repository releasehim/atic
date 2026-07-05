use std::collections::HashMap;
use crate::fecha::Fecha;

#[derive(Debug, Clone, PartialEq)]
pub struct Blockchain {
    pub nombre: String,
    pub prefijo: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Criptomoneda {
    pub nombre: String,
    pub prefijo: String,
    pub blockchains: Vec<Blockchain>,
}

#[derive(Debug, Clone)]
pub struct Usuario {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub dni: String,
    pub identidad_validada: bool,
    pub balances: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TipoTransaccion {
    IngresoFiat(f64),
    CompraCripto {
        cripto: String,
        monto_cripto: f64,
        cotizacion: f64,
    },
    VentaCripto {
        cripto: String,
        monto_cripto: f64,
        cotizacion: f64,
    },
    RetiroCripto {
        blockchain: String,
        hash: String,
        cripto: String,
        monto: f64,
        cotizacion: f64,
    },
    RecepcionCripto {
        blockchain: String,
        cripto: String,
        monto: f64,
        cotizacion: f64,
    },
    RetiroFiat {
        monto: f64,
        medio: String,
    },
}

#[derive(Debug, Clone)]
pub struct Transaccion {
    pub fecha: Fecha,
    pub usuario_dni: String,
    pub tipo: TipoTransaccion,
}

pub struct XYZPlataforma {
    pub usuarios: HashMap<String, Usuario>, // dni -> Usuario
    pub transacciones: Vec<Transaccion>,
    pub criptomonedas: HashMap<String, Criptomoneda>, // prefijo -> Criptomoneda
    pub counter: u32,
}

impl XYZPlataforma {
    pub fn new() -> Self {
        Self {
            usuarios: HashMap::new(),
            transacciones: Vec::new(),
            criptomonedas: HashMap::new(),
            counter: 0,
        }
    }

    pub fn registrar_usuario(&mut self, u: Usuario) {
        self.usuarios.insert(u.dni.clone(), u);
    }

    pub fn registrar_criptomoneda(&mut self, c: Criptomoneda) {
        self.criptomonedas.insert(c.prefijo.clone(), c);
    }

    pub fn ingresar_dinero(&mut self, dni: &str, monto: f64, fecha: Fecha) -> bool {
        if let Some(u) = self.usuarios.get_mut(dni) {
            *u.balances.entry("fiat".to_string()).or_insert(0.0) += monto;
            self.transacciones.push(Transaccion {
                fecha,
                usuario_dni: dni.to_string(),
                tipo: TipoTransaccion::IngresoFiat(monto),
            });
            true
        } else {
            false
        }
    }

    pub fn comprar_criptomoneda(&mut self, dni: &str, monto_fiat: f64, cripto_prefijo: &str, cotizacion: f64, fecha: Fecha) -> bool {
        let user_valid = match self.usuarios.get(dni) {
            Some(u) => u.identidad_validada,
            None => return false,
        };
        if !user_valid {
            return false;
        }

        let u = self.usuarios.get_mut(dni).unwrap();
        let balance_fiat = *u.balances.get("fiat").unwrap_or(&0.0);
        if balance_fiat < monto_fiat {
            return false;
        }

        *u.balances.get_mut("fiat").unwrap() -= monto_fiat;
        let monto_cripto = monto_fiat / cotizacion;
        *u.balances.entry(cripto_prefijo.to_string()).or_insert(0.0) += monto_cripto;

        self.transacciones.push(Transaccion {
            fecha,
            usuario_dni: dni.to_string(),
            tipo: TipoTransaccion::CompraCripto {
                cripto: cripto_prefijo.to_string(),
                monto_cripto,
                cotizacion,
            },
        });
        true
    }

    pub fn vender_criptomoneda(&mut self, dni: &str, monto_cripto: f64, cripto_prefijo: &str, cotizacion: f64, fecha: Fecha) -> bool {
        let user_valid = match self.usuarios.get(dni) {
            Some(u) => u.identidad_validada,
            None => return false,
        };
        if !user_valid {
            return false;
        }

        let u = self.usuarios.get_mut(dni).unwrap();
        let balance_cripto = *u.balances.get(cripto_prefijo).unwrap_or(&0.0);
        if balance_cripto < monto_cripto {
            return false;
        }

        *u.balances.get_mut(cripto_prefijo).unwrap() -= monto_cripto;
        let monto_fiat = monto_cripto * cotizacion;
        *u.balances.entry("fiat".to_string()).or_insert(0.0) += monto_fiat;

        self.transacciones.push(Transaccion {
            fecha,
            usuario_dni: dni.to_string(),
            tipo: TipoTransaccion::VentaCripto {
                cripto: cripto_prefijo.to_string(),
                monto_cripto,
                cotizacion,
            },
        });
        true
    }

    pub fn retirar_cripto_a_blockchain(&mut self, dni: &str, monto_cripto: f64, cripto_prefijo: &str, blockchain_prefijo: &str, cotizacion: f64, fecha: Fecha) -> Option<String> {
        let user_valid = match self.usuarios.get(dni) {
            Some(u) => u.identidad_validada,
            None => return None,
        };
        if !user_valid {
            return None;
        }

        let u = self.usuarios.get_mut(dni).unwrap();
        let balance_cripto = *u.balances.get(cripto_prefijo).unwrap_or(&0.0);
        if balance_cripto < monto_cripto {
            return None;
        }

        *u.balances.get_mut(cripto_prefijo).unwrap() -= monto_cripto;

        self.counter += 1;
        let mut hash = blockchain_prefijo.to_string();
        hash += &self.counter.to_string();

        self.transacciones.push(Transaccion {
            fecha,
            usuario_dni: dni.to_string(),
            tipo: TipoTransaccion::RetiroCripto {
                blockchain: blockchain_prefijo.to_string(),
                hash: hash.clone(),
                cripto: cripto_prefijo.to_string(),
                monto: monto_cripto,
                cotizacion,
            },
        });
        Some(hash)
    }

    pub fn recibir_cripto_de_blockchain(&mut self, dni: &str, monto_cripto: f64, cripto_prefijo: &str, blockchain_prefijo: &str, cotizacion: f64, fecha: Fecha) -> bool {
        if let Some(u) = self.usuarios.get_mut(dni) {
            *u.balances.entry(cripto_prefijo.to_string()).or_insert(0.0) += monto_cripto;
            self.transacciones.push(Transaccion {
                fecha,
                usuario_dni: dni.to_string(),
                tipo: TipoTransaccion::RecepcionCripto {
                    blockchain: blockchain_prefijo.to_string(),
                    cripto: cripto_prefijo.to_string(),
                    monto: monto_cripto,
                    cotizacion,
                },
            });
            true
        } else {
            false
        }
    }

    pub fn retirar_fiat(&mut self, dni: &str, monto_fiat: f64, medio: String, fecha: Fecha) -> bool {
        if let Some(u) = self.usuarios.get_mut(dni) {
            let balance = *u.balances.get("fiat").unwrap_or(&0.0);
            if balance < monto_fiat {
                return false;
            }
            *u.balances.get_mut("fiat").unwrap() -= monto_fiat;
            self.transacciones.push(Transaccion {
                fecha,
                usuario_dni: dni.to_string(),
                tipo: TipoTransaccion::RetiroFiat { monto: monto_fiat, medio },
            });
            true
        } else {
            false
        }
    }

    pub fn cripto_mas_cantidad_ventas(&self) -> Option<String> {
        let mut counts = HashMap::new();
        for t in &self.transacciones {
            if let TipoTransaccion::VentaCripto { ref cripto, .. } = t.tipo {
                *counts.entry(cripto.clone()).or_insert(0) += 1;
            }
        }
        let mut max_c = None;
        let mut max_count = 0;
        for (c, count) in counts {
            if count > max_count {
                max_count = count;
                max_c = Some(c);
            }
        }
        max_c
    }

    pub fn cripto_mas_cantidad_compras(&self) -> Option<String> {
        let mut counts = HashMap::new();
        for t in &self.transacciones {
            if let TipoTransaccion::CompraCripto { ref cripto, .. } = t.tipo {
                *counts.entry(cripto.clone()).or_insert(0) += 1;
            }
        }
        let mut max_c = None;
        let mut max_count = 0;
        for (c, count) in counts {
            if count > max_count {
                max_count = count;
                max_c = Some(c);
            }
        }
        max_c
    }

    pub fn cripto_mas_volumen_ventas(&self) -> Option<String> {
        let mut volumes = HashMap::new();
        for t in &self.transacciones {
            if let TipoTransaccion::VentaCripto { ref cripto, monto_cripto, cotizacion } = t.tipo {
                // Volume in fiat: monto * cotizacion
                *volumes.entry(cripto.clone()).or_insert(0.0) += monto_cripto * cotizacion;
            }
        }
        let mut max_c = None;
        let mut max_vol = 0.0;
        for (c, vol) in volumes {
            if vol > max_vol {
                max_vol = vol;
                max_c = Some(c);
            }
        }
        max_c
    }

    pub fn cripto_mas_volumen_compras(&self) -> Option<String> {
        let mut volumes = HashMap::new();
        for t in &self.transacciones {
            if let TipoTransaccion::CompraCripto { ref cripto, monto_cripto, cotizacion } = t.tipo {
                // Volume in fiat: monto * cotizacion
                *volumes.entry(cripto.clone()).or_insert(0.0) += monto_cripto * cotizacion;
            }
        }
        let mut max_c = None;
        let mut max_vol = 0.0;
        for (c, vol) in volumes {
            if vol > max_vol {
                max_vol = vol;
                max_c = Some(c);
            }
        }
        max_c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xyz_plataforma() {
        let mut platform = XYZPlataforma::new();
        let f = Fecha::new(1, 1, 2026);

        let mut balances = HashMap::new();
        balances.insert("fiat".to_string(), 1000.0);
        let u = Usuario {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan@xyz.com".to_string(),
            dni: "1234".to_string(),
            identidad_validada: true,
            balances,
        };
        platform.registrar_usuario(u);

        // Buy BTC
        let buy_ok = platform.comprar_criptomoneda("1234", 600.0, "BTC", 50000.0, f);
        assert_eq!(buy_ok, true);

        // Balance check
        let user = platform.usuarios.get("1234").unwrap();
        assert_eq!(*user.balances.get("fiat").unwrap(), 400.0);
        assert_eq!(*user.balances.get("BTC").unwrap(), 600.0 / 50000.0);

        // Sell BTC
        let sell_ok = platform.vender_criptomoneda("1234", 0.005, "BTC", 60000.0, f);
        assert_eq!(sell_ok, true);

        // Withdraw remaining BTC
        let hash = platform.retirar_cripto_a_blockchain("1234", 0.005, "BTC", "ETH-Net", 50000.0, f);
        assert_eq!(hash, Some("ETH-Net1".to_string()));

        // Reports
        assert_eq!(platform.cripto_mas_cantidad_compras(), Some("BTC".to_string()));
        assert_eq!(platform.cripto_mas_cantidad_ventas(), Some("BTC".to_string()));
        assert_eq!(platform.cripto_mas_volumen_compras(), Some("BTC".to_string()));
        assert_eq!(platform.cripto_mas_volumen_ventas(), Some("BTC".to_string()));
    }
}
