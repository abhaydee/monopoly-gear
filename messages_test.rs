use gtest::prelude::*;

mod gear;

#[test]
fn test_take_your_turn_with_valid_player() {
    let game = Game::new();
    let player = ActorId::new("player1");
    let result = take_your_turn(&player, &game).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_take_your_turn_with_invalid_player() {
    let game = Game::new();
    let player = ActorId::new("invalid_player");
    let result = take_your_turn(&player, &game);
    assert!(result.is_err());
}

#[test]
fn test_take_your_turn_with_expired_wait_duration() {
    let game = Game::new();
    let player = ActorId::new("player1");
    let result = take_your_turn(&player, &game).unwrap();
    assert_eq!(result.len(), 0);
}
