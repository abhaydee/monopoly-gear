use gtest::prelude::*;

mod gear;

#[test]
fn test_pay_fine_in_jail() {
    let monopoly_id = msg::source();
    let message: YourTurn = msg::load().expect("Unable to decode struct`YourTurn`");
    let (_, mut player_info) = message
        .players
        .iter()
        .find(|(player, _player_info)| player == &exec::program_id())
        .expect("Can't find my address")
        .clone();

    // Test case 1: Player is in jail and has less balance than FINE.
    // Assert that the player is able to pay the fine and get out of jail.
    player_info.in_jail = true;
    player_info.balance = FINE - 1;

    let reply: GameEvent = msg::send_for_reply_as(
        monopoly_id,
        GameAction::ThrowRoll {
            pay_fine: true,
            properties_for_sale: None,
        },
        0,
    )
    .expect("Error in sending a message `GameAction::ThrowRoll`")
    .await
    .expect("Unable to decode `GameEvent");

    assert_eq!(reply, GameEvent::Jail { in_jail: false, position: 0 });

    // Test case 2: Player is in jail and has enough balance to pay the fine.
    // Assert that the player is not able to pay the fine and remains in jail.
    player_info.balance = FINE + 1;

    let reply: GameEvent = msg::send_for_reply_as(
        monopoly_id,
        GameAction::ThrowRoll {
            pay_fine: true,
            properties_for_sale: None,
        },
        0,
    )
    .expect("Error in sending a message `GameAction::ThrowRoll`")
    .await
    .expect("Unable to decode `GameEvent");

    assert_eq!(reply, GameEvent::Jail { in_jail: true, position: 0 });
}
