use nannou::prelude::*;
use rand::{thread_rng, Rng};

const WINDOW_WIDTH: u32 = 800;
const NUM_GRID_CELLS: u32 = 100;
const FRAMES_PER_ITERATION: i32 = 15;
const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WINDOW_WIDTH, WINDOW_WIDTH)
        .run();
}

struct Model {
    passed_frames: i32,
    iterations: i32,
    game_grid: GameGrid,
}

struct GameGrid {
    grid: [[bool; NUM_GRID_CELLS as usize]; NUM_GRID_CELLS as usize]
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

        GameGrid {
            grid,
        }
    }

    fn update(&mut self) {
        let mut new_grid = self.grid;

        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let live_neighbors = NEIGHBOR_OFFSETS.iter().fold(0, |acc, (x, y)| {
                    let x_index = i as i32 + x;
                    if x_index < 0 || x_index >= NUM_GRID_CELLS as i32 {
                        return acc;
                    }

                    let y_index = j as i32 + y;
                    if y_index < 0 || y_index >= NUM_GRID_CELLS as i32 {
                        return acc;
                    }

                    if let Some(&c) = self.grid.get(x_index as usize)
                        .and_then(|r| r.get(y_index as usize)) {
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
}

fn model(_app: &App) -> Model {
    Model {
        passed_frames: 0,
        iterations: 0,
        game_grid: GameGrid::new(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.passed_frames = (_model.passed_frames + 1) % FRAMES_PER_ITERATION;

    if _model.passed_frames == 0 {
        _model.iterations += 1;
        _model.game_grid.update();
    }
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();
    let cell_width  = app.window_rect().w() / NUM_GRID_CELLS as f32;
    let cell_height = app.window_rect().h() / NUM_GRID_CELLS as f32;

    for (i, row) in _model.game_grid.grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let color = match cell {
                true => DARKSEAGREEN,
                false => BLACK,
            };

            draw.rect()
                .x_y(
                    app.window_rect().left() + (cell_width) * (i as f32 + 0.5),
                    app.window_rect().top() - (cell_height) * (j as f32 + 0.5)
                )
                .w_h(cell_width, cell_height)
                .color(color);
        }
    }

    draw.text(app.fps().floor().to_string().as_str())
        .x_y(app.window_rect().left() + 10.0, app.window_rect().top() - 10.0);

    draw.to_frame(app, &frame).unwrap();
}
