use wasm_bindgen::prelude::wasm_bindgen;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

#[derive(PartialEq)]
#[allow(dead_code)]
#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub enum GameState {
    Won,
    Lost,
    Playing,
}

#[wasm_bindgen]
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    pub fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body: body,
            direction: Direction::Up,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    state: Option<GameState>,
    reward_cell: Option<usize>,
    points: usize,
}

impl Default for World {
    fn default() -> Self {
        let width = 8;
        let size = width * width;
        let snake = Snake::new(10, 3);

        World {
            reward_cell: Option::None,
            width: width,
            points: 0,
            size: size,
            snake: snake,
            next_cell: Option::None,
            state: Option::None,
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_start_idx: usize) -> Self {
        let snake = Snake::new(10, 3);
        let size = width * width;
        World {
            width: width,
            size: width * width,
            snake: Snake::new(snake_start_idx, 3),
            reward_cell: World::generate_reward_cell(size, &snake.body),
            ..World::default()
        }
    }

    pub fn game_state(&self) -> Option<GameState> {
        self.state
    }

    pub fn game_state_text(&self) -> String {
        match self.state {
            Some(GameState::Won) => String::from("You have Won"),
            Some(GameState::Lost) => String::from("You have Lost"),
            Some(GameState::Playing) => String::from("You are Playing"),
            None => String::from("No State"),
        }
    }

    pub fn points(&self) -> usize {
        self.points
    }

    /// Starts the game
    pub fn start_game(&mut self) {
        self.state = Some(GameState::Playing);
    }

    // *const is a raw pointer
    // borrowing rules do not apply to this expression
    // basically convices the compiler that the Snake will exist as long as the World does
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn snake_len(&self) -> usize {
        self.snake.body.len()
    }

    // cannot return a reference to JS due to borrowing values
    // pub fn snake_cells(&self) -> Vec<SnakeCell> {
    //     self.snake.body
    // }
    pub fn set_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.generate_next_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn step(&mut self) {
        match self.state {
            Some(GameState::Playing) => {
                let temp = self.snake.body.clone();

                self.snake.body[0] = match self.next_cell {
                    Some(cell) => {
                        self.next_cell = Option::None;
                        cell
                    }
                    None => self.generate_next_cell(&self.snake.direction),
                };

                for i in 1..self.snake_len() {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0)
                }

                if self.snake.body[1..self.snake_len()].contains(&self.snake.body[0]) {
                    self.state = Some(GameState::Lost);
                }

                if self.reward_cell == Some(self.snake_head_idx()) {
                    if self.snake_len() < self.size {
                        self.points += 1;
                        self.reward_cell = World::generate_reward_cell(self.size, &self.snake.body);
                    } else {
                        self.reward_cell = None;
                        self.state = Some(GameState::Won);
                    }
                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            }
            _ => {}
        };
    }

    fn generate_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut reward_cell;

        loop {
            reward_cell = rnd(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }

        Some(reward_cell)
    }

    fn generate_next_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        match direction {
            Direction::Right => {
                let threshold = (row + 1) * self.width;
                if snake_idx + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let threshold = (row) * self.width;
                if snake_idx == threshold {
                    SnakeCell(threshold + self.width - 1)
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let threshold = snake_idx - (row * self.width);
                if snake_idx == threshold {
                    SnakeCell((self.size - self.width) + threshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let threshold = snake_idx - ((self.width - row) * self.width);
                if snake_idx + self.width == threshold {
                    SnakeCell(threshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        }
    }
}

//  wasm-pack build --target web
