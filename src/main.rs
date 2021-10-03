use bracket_lib::prelude::*;
use rand::Rng;
use std::{
    any::Any,
    io::{self, Read},
};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const MS_PER_TICK: i32 = 60;
const GRAVITY: i32 = 1;
const PLAYER_SPEED_X: i32 = 1;
const PLAYER_FLAP_Y: i32 = 4;
const PLAYER_START_X: i32 = 5;
const PLAYER_START_Y: i32 = 25;
const MIN_SPACE_BETWEEN_OBSTACLE: i32 = 20;
const MAX_SPACE_BETWEEN_OBSTACLE: i32 = 40;
const MIN_OBSTACLE_GAP_Y: i32 = 10;
const MAX_OBSTACLE_GAP_Y: i32 = SCREEN_HEIGHT - 10;
const MIN_OBSTACLE_GAP: i32 = 7;
const MAX_OBSTACLE_GAP: i32 = 13;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Flappy")
        .build()?;

    main_loop(context, State::new())
}

struct State {
    mode: GameMode,
    player: Player,
    accumulator_ms: f32,
    obstacles: Vec<Obstacle>,
    score: i32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.playing(ctx),
            GameMode::End => self.end(ctx),
        }
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}
impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: { Player::new(PLAYER_START_X, PLAYER_START_Y) },
            accumulator_ms: 0.0,
            obstacles: vec![],
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Hello Flappy");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        match ctx.key {
            Some(VirtualKeyCode::P) => {
                self.restart();
            }
            Some(VirtualKeyCode::Q) => {
                ctx.quitting = true;
            }
            _ => {}
        }
    }

    fn playing(&mut self, ctx: &mut BTerm) {
        // Get user input
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        // Physics update
        self.accumulator_ms += ctx.frame_time_ms;
        if self.accumulator_ms as i32 > MS_PER_TICK {
            self.accumulator_ms = 0.0;
            self.fixed_update();
        }

        // Render
        self.render(ctx);
    }

    fn end(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over");
        ctx.print_centered(8, &format!("Score: {}", self.score));
        ctx.print_centered(11, "Press any key to return to main menu");
        match ctx.key {
            Some(_) => {
                self.mode = GameMode::Menu;
            }
            _ => {}
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(5, 25);
        self.accumulator_ms = 0.0;
        self.score = 0;

        self.obstacles.clear();
        let mut next_pos = 0;
        assert!(MIN_SPACE_BETWEEN_OBSTACLE > 0);
        while next_pos < SCREEN_WIDTH * 2 {
            let space = rand::thread_rng()
                .gen_range(MIN_SPACE_BETWEEN_OBSTACLE..MAX_SPACE_BETWEEN_OBSTACLE + 1);
            next_pos += space;

            let obstacle = Obstacle::new(next_pos);
            self.obstacles.push(obstacle);
        }
    }

    fn fixed_update(&mut self) {
        // Update the player
        self.player.fixed_update();
        
        // Update the obstacles
        for o in self.obstacles.iter_mut() {
            o.fixed_update();
        }

        // Check if the first obstacle left the left side of the screen.
        // If so, delete it and create a new one on the right.
        let first_obs = self.obstacles.first().unwrap();
        if first_obs.x < 0 {
            // Remove the last obstacle.
            self.obstacles.remove(0);

            // Create & add a new obstacle
            let last_obs = self.obstacles.last().unwrap();
            let mut next_pos = last_obs.x;
            let space = rand::thread_rng()
                .gen_range(MIN_SPACE_BETWEEN_OBSTACLE..MAX_SPACE_BETWEEN_OBSTACLE + 1);
            next_pos += space;
            let obstacle = Obstacle::new(next_pos);
            self.obstacles.push(obstacle);
        }

        // Note: if PLAYER_SPEED_X > 1, the player will be able
        // to go through obstacles and also not increment score.
        let player_x = self.player.x;
        for o in self.obstacles.iter_mut() {
            if o.x == player_x {
                let player_y = self.player.y;
                let gap_y = o.gap_y;
                let gap_size = o.gap_size;

                if gap_y < player_y && player_y <= gap_y + gap_size {
                    self.score += 1;
                } else {
                    self.mode = GameMode::End;
                }
            }
        }
        if self.player.y < 0 || self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    fn render(&self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.player.render(ctx);
        for o in self.obstacles.iter() {
            o.render(ctx);
        }
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print_centered(8, &format!("Score: {}", self.score));
    }
}

struct Player {
    x: i32,
    y: i32,
    speed_y: i32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player { x, y, speed_y: 0 }
    }

    fn fixed_update(&mut self) {
        self.speed_y += GRAVITY;
        if self.speed_y > 2 {
            self.speed_y -= 1;
        }
        //self.x += PLAYER_SPEED_X;
        self.y += self.speed_y;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.speed_y = -PLAYER_FLAP_Y;
    }

    fn render(&self, ctx: &mut BTerm) {
        ctx.set(self.x as i32, self.y as i32, YELLOW, BLACK, to_cp437('@'));
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    gap_size: i32,
}
impl Obstacle {
    fn fixed_update(&mut self) {
        self.x -= PLAYER_SPEED_X;
    }

    fn new(x: i32) -> Obstacle {
        let gap_size = rand::thread_rng().gen_range(MIN_OBSTACLE_GAP..MAX_OBSTACLE_GAP + 1);
        let gap_y = rand::thread_rng().gen_range(MIN_OBSTACLE_GAP_Y..MAX_OBSTACLE_GAP_Y + 1);

        Obstacle { x, gap_y, gap_size }
    }

    fn render(&self, ctx: &mut BTerm) {
        for i in 0..SCREEN_HEIGHT {
            if !(i >= self.gap_y && i < self.gap_y + self.gap_size) {
                ctx.set(self.x as i32, i, YELLOW, BLACK, to_cp437('+'));
            }
        }
    }
}
