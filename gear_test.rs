use gtest::prelude::*;

mod gear;

#[test]
fn test_transfer() {
    let gear = gear::Gear::new();
    gear.transfer(100, "0x1234567890abcdef");
    assert_eq!(gear.balance(), 0);