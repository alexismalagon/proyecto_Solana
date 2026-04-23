# 🗳️ ElectionChain Solana

![banner](./images/banner-electionchain.jpg)

Sistema de auditoría electoral desarrollado como **Solana Program** utilizando **Rust** y el framework **Anchor**.  

Este proyecto implementa un sistema **CRUD** para registrar, auditar y gestionar resultados electorales por casilla directamente en blockchain, garantizando:

- 🔑 Integridad mediante Program Derived Addresses (PDAs)  
- 🔒 Seguridad basada en firmas autorizadas  
- ⚡ Transparencia y trazabilidad *On-Chain*  

---

## 📚 Descripción

**ElectionChain Solana** simula un sistema descentralizado de auditoría electoral donde una autoridad puede:

- Inicializar una elección  
- Registrar resultados de casillas  
- Editar resultados (solo en recuentos oficiales)  
- Eliminar casillas anuladas  
- Consultar el cómputo total en blockchain  

---

## 🧠 Arquitectura y Estructuras de Datos

En Solana es obligatorio definir el tamaño de los datos para calcular la renta (*rent*).

### 📦 PDA Principal: `AuditoriaEleccion`

Cuenta raíz que almacena toda la información de la elección.

```rust
#[account]
#[derive(InitSpace)]
pub struct AuditoriaEleccion {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_eleccion: String,
    #[max_len(20)]
    pub casillas: Vec<CasillaElectoral>,
}
```

---

### 🧩 Estructura Interna: `CasillaElectoral`

Cada casilla contiene:

- `id_casilla (String)` → identificador único (máx. 20 caracteres)  
- `votos_candidato_a (u32)` → votos del candidato A  
- `votos_candidato_b (u32)` → votos del candidato B  

```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct CasillaElectoral {
    #[max_len(20)]
    pub id_casilla: String,
    pub votos_candidato_a: u32,
    pub votos_candidato_b: u32,
}
```

---

## 🔒 Seguridad

El contrato valida que solo una entidad autorizada pueda modificar los datos:

```rust
require!(
    eleccion.owner == ctx.accounts.owner.key(),
    Errores::NoEresElAutorizado
);
```

✔ Previene manipulación de resultados  
✔ Garantiza control por autoridad electoral  

---

## ⚙️ Funcionalidad (CRUD)

### 🟢 Inicializar Elección

Crea la cuenta principal usando:

```rust
[b"eleccion", owner.key().as_ref()]
```

Inicializa:
- Owner  
- Nombre de la elección  
- Lista vacía de casillas  

---

### ➕ Registrar Casilla

- Recibe:
  - ID de casilla  
  - votos candidato A  
  - votos candidato B  
- Inserta en el vector con `.push()`  
- No realiza cálculos automáticos (datos directos del acta)  

---

### ✏️ Editar Casilla

- Busca por `id_casilla`  
- Actualiza votos únicamente en caso de recuento oficial  

---

### ❌ Eliminar Casilla

```rust
.iter().position(|c| c.id_casilla == id_casilla)
```

- Si existe → `.remove(index)`  
- Si no → error `CasillaNoEncontrada`  

---

### 📖 Ver Resultados

```rust
msg!("Cómputo Transparente: {:#?}", eleccion.casillas);
```

Muestra el estado actual del conteo en logs *On-Chain*

---

## 🧪 Despliegue en Solana Playground

1. Copia el código en `lib.rs`  
2. Ejecuta:

```bash
cargo clean
```

3. Haz clic en **Build**  
4. Haz clic en **Deploy (Devnet)**  

---

## 🧑‍💻 Pruebas

Puedes interactuar con el contrato usando:

- Pestaña **Test** del Playground  
- Scripts en TypeScript:

```ts
pg.program.methods...
```

Parámetros:
- `id_casilla: String`  
- `votos_a: u32`  
- `votos_b: u32`  

---

## ⚠️ Manejo de Errores

El programa define errores personalizados:

```rust
#[error_code]
pub enum Errores {
    #[msg("Violación de seguridad: Llave no autorizada por el tribunal electoral.")]
    NoEresElAutorizado,
    #[msg("Error de auditoría: El ID de la casilla no existe en el cómputo.")]
    CasillaNoEncontrada,
}
```

---

## 📌 Conclusión

Este proyecto demuestra:

- Transparencia en sistemas electorales usando blockchain  
- Integridad de datos mediante control de acceso  
- Uso eficiente de estructuras dinámicas en Solana  
- Implementación de auditoría descentralizada  

---

## 🚀 Próximos pasos

- Integrar frontend para visualización de resultados  
- Añadir múltiples candidatos  
- Implementar firmas múltiples (multi-sig)  
- Crear dashboards de análisis electoral  

---
