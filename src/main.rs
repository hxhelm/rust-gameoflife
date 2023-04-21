use nannou::prelude::*;
use nannou::state::Mouse;
use nannou::Event::WindowEvent;
use rand::{thread_rng, Rng};

const WINDOW_WIDTH: u32 = 800;
const NUM_GRID_CELLS: u16 = 100;
const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .size(WINDOW_WIDTH, WINDOW_WIDTH)
        .run();
}

struct GameGrid {
    grid: [[bool; NUM_GRID_CELLS as usize]; NUM_GRID_CELLS as usize],
}

impl GameGrid {
    fn new() -> Self {
        let mut grid: [[bool; NUM_GRID_CELLS as usize]; NUM_GRID_CELLS as usize] =
            [[false; NUM_GRID_CELLS as usize]; NUM_GRID_CELLS as usize];

        for cell in grid.iter_mut().flat_map(|r| r.iter_mut()) {
            let mut rng = thread_rng();
            let state: bool = rng.gen();

            *cell = state;
        }

        Self { grid }
    }

    fn update(&mut self) {
        let mut new_grid = self.grid;

        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let live_neighbors = NEIGHBOR_OFFSETS.iter().fold(0, |acc, (x, y)| {
                    let x_index = <i32 as NumCast>::from(i).unwrap() + x;
                    if x_index < 0 || x_index >= <i32 as NumCast>::from(NUM_GRID_CELLS).unwrap() {
                        return acc;
                    }

                    let y_index = <i32 as NumCast>::from(j).unwrap() + y;
                    if y_index < 0 || y_index >= <i32 as NumCast>::from(NUM_GRID_CELLS).unwrap() {
                        return acc;
                    }

                    if let Some(&c) = self
                        .grid
                        .get(x_index as usize)
                        .and_then(|r| r.get(y_index as usize))
                    {
                        if c {
                            return acc + 1;
                        }
                    }

                    acc
                });

                if cell {
                    if !(2..=3).contains(&live_neighbors) {
                        new_grid[i][j] = false;
                    }
                } else if live_neighbors == 3 {
                    new_grid[i][j] = true;
                }
            }
        }

        self.grid = new_grid;
    }

    fn empty(&mut self) {
        self.grid = [[false; NUM_GRID_CELLS as usize]; NUM_GRID_CELLS as usize];
    }

    fn change_cell_state(&mut self, x: usize, y: usize) {
        self.grid[x][y] = !self.grid[x][y];
    }
}

struct Model {
    passed_frames: i32,
    frames_per_iteration: i32,
    paused: bool,
    game_grid: GameGrid,
}

fn model(_app: &App) -> Model {
    Model {
        passed_frames: 0,
        frames_per_iteration: 0,
        paused: false,
        game_grid: GameGrid::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.passed_frames = (model.passed_frames + 1) % model.frames_per_iteration;

    if model.passed_frames == 0 && !model.paused {
        model.game_grid.update();
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let WindowEvent {
        simple: Some(window_event),
        ..
    } = event
    {
        match window_event {
            KeyReleased(Key::R) => model.game_grid = GameGrid::new(),
            KeyReleased(Key::Q) => app.quit(),
            KeyReleased(Key::Space) => model.paused = !model.paused,
            KeyReleased(Key::B) => {
                model.game_grid.empty();
                model.paused = true;
            }
            MousePressed(MouseButton::Left) => {
                let (x, y) = get_grid_index_from_mouse_position(&app.mouse, &app.window_rect());
                model.game_grid.change_cell_state(x, y);
            }
            _ => (),
        }
    };
}

fn get_grid_index_from_mouse_position(mouse: &Mouse, window: &Rect) -> (usize, usize) {
    let pos = mouse.position();

    let cell_x: f32 =
        (pos.x - window.left()) / (window.w() / <f32 as NumCast>::from(NUM_GRID_CELLS).unwrap());
    let cell_x = cell_x.floor() as usize;

    let cell_y: f32 =
        (window.top() - pos.y) / (window.h() / <f32 as NumCast>::from(NUM_GRID_CELLS).unwrap());
    let cell_y = cell_y.floor() as usize;

    (cell_x, cell_y)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window_left = app.window_rect().left();
    let window_top = app.window_rect().top();

    let cell_width = app.window_rect().w() / <f32 as NumCast>::from(NUM_GRID_CELLS).unwrap();
    let cell_height = app.window_rect().h() / <f32 as NumCast>::from(NUM_GRID_CELLS).unwrap();

    draw.background().color(BLACK);

    model
        .game_grid
        .grid
        .iter()
        .enumerate()
        .for_each(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c)
                .for_each(|(j, _)| {
                    draw.rect()
                        .x_y(
                            (cell_width).mul_add(i as f32 + 0.5, window_left),
                            (cell_height).mul_add(-(j as f32) - 0.5, window_top),
                        )
                        .w_h(cell_width, cell_height)
                        .color(DARKSEAGREEN);
                });
        });

    draw.text(app.fps().floor().to_string().as_str()).x_y(
        app.window_rect().left() + 10.0,
        app.window_rect().top() - 10.0,
    );

    draw.to_frame(app, &frame).unwrap();
}
