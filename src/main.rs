extern crate piston_window;

use piston_window::*;
use piston_window::Button;
use piston_window::Key;

fn set_collectables(collectables: &mut [[f64; 3]; 3]) -> [[f64; 3]; 3] 
{
    collectables[0][0] = 600.0;
    collectables[0][1] = 400.0;
	collectables[1][0] = 300.0;
    collectables[1][1] = 200.0;
    collectables[2][0] = 600.0;
    collectables[2][1] = 150.0;

    *collectables
}

fn main()
{
    let mut window: PistonWindow = WindowSettings::new("mini so_long", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();
	
    let mut x = 50.0;
	let mut y = 50.0;
	let total_collectables = 3;
	let mut count_steps = 0;
	let mut collected = 0;
	let mut array: [[f64; 3]; 3] = [[0.0; 3]; 3];
	set_collectables(&mut array);

	while let Some(event) = window.next() 
	{
		if let Some(Button::Keyboard(Key)) = event.press_args()
		{
			match Key {
				Key::D => {
					if (x + 50.0) / 50.0 != 15.0
						{
							if (x + 50.0) / 50.0 == 400.0 / 50.0 && (y / 50.0 == 2.0) && collected == total_collectables
								{println!("*-*-*-*-*- You win! *-*-*-*-*-"); std::process::exit(0);}
							else if (x + 50.0) == array[0][0] && y == array[0][1]
								{array[0][0] = -100.00; collected += 1;}
							else if x + 50.0 == array[1][0] && y == array[1][1]
								{array[1][0] = -100.00; collected += 1;}
							else if x + 50.0 == array[2][0] && y == array[2][1]
								{array[2][0] = -100.00; collected += 1;}
							x += 50.0;
							count_steps += 1;
							println!("Steps = {count_steps}");
						}
				},
				Key::S => {
					if (y + 50.0) / 50.0 != 11.0
						{
							if x / 50.0 == 400.0 / 50.0 && ((y + 50.0) / 50.0 == 2.0) && collected == total_collectables
								{println!("*-*-*-*-*- You win! *-*-*-*-*-"); std::process::exit(0);}
							else if x == array[0][0] && y + 50.0 == array[0][1]
								{array[0][0] = -100.00; collected += 1;}
							else if x == array[1][0] && y + 50.0 == array[1][1]
								{array[1][0] = -100.00; collected += 1;}
							else if x == array[2][0] && y + 50.0 == array[2][1]
								{array[2][0] = -100.00; collected += 1;}
							y += 50.0;
							count_steps += 1;
							println!("Steps = {count_steps}");
						}
				},
				Key::W => {
					if (y - 50.0) / 50.0 != 0.0
						{
							if x / 50.0 == 400.0 / 50.0 && ((y - 50.0 ) / 50.0 == 2.0) && collected == total_collectables
								{println!("*-*-*-*-*- You win! *-*-*-*-*-"); std::process::exit(0);}
							else if x == array[0][0] && y - 50.0 == array[0][1]
								{array[0][0] = -100.00; collected += 1;}
							else if x == array[1][0] && y - 50.0 == array[1][1]
								{array[1][0] = -100.00; collected += 1;}
							else if x == array[2][0] && y - 50.0 == array[2][1]
								{array[2][0] = -100.00; collected += 1;} 
							y -= 50.0;
							count_steps += 1;
							println!("Steps = {count_steps}");
						}
				},
				Key::A => {
					if (x - 50.0) / 50.0 != 0.0
						{
							if (x - 50.0) / 50.0 == 400.0 / 50.0 && (y / 50.0 == 2.0) && collected == total_collectables
								{println!("*-*-*-*-*- You win! *-*-*-*-*-"); std::process::exit(0);}
							else if x - 50.0 == array[0][0] && y == array[0][1]
								{array[0][0] = -100.00; collected += 1;}
							else if x - 50.0 == array[1][0] && y == array[1][1]
								{array[1][0] = -100.00; collected += 1;}
							else if x - 50.0 == array[2][0] && y == array[2][1]
								{array[2][0] = -100.00; collected += 1;}
							x -= 50.0;
							count_steps += 1;
							println!("Steps = {count_steps}");
						}
				}
				_ => {}
			}
		}

       	if let Some(_) = event.render_args() {
           	window.draw_2d(&event, |_context, graphics, _device|
				{
					clear([0.0, 0.0, 0.0, 1.0], graphics);
					rectangle([0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 800.0, 50.0], _context.transform, graphics);
					rectangle([0.0, 1.0, 0.0, 1.0], [0.0, 550.0, 800.0, 50.0], _context.transform, graphics);
					rectangle([0.0, 1.0, 0.0, 1.0], [0.0, 50.0, 50.0, 500.0], _context.transform, graphics);
					rectangle([0.0, 1.0, 0.0, 1.0], [750.0, 50.0, 50.0, 500.0], _context.transform, graphics);
					rectangle([1.0, 1.0, 1.0, 1.0], [400.0, 100.0, 50.0, 50.0], _context.transform, graphics);
					if array[0][0] > 0.0
						{rectangle([1.0, 1.0, 0.0, 1.0], [array[0][0], array[0][1], 50.0, 50.0], _context.transform, graphics);}
					if array[1][0] > 0.0
						{rectangle([1.0, 1.0, 0.0, 1.0], [array[1][0], array[1][1], 50.0, 50.0], _context.transform, graphics);}
					if array[2][0] > 0.0
						{rectangle([1.0, 1.0, 0.0, 1.0], [array[2][0], array[2][1], 50.0, 50.0], _context.transform, graphics);}
					rectangle([1.0, 0.0, 0.0, 1.0], [x, y, 50.0, 50.0], _context.transform, graphics);
       		});
    	}
    }
}