use gtest::prelude::*;

mod gear;

#[test]
fn test_start_registration() {
    let mut game = GameState::default();
    game.admin = ActorId::new("admin");

    let action = GameAction::StartRegistration;
    let event = game.handle(action);

    assert_eq!(event, GameEvent::StartRegistration);
    assert_eq!(game.game_status, GameStatus::Registration);
}

#[test]
fn test_register_player() {
    let mut game = GameState::default();
    game.admin = ActorId::new("admin");

    let action = GameAction::Register {
        player: ActorId::new("player"),
    };
    let event = game.handle(action);

    assert_eq!(event, GameEvent::Registered);
    assert_eq!(game.players.len(), 1);
}

#[test]
fn test_throw_roll() {
    let mut game = GameState::default();
    game.admin = ActorId::new("admin");
    game.players.push((ActorId::new("player"), PlayerInfo::default()));

    let action = GameAction::ThrowRoll {
        pay_fine: false,
        properties_for_sale: None,
    };
    let event = game.handle(action);

    assert_eq!(event, GameEvent::Step {
        players: vec![(ActorId::new("player"), PlayerInfo {
            position: 1,
            ..PlayerInfo::default()
        })],
        properties: vec![],
        current_player: ActorId::new("player"),
        ownership: vec![],
        current_step: 1,
    });
}

#[test]
fn test_add_gear() {
    let mut game = GameState::default();
    game.admin = ActorId::new("admin");
    game.players.push((ActorId::new("player"), PlayerInfo::default()));

    let action = GameAction::AddGear {
        properties_for_sale: None,
    };
    let event = game.handle(action);

    assert_eq!(event, GameEvent::StrategicSuccess);
}

#[test]
fn test_upgrade() {
    let mut game = GameState::default();
    game.admin = ActorId::new("admin");
    game.players.push((ActorId::new("player"), PlayerInfo::default()));

    let action = GameAction::Upgrade {
        properties_for_sale: None,
    };
    let event = game.handle(action);

    assert_eq!(event, GameEvent::StrategicSuccess);
}
