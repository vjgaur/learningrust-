pub fn run  (){

   //enums are type which has a few definite values
 let avatar1 = Movement::Left;
 let avatar2 = Movement::Up;
 let avatar3 = Movement::Down;
 let avatar4 = Movement::Right;

 move_avatar(avatar1);
 move_avatar(avatar2);
 move_avatar(avatar3);
 move_avatar(avatar4);

}
enum Movement {
    //varaiance
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m:Movement)
{
    //Perform acton depending on info
    match m{
        
        Movement::Up => println!("Avatar moved up"),
        Movement::Down => println!("Avatar moved down"),
        Movement::Left => println!("Avatar moved left"),
        Movement::Right => println!("Avatar moved right")
    }

}