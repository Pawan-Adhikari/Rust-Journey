enum direction
{
    North,
    South,
    East,
    West,
    Unknown(String),
}

fn move_player(current_dir : direction)
{
    match current_dir
    {
        direction::North => println!("Moved North!"),
        direction::South => println!("Moved South!"),
        direction::East => println!("Moved East!"),
        direction::West => println!("Moved West!"),
        direction::Unknown(String) => println!("Not a valid direction"),
    }
}

fn main()
{

    move_player("jj");
}