use super::game::Field;
use super::game::GameStatus;
use super::game::Player;


#[test]
fn create_field() {
    let mut field = Field::new();
    assert_eq!(true, field.can_play(0));
    assert_eq!(Ok(()), field.one_play(&Player::X, 0));
    assert_ne!(Ok(()), field.one_play(&Player::X, 0));
    assert_eq!(Ok(()), field.one_play(&Player::O, 2));

    assert_eq!(GameStatus::Play, field.game_progress());
}

#[test] 
fn player_win_line() {
    let mut field = Field::new();
    let player = Player::X;
    
    for point in 3..6 {
        assert_eq!(Ok(()), field.one_play(&player, point));
    }

    assert_eq!(GameStatus::Win(Player::X), field.game_progress());
}

#[test] 
fn player_win_column() {
    let mut field = Field::new();
    let player = Player::X;
    
    for index in 0..3 {
        let point = 2 + 3 * index;
        assert_eq!(Ok(()), field.one_play(&player, point));
    }

    assert_eq!(GameStatus::Win(Player::X), field.game_progress());

}

#[test] 
fn player_win_diag_one() {
    let mut field = Field::new();
    let player = Player::X;
    
    for index in 0..3 {
        let point = 4 * index;
        assert_eq!(Ok(()), field.one_play(&player, point));
    }

    assert_eq!(GameStatus::Win(Player::X), field.game_progress());

}

#[test] 
fn player_win_diag_two() {
    let mut field = Field::new();
    let player = Player::X;
    
    for index in 0..3 {
        let point = 2 + 2 * index;
        assert_eq!(Ok(()), field.one_play(&player, point));
    }

    assert_eq!(GameStatus::Win(Player::X), field.game_progress());

}