use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(bool);
impl PlayerId{
    pub fn new_o()->PlayerId{
        PlayerId(true)
    }
    pub fn new_x()->PlayerId{
        PlayerId(false)
    }
    pub fn is_o(&self)->bool{
        self.0
    }
    pub fn is_x(&self)->bool{
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