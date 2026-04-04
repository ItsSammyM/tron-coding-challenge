use std::collections::HashMap;
use std::fmt::Display;
pub mod prelude;



pub struct GameEngine<A: Bot, B: Bot>{
    game_state: GameState,
    player_a_bot: A,
    player_b_bot: B
}
impl<A: Bot, B: Bot> GameEngine<A, B> {
    pub fn new()->Self{
        Self {
            game_state: GameState::new(),
            player_a_bot: A::new(PlayerId::new_a()),
            player_b_bot: B::new(PlayerId::new_b())
        }
    }
    /// returns true if game not over
    pub fn go_to_next_frame(&mut self) -> bool {
        self.game_state.go_to_next_frame(
            self.player_a_bot.next_action(&self.game_state),
            self.player_b_bot.next_action(&self.game_state)
        )
    }
    pub fn print_current_game_state(&self){
        println!("{}", self.game_state)
    }
    pub fn run_game(&mut self){
        self.print_current_game_state();
        while self.go_to_next_frame() {
            self.print_current_game_state();
        }
        self.print_current_game_state();
    }
}




pub struct GameState{
    grid_history: Vec<Grid>,
    game_over: Option<GameOver>,
}
impl GameState{
    pub fn new()->Self{
        Self { grid_history: Vec::from([Grid::new_default()]), game_over: None }
    }
    pub fn current_grid(&self)->&Grid{
        self.grid_history.last().expect("game state must have at least 1 grid")
    }
    pub fn get_current_time(&self)->usize{
        self.grid_history.len().checked_sub(1).expect("game state must have at least 1 grid")
    }
    pub fn get_grid(&self, time_since_start: usize)->Option<&Grid>{
        self.grid_history.get(time_since_start)
    }
    /// returns true if game not over
    fn go_to_next_frame(&mut self, player_a_choice: Direction, player_b_choice: Direction) -> bool {
        if self.game_over.is_some() {
            return false;
        }

        let next_frame_result = self.current_grid().next_grid(player_a_choice, player_b_choice);
        match next_frame_result {
            NextFrameResult::NextFrame(grid) => {
                self.grid_history.push(grid);
                true
            },
            NextFrameResult::Winner { player_who_won } => {
                self.game_over = Some(GameOver::Winner { player_who_won });
                false
            },
            NextFrameResult::Draw => {
                self.game_over = Some(GameOver::Draw);
                false
            }
        }
    }
}
impl Display for GameState{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.game_over {
            Some(game_over) => writeln!(f, "{}", game_over),
            None => writeln!(f, "{}", self.current_grid()),
        }
    }
}

pub enum GameOver{
    Winner{
        player_who_won: PlayerId
    },
    Draw
}
impl Display for GameOver{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GameOver::Winner { player_who_won } => format!("Game over: Player {} won", player_who_won),
            GameOver::Draw => "Game over: Draw".to_owned(),
        })
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextFrameResult{
    NextFrame(Grid),
    Winner{
        player_who_won: PlayerId
    },
    Draw
}

