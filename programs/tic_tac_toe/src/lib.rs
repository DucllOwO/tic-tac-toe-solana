use anchor_lang::prelude::*;
use instructions::*;
use states::game::Tile;

pub mod errors;
pub mod instructions;
pub mod states;

declare_id!("61ZTuybsFkoAEWBvS2S5r6CgQ2xoG93x65CYm3CqDteY");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        instructions::setup_game::setup_game(ctx, player_two)
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        instructions::play::play(ctx, tile)
    }
}
