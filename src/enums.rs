// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right")
    }
}

pub fn run() {
    let avatar_1 = Movement::Left;
    let avatar_2 = Movement::Up;
    let avatar_3 = Movement::Right;
    let avatar_4 = Movement::Down;

    move_avatar(avatar_1);
    move_avatar(avatar_2);
    move_avatar(avatar_3);
    move_avatar(avatar_4);
}
