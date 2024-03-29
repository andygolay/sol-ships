use crate::*;

pub fn leave_game(
    client: &Client,
    user: Pubkey,
    game: Pubkey,
    color: sol_ships::Color,
) -> ClientResult<()> {
    let leave_game_ix = Instruction {
        program_id: sol_ships::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(user, false),
            AccountMeta::new(game, false),
        ],
        data: sol_ships::instruction::LeaveGame {}.data(),
    };

    send_and_confirm_tx(
        &client,
        [leave_game_ix].to_vec(),
        None,
        "leave_game".to_string(),
    )?;

    Ok(())
}