pub const GRID_SIZE: usize = 21;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid(
    [GridCell; GRID_SIZE as usize * GRID_SIZE as usize]
);
impl Grid{
    fn new_default()->Self{
        let mut out = Self([const { GridCell::Empty }; GRID_SIZE as usize * GRID_SIZE as usize]);
        *out.try_get_cell_mut((9,10)).expect("pos is in bounds") = GridCell::Head(PlayerId::new_a(), Direction::NegativeX);
        *out.try_get_cell_mut((11,10)).expect("pos is in bounds") = GridCell::Head(PlayerId::new_b(), Direction::PositiveX);
        out
    }
    pub fn next_grid(&self, player_a_choice: Direction, player_b_choice: Direction) -> NextFrameResult {

        //function is a hot mess

        let (a_pos, b_pos) = self.player_head_positions();

        let next_a_pos = a_pos.after_moved(player_a_choice);
        let next_b_pos = b_pos.after_moved(player_b_choice);

        if next_a_pos.is_none() && next_b_pos.is_none() {return NextFrameResult::Draw};

        let Some(next_a_pos) = next_a_pos else {return NextFrameResult::Winner { player_who_won: PlayerId::new_b() }};
        let Some(next_b_pos) = next_b_pos else {return NextFrameResult::Winner { player_who_won: PlayerId::new_a() }};

        if next_a_pos == next_b_pos {return NextFrameResult::Draw};
        
        let next_a_cell = self.get_cell(next_a_pos);
        let next_b_cell = self.get_cell(next_b_pos);
        
        let a_blocked = next_a_cell.is_not_empty();
        let b_blocked = next_b_cell.is_not_empty();

        if a_blocked && b_blocked {return NextFrameResult::Draw};
        if a_blocked {return NextFrameResult::Winner { player_who_won: PlayerId::new_b() }};
        if b_blocked {return NextFrameResult::Winner { player_who_won: PlayerId::new_a() }};

        let mut out = self.clone();
        *out.get_cell_mut(a_pos) = GridCell::Tail(PlayerId::new_a(), player_a_choice);
        *out.get_cell_mut(b_pos) = GridCell::Tail(PlayerId::new_b(), player_b_choice);
        *out.get_cell_mut(next_a_pos) = GridCell::Head(PlayerId::new_a(), player_a_choice);
        *out.get_cell_mut(next_b_pos) = GridCell::Head(PlayerId::new_b(), player_b_choice);

        NextFrameResult::NextFrame(
            out
        )
    }
    
    
    pub fn get_cell_mut(&mut self, pos: impl Into<GridPosition>)->&mut GridCell{
        self.0.get_mut(pos.into().0 as usize).expect("position is in bounds")
    }
    pub fn try_get_cell_mut(&mut self, pos: impl TryInto<GridPosition>)->Option<&mut GridCell>{
        self.0.get_mut(pos.try_into().ok()?.0 as usize)
    }
    pub fn get_cell(&self, pos: impl Into<GridPosition>)->&GridCell{
        self.0.get(pos.into().0 as usize).expect("position is in bounds")
    }
    pub fn try_get_cell(&self, pos: impl TryInto<GridPosition>)->Option<&GridCell>{
        self.0.get(pos.try_into().ok()?.0 as usize)
    }
    
    
    pub fn head_positions_map(&self)->HashMap<PlayerId, GridPosition>{
        self.0
            .iter()
            .enumerate()
            .filter_map(|(pos, cell)|{
                let GridCell::Head(player_id, ..) = cell else {return None};
                Some((*player_id, GridPosition(pos)))
            })
            .collect()
    }
    /// reutrns (Player A Head Position, Player B Head Position)
    pub fn player_head_positions(&self)->(GridPosition, GridPosition){
        (
            self.player_head_position(PlayerId(true)),
            self.player_head_position(PlayerId(false))
        )
    }
    pub fn player_head_position(&self, id: PlayerId)->GridPosition{
        self.0
            .iter()
            .enumerate()
            .find_map(|(pos, cell)|{
                let GridCell::Head(player_id, ..) = cell else {return None};
                if *player_id != id {return None}; 
                Some(GridPosition(pos))
            })
            .expect("both players must have a head")
    }

