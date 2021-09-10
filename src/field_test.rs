use super::game::*;

#[test]
fn create_field() {
    let mut field = Field::new();
    assert_eq!(true, field.can_play(0));
    assert_eq!(Ok(()), field.one_play(&Player::X, 0));
    assert_ne!(Ok(()), field.one_play(&Player::X, 0));
    assert_eq!(Ok(()), field.one_play(&Player::O, 2));

    assert!(field.check_if_win(&Player::None));
    assert!(!field.check_if_win(&Player::X));
}

#[test] 
fn player_win_line() {
    let mut field = Field::new();
    let player = Player::X;
    
    for point in 3..6 {
        assert_eq!(Ok(()), field.one_play(&player, point));
    }

    assert!(field.check_if_win(&player));
    assert!(!field.check_if_win(&Player::O));
}

#[test] 
fn player_win_column() {
    let mut field = Field::new();
    let player = Player::X;
    
    for index in 0..3 {
        let point = 2 + 3 * index;
        assert_eq!(Ok(()), field.one_play(&player, point));
    }

    assert!(field.check_if_win(&player));
    assert!(!field.check_if_win(&Player::O)); 
}

#[test] 
fn player_win_diag_one() {
    let mut field = Field::new();
    let player = Player::X;
    
    for index in 0..3 {
        let point = 4 * index;
        assert_eq!(Ok(()), field.one_play(&player, point));
    }

    assert!(field.check_if_win(&player));
    assert!(!field.check_if_win(&Player::O)); 
}

#[test] 
fn player_win_diag_two() {
    let mut field = Field::new();
    let player = Player::X;
    
    for index in 0..3 {
        let point = 2 + 2 * index;
        assert_eq!(Ok(()), field.one_play(&player, point));
    }

    assert!(field.check_if_win(&player));
    assert!(!field.check_if_win(&Player::O)); 
}