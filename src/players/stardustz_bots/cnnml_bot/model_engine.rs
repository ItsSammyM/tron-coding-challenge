use crate::{engine::prelude::*, players::stardustz_bots::cnnml_bot::model::{Model, ModelInput, ModelOutput, ModelOutputDirection}};

pub struct ModelEngine<'a>{
    grid: &'a Grid,
    player: PlayerId,
    model: &'a Model,
}
impl<'a> ModelEngine<'a> {
    pub fn new(args: BotArgs, grid: &'a Grid, model: &'a Model)->Self{
        Self{
            grid,
            player: args.my_player(),
            model,
        }
    }
    fn get_screen(&self) -> Vec<f32> {
        let mut screen: Vec<f32> = vec![0.0; 7*7];
        let head_pos = self.player.get_head_pos(self.grid);
        for x in (head_pos.x() as i32-3i32)..(head_pos.x() as i32+3i32) {
            for y in (head_pos.y() as i32-3i32)..(head_pos.y() as i32+3i32) {
                *screen.get_mut((x - head_pos.x() as i32 + 3) as usize + ((y - head_pos.y() as i32 + 3) as usize * 7)).unwrap() =
                    Self::pos_is_blocked(self.grid, x, y)
            }
        }
        screen
    }
    // 1 if blocked, 0 if clear
    fn pos_is_blocked(grid: &'a Grid, x: i32, y: i32) -> f32 {
        let Some(x) = usize::try_from(x).ok() else {return 1.0};
        let Some(y) = usize::try_from(y).ok() else {return 1.0};
        let Some(pos) = GridPosition::new(x, y) else {return 1.0};
        if pos.is_not_empty(grid) {return 1.0}
        0.0
    }
    fn pos_is_blocked_grid_pos(grid: &'a Grid, pos: Option<GridPosition>) -> f32 {
        let Some(pos) = pos else {return 1.0};
        if pos.is_not_empty(grid) {return 1.0}
        0.0
    }
    fn rotate_grid(current: &Vec<f32>, new_up: Direction) -> Vec<f32> {
        let size = 7;
        assert_eq!(current.len(), size * size);

        let mut result = vec![0.0; size * size];

        for y in 0..size {
            for x in 0..size {
                let (nx, ny) = match new_up {
                    Direction::PositiveY => (x, y),
                    // 90° clockwise
                    Direction::PositiveX => (size - 1 - y, x),
                    // 180°
                    Direction::NegativeY => (size - 1 - x, size - 1 - y),
                    // 90° counterclockwise
                    Direction::NegativeX => (y, size - 1 - x),
                };

                result[nx + ny * size] = current[x + y * size];
            }
        }

        result
    }
    fn get_model_input(&self, memory: Vec<f32>) -> ModelInput {
        let head_direction = self.player.get_head_direction(self.grid);
        let position = self.player.get_head_pos(self.grid);

        let screen = self.get_screen();
        let rotated = Self::rotate_grid(&screen, head_direction);
        
        let mut nearby_cells = Vec::new();
        nearby_cells.push(Self::pos_is_blocked_grid_pos(self.grid, position.after_moved(head_direction.left_of())));
        nearby_cells.push(Self::pos_is_blocked_grid_pos(self.grid, position.after_moved(head_direction)));
        nearby_cells.push(Self::pos_is_blocked_grid_pos(self.grid, position.after_moved(head_direction.right_of())));
        ModelInput { image: rotated, memory, neighboring_cells: nearby_cells }
    }
    pub fn parse_model_output(&self, output: ModelOutputDirection)->Direction{
        let current_dir = self.grid.player_head_direction(self.player);
        match output {
            ModelOutputDirection::Left => current_dir.left_of(),
            ModelOutputDirection::Up => current_dir,
            ModelOutputDirection::Right => current_dir.right_of(),
        }
    }
    pub fn get_model_next_step(&self, memory: Vec<f32>)->ModelOutput{
        self.model.forward(
            Self::get_model_input(&self, memory)
        )
    }
}