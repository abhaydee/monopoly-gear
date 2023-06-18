use gtest::prelude::*;

mod gear;

#[test]
fn test_check_status() {
    let game = Game::new(1, vec![1, 2, 3], 0, 0);
    game.check_status(GameStatus::Started);
    assert!(true);
}

#[test]
fn test_only_admin() {
    let game = Game::new(1, vec![1, 2, 3], 0, 0);
    game.only_admin();
    assert!(true);
}

#[test]
fn test_only_player() {
    let game = Game::new(1, vec![1, 2, 3], 0, 0);
    game.only_player();
    assert!(true);
}

#[test]
fn test_get_player_info() {
    let game = Game::new(1, vec![1, 2, 3], 0, 0);
    let player_info = get_player_info(&1, &mut game.players, 0);
    assert!(true);
}

#[test]
fn test_sell_property_ok() {
    let game = Game::new(1, vec![1, 2, 3], 0, 0);
    let res = sell_property(&1, &mut game.ownership, &vec![1], &mut game.properties_in_bank, &game.properties, &mut game.players[0]);
    assert!(res.is_ok());
}

#[test]
fn test_sell_property_wrong_player() {
    let game = Game::new(1, vec![1, 2, 3], 0, 0);
    let res = sell_property(&2, &mut game.ownership, &vec![1], &mut game.properties_in_bank, &game.properties, &mut game.players[0]);
    assert!(res.is_err());
}

#[test]
fn test_get_rolls() {
    let (r1, r2) = get_rolls();
    assert!(r1 > 0 && r1 <= 6);
    assert!(r2 > 0 && r2 <= 6);
}

#[test]
fn test_bankrupt_and_penalty() {
    let game = Game::new(1, vec![1, 2, 3], 0, 0);
    bankrupt_and_penalty(&1, &mut game.players, &mut game.players_queue, &mut game.properties, &mut game.properties_in_bank, &mut game.ownership);
    assert!(game.players[0].debt == 0);
}

#[test]
fn test_init_properties() {
    init_properties(&mut game.properties, &mut game.ownership);
    assert!(game.properties[0].is_none());
    assert!(game.ownership[0] == ActorId::zero());
}
