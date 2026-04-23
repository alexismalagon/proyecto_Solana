use anchor_lang::prelude::*;

// ID del programa (Se genera al hacer build en SolPG)
declare_id!("7Ceg2K7EAVhhHJgSKDHL3CKjnF1n5Ljpjrp6iNX6v3wq");

#[program]
pub mod electionchain_solana {
    use super::*;

    // 1. CREATE (PDA): Inicializa el centro de control electoral
    pub fn inicializar_eleccion(ctx: Context<CrearEleccion>, nombre_eleccion: String) -> Result<()> {
        let eleccion = &mut ctx.accounts.eleccion;
        eleccion.owner = ctx.accounts.owner.key();
        eleccion.nombre_eleccion = nombre_eleccion;
        eleccion.casillas = Vec::new();
        
        msg!("Auditoría inicializada para la elección: {}", eleccion.nombre_eleccion);
        Ok(())
    }

    // 2. CREATE (Dato): Registra los resultados de una casilla sin valores inferidos
    pub fn registrar_casilla(
        ctx: Context<GestionarCasilla>, 
        id_casilla: String, 
        votos_a: u32, 
        votos_b: u32
    ) -> Result<()> {
        let eleccion = &mut ctx.accounts.eleccion;
        require!(eleccion.owner == ctx.accounts.owner.key(), Errores::NoEresElAutorizado);

        let nueva_casilla = CasillaElectoral {
            id_casilla: id_casilla.clone(),
            votos_candidato_a: votos_a,
            votos_candidato_b: votos_b,
        };

        eleccion.casillas.push(nueva_casilla);
        msg!("Acta de la casilla '{}' asegurada en la blockchain.", id_casilla);
        Ok(())
    }

    // 3. UPDATE: Modifica los votos (Solo en caso de recuento oficial ordenado por tribunal)
    pub fn editar_casilla(
        ctx: Context<GestionarCasilla>, 
        id_casilla: String, 
        nuevos_votos_a: u32, 
        nuevos_votos_b: u32
    ) -> Result<()> {
        let eleccion = &mut ctx.accounts.eleccion;
        require!(eleccion.owner == ctx.accounts.owner.key(), Errores::NoEresElAutorizado);

        let lista = &mut eleccion.casillas;
        for i in 0..lista.len() {
            if lista[i].id_casilla == id_casilla {
                lista[i].votos_candidato_a = nuevos_votos_a;
                lista[i].votos_candidato_b = nuevos_votos_b;
                msg!("Recuento oficial aplicado a la casilla '{}'.", id_casilla);
                return Ok(());
            }
        }
        Err(Errores::CasillaNoEncontrada.into())
    }

    // 4. DELETE: Elimina la casilla (Ej. si el tribunal electoral anula los votos)
    pub fn eliminar_casilla(ctx: Context<GestionarCasilla>, id_casilla: String) -> Result<()> {
        let eleccion = &mut ctx.accounts.eleccion;
        require!(eleccion.owner == ctx.accounts.owner.key(), Errores::NoEresElAutorizado);

        let lista = &mut eleccion.casillas;
        let index = lista.iter().position(|c| c.id_casilla == id_casilla);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Casilla '{}' ANULADA y retirada del conteo oficial.", id_casilla);
            Ok(())
        } else {
            Err(Errores::CasillaNoEncontrada.into())
        }
    }

    // 5. READ: Emite el estado actual del cómputo
    pub fn ver_resultados(ctx: Context<GestionarCasilla>) -> Result<()> {
        msg!("Elección: {}", ctx.accounts.eleccion.nombre_eleccion);
        msg!("Cómputo Transparente: {:#?}", ctx.accounts.eleccion.casillas);
        Ok(())
    }
}

// --- ESTADO DEL PROGRAMA ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct CasillaElectoral {
    #[max_len(20)]
    pub id_casilla: String,
    pub votos_candidato_a: u32,
    pub votos_candidato_b: u32,
}

#[account]
#[derive(InitSpace)]
pub struct AuditoriaEleccion {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_eleccion: String,
    #[max_len(20)] // Capacidad para auditar 20 casillas por cuenta
    pub casillas: Vec<CasillaElectoral>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct CrearEleccion<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + AuditoriaEleccion::INIT_SPACE,
        seeds = [b"eleccion", owner.key().as_ref()],
        bump
    )]
    pub eleccion: Account<'info, AuditoriaEleccion>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarCasilla<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub eleccion: Account<'info, AuditoriaEleccion>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("Violación de seguridad: Llave no autorizada por el tribunal electoral.")]
    NoEresElAutorizado,
    #[msg("Error de auditoría: El ID de la casilla no existe en el cómputo.")]
    CasillaNoEncontrada,
}
