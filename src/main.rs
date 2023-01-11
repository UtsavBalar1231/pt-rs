use ggez::{
    event,
    glam::*,
    graphics::{self, Canvas, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

const SCREEN_SIZE: (usize, usize) = (480, 480);

#[derive(Eq, PartialEq, Debug)]
struct Tank {
    pos: Position,
    next_pos: Option<Position>,
    last_known_position: Option<Position>,
    direction: Direction,
    last_known_direction: Option<Direction>,
    next_direction: Option<Direction>,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// We create a helper function that will allow us to easily get the inverse
    /// of a `Direction` which we can use later to check if the player should be
    /// able to move the snake in a certain direction.
    pub fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    /// We also create a helper function that will let us convert between a
    /// `ggez` `Keycode` and the `Direction` that it represents. Of course,
    /// not every keycode represents a direction, so we return `None` if this
    /// is the case.
    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }

    pub fn new_move(pos: Position, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position::new(pos.x, pos.y - 1),
            Direction::Down => Position::new(pos.x - 1, pos.y),
            Direction::Left => Position::new(pos.x - 1, pos.y - 1),
            Direction::Right => Position::new(pos.x + 1, pos.y + 1),
        }
    }
}

/// We implement the `From` trait, which in this case allows us to convert easily between
/// a GridPosition and a ggez `graphics::Rect` which fills that grid cell.
/// Now we can just call `.into()` on a `GridPosition` where we want a
/// `Rect` that represents that grid cell.
impl From<Position> for graphics::Rect {
    fn from(pos: Position) -> Self {
        graphics::Rect::new_i32((pos.x * 10) as i32, (pos.y * 10) as i32, 10, 10)
    }
}

struct GameState {
    tank: Tank,
    //    terrain: graphics::Mesh,
}

impl GameState {
    fn new() -> Self {
        let pos = Position::new(10, 10);
        let tank = Tank::new(pos, Direction::Right);

        /* let terrain = &mut graphics::MeshBuilder::new();

        terrain
        .rectangle(
        graphics::DrawMode::stroke(1.0),
        graphics::Rect::new(200.0, 200.0, 50.0, 50.0),
        graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )
        .expect("Create mesh failed!"); */

        GameState { tank }
    }
}

/// Now we implement EventHandler for GameState. This provides an interface
/// that ggez will call automatically when different events happen.
impl event::EventHandler<ggez::GameError> for GameState {
    /// Update will happen on every frame before it is drawn. This is where we update
    /// our game state to react to whatever is happening in the game world.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Rely on ggez's built-in timer for deciding when to update the game, and how many times.
        // If the update is early, there will be no cycles, otherwises, the logic will run once for each
        // frame fitting in the time since the last update.
        while ctx.time.check_update_time(8) {
            if self.tank.pos.x < SCREEN_SIZE.0 as isize && self.tank.pos.y < SCREEN_SIZE.1 as isize
            {
                self.tank.update(Position {
                    x: self.tank.pos.x + 10,
                    y: self.tank.pos.y + 10,
                });
            }
            // We check to see if the game is over. If not, we'll update. If so, we'll just do nothing.
            // Here we do the actual updating of our game world. First we tell the snake to update itself,
            // passing in a reference to our piece of food.
            // Next we check if the snake ate anything as it updated.
        }

        Ok(())
    }

    /// draw is where we should actually render the game's current state.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([1.0, 1.0, 1.0, 1.0]));

        self.tank.draw(&mut canvas);

        canvas.finish(ctx)?;

        ggez::timer::yield_now();

        Ok(())
    }

    /// key_down_event gets fired when a key gets pressed.
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        // Here we attempt to convert the Keycode into a Direction using the helper
        // we defined earlier.
        if let Some(dir) = input.keycode.and_then(Direction::from_keycode) {
            // If it succeeds, we check if a new direction has already been set
            // and make sure the new direction is different then `snake.dir`
            if let Some(last_known_dir) = self.tank.last_known_direction {
                if self.tank.direction != last_known_dir && dir.inverse() != self.tank.direction {
                    self.tank.next_direction = Some(dir);
                } else if dir.inverse() != last_known_dir {
                    // If no new direction has been set and the direction is not the inverse
                    // of the `last_update_dir`, then set the snake's new direction to be the
                    // direction the user pressed.
                    self.tank.direction = dir;
                }
            }
        }
        Ok(())
    }
}

impl Tank {
    pub fn new(pos: Position, direction: Direction) -> Self {
        Tank {
            pos,
            last_known_position: None,
            next_direction: None,
            direction,
            last_known_direction: None,
            next_pos: None,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        println!("Drawing tank at {:?}", self.pos);
        // draw tank
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.pos.into())
                .color([1.0, 0.5, 0.0, 1.0]),
        );

        // draw turret
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new().dest_rect(self.pos.into()),
        );
    }

    pub fn update(&mut self, new_pos: Position) {
        if let Some(ref mut last_known_pos) = &mut self.last_known_position {
            if last_known_pos == &mut self.pos && self.next_pos.is_some() {
                self.pos = self.next_pos.take().unwrap();
            } else {
                self.last_known_position = Some(self.pos);
                self.pos = new_pos;
            }
        }
        let new_pos = Position::new_move(new_pos, self.direction);

        self.next_pos = Some(new_pos);

        if let Some(ref mut last_known_dir) = &mut self.last_known_direction {
            if last_known_dir == &mut self.direction && self.next_direction.is_some() {
                self.direction = self.next_direction.take().unwrap();
            } else {
                self.last_known_direction = Some(self.direction);
            }
        }

        if let Some(ref mut next_dir) = &mut self.next_direction {
            if next_dir.inverse() != self.direction {
                self.direction = *next_dir;
            }
        }

        self.next_direction = None;

        self.pos = Position::new_move(self.pos, self.direction);
    }
}

pub fn main() -> GameResult {
    // Here we use a ContextBuilder to setup metadata about our game. First the title and author
    let (ctx, events_loop) = ggez::ContextBuilder::new("pockettanks", "Utsav Balar")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("Pocket Tanks!"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
        )
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build()?;

    // Next we create a new instance of our GameState struct, which implements EventHandler
    let state = GameState::new();

    event::run(ctx, events_loop, state)
}