    pub fn cell_is_empty(&self, pos: impl Into<GridPosition>)->bool{
        self.get_cell(pos).is_empty()
    }
    pub fn cell_is_not_empty(&self, pos: impl Into<GridPosition>)->bool{
        self.get_cell(pos).is_not_empty()
    }
}
impl std::fmt::Display for Grid{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        for row in (0..GRID_SIZE).rev() {
            for col in 0..GRID_SIZE {
                let cell = self.try_get_cell((col,row)).expect("in bounds");
                string += &format!("{}", cell);
            }
            string += &format!("\n");
        }
        write!(f, "{}", string)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(bool);
impl PlayerId{
    pub fn new_a()->PlayerId{
        PlayerId(true)
    }
    pub fn new_b()->PlayerId{
        PlayerId(false)
    }
    pub fn is_a(&self)->bool{
        self.0
    }
    pub fn is_b(&self)->bool{
        !self.0
    }
    pub fn other(&self)->Self{
        PlayerId(!self.0)
    }
}
impl Display for PlayerId{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player = match self.0 {
            true => "A",
            false => "B",
        };
        write!(f, "{}", player)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridCell{
    Empty,
    Tail(PlayerId, Direction),
    Head(PlayerId, Direction)
}
impl GridCell{
    pub fn is_empty(&self) -> bool {
        *self == GridCell::Empty
    }
    pub fn is_not_empty(&self) -> bool {
        *self != GridCell::Empty
    }
    pub fn is_head(&self) -> bool {
        matches!(self, GridCell::Head(.. ))
    }
    pub fn is_players_head(&self, player: PlayerId) -> bool {
        if let GridCell::Head(p, ..) = self {
            player == *p
        }else{
            false
        }
    }
}
impl Display for GridCell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            GridCell::Empty => " .",
            GridCell::Tail(player_id, _direction) if player_id.is_a() => " o",
            GridCell::Tail(_player_id, _direction) => " x",
            GridCell::Head(_player_id, direction) => match direction {
                Direction::PositiveY => " ^",
                Direction::NegativeY => " v",
                Direction::PositiveX => " >",
                Direction::NegativeX => " <",
            },
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Invariant: Position is in bounds
pub struct GridPosition(usize);
impl From<GridPosition> for (usize, usize){
    fn from(value: GridPosition) -> Self {
        (value.0 % GRID_SIZE as usize, value.0 / GRID_SIZE as usize) 
    }
}
impl From<&GridPosition> for (usize, usize){
    fn from(value: &GridPosition) -> Self {
        (value.0 % GRID_SIZE as usize, value.0 / GRID_SIZE as usize) 
    }
}
pub struct GridPositionOutOfBoundsError;
impl TryFrom<(usize, usize)> for GridPosition{
    type Error = GridPositionOutOfBoundsError;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        GridPosition::new(value.0, value.1).ok_or(GridPositionOutOfBoundsError)
    }
}
impl GridPosition{
    pub fn new(x: usize, y: usize)->Option<GridPosition>{
        if x < GRID_SIZE && y < GRID_SIZE {
            Some(GridPosition(x as usize + (y * GRID_SIZE) as usize))
        }else{
            None
        }
    }
    /// returns None if result is outside grid
    pub fn after_moved(&self, direction: Direction)->Option<Self>{
        let (x,y) = self.into();
        match direction {
            Direction::PositiveY => GridPosition::new(x,y+1),
            Direction::NegativeY => GridPosition::new(x,y.checked_sub(1)?),
            Direction::PositiveX => GridPosition::new(x+1,y),
            Direction::NegativeX => GridPosition::new(x.checked_sub(1)?,y),
        }
    }
    pub fn is_empty(&self, grid: &Grid)->bool{
        grid.get_cell(*self).is_empty()
    }
    pub fn is_not_empty(&self, grid: &Grid)->bool{
        grid.get_cell(*self).is_not_empty()
    }

    pub fn get_cell<'a>(&self, grid: &'a Grid)->&'a GridCell{
        grid.get_cell(*self)
    }

    pub fn borders_cell<F: Fn(&GridCell) -> bool>(&self, grid: &Grid, condition: F)->bool{
        Direction::all_slice()
            .iter()
            .map(|d|self.after_moved(*d))
            .any(|pos|
                if let Some(pos) = pos {
                    condition(pos.get_cell(grid))
                } else {
                    false
                }
            )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction{
    PositiveY,
    NegativeY,
    PositiveX,
    NegativeX,
}
impl Direction{
    pub fn all()->impl Iterator<Item = Direction>{
        Self::all_slice().into_iter()
    }
    pub const fn all_slice()->[Self; 4]{
        [Direction::PositiveX, Direction::PositiveY, Direction::NegativeX, Direction::NegativeY]
    }
    pub const fn up()->Self{
        Direction::PositiveY
    }
    pub const fn down()->Self{
        Direction::NegativeY
    }
    pub const fn left()->Self{
        Direction::NegativeX
    }
    pub const fn right()->Self{
        Direction::PositiveX
    }
}
pub trait Bot{
    fn new(my_player_id: PlayerId)->Self;
    fn next_action(&mut self, game_state: &GameState)->Direction;
}