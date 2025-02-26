pub fn move_validator(mut movement_rule: String, possible_move: String) -> bool {
    bracket_dissolver(&mut movement_rule);
    return false;
}

fn bracket_dissolver(calculation: &mut str) {
    let value: f32 = 0.0;
    println!("{:?}", calculation);
    // TODO: this version only works when there are no ((xyz) + (xyz)) so please future me change that
    match calculation.find("(") {
        Some(x) => match calculation.rfind(")") {
            Some(y) => bracket_dissolver(&mut calculation[(x + 1)..(y)]),
            None => panic!(),
        },
        None => println!("end"),
    }
}
