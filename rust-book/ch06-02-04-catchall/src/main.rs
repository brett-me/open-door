// ================================================================================================
// CH 06-01-04 CATCH-ALL PATTERNS
// ================================================================================================
// some values can take special actions while other values take one default action
// catch-all needs to be listed last

fn main() {}

// catch-all calls function
fn action_move(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

// _ as a placeholder for other
fn action_reroll(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

// default action results no action (empty tuple in this case)
fn action_none(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
