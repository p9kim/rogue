extern crate tcod;

use tcod::console::*;
use tcod::colors;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 60;

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    tcod::system::set_fps(LIMIT_FPS);

    let mut playerPos_x = SCREEN_WIDTH / 2;
    let mut playerPos_y = SCREEN_HEIGHT / 2;

    con.set_default_foreground(colors::WHITE);
    con.put_char(playerPos_x, playerPos_y, '@', BackgroundFlag::None);

    blit(&mut con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let npc = Object::new((SCREEN_WIDTH / 2) - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
    let mut objects = [player, npc];

    while !root.window_closed() 
    {
        root.set_default_foreground(colors::WHITE);
        root.put_char(playerPos_x, playerPos_y, '@', BackgroundFlag::None);
        root.flush();

        root.put_char(playerPos_x, playerPos_y, ' ', BackgroundFlag::None);
        let exit = handle_keys(&mut root, &mut playerPos_x, &mut playerPos_y);
        if exit 
        {
            break
        }
    }
}

fn handle_keys(root: &mut Root, playerPos_x: &mut i32, playerPos_y: &mut i32) -> bool
{
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key 
    {
        Key { code: Enter, alt: true, .. } => 
        {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        },
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } => *playerPos_y -= 1,
        Key { code: Down, .. } => *playerPos_y += 1,
        Key { code: Left, .. } => *playerPos_x -= 1,
        Key { code: Right, .. } => *playerPos_x += 1,
        
        _ => {},
    }
    false
}

struct Object 
{
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object 
{
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self 
    {
        Object
        {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32)
    {
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&mut self, dx: i32, dy: i32)
    {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console)
    {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

