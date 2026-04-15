use serde::{Deserialize, Serialize};

pub fn save<T: Sized + Serialize>(object: &T, name: &str) {
    let path = format!("src/players/stardustz_bots/cnnml_bot/{}.json", name);
    std::fs::write(path, serde_json::to_string(object).unwrap()).unwrap();
}
pub fn load<T: Sized + for<'a> Deserialize<'a>>(name: &str) -> Option<T> {
    serde_json::from_str(
        &std::fs::read_to_string(format!("src/players/stardustz_bots/cnnml_bot/{}.json", name)).ok()?
    ).ok()
}

pub fn get_2d<T>(slice: &[T], x: usize, y: usize) -> Option<&T>{
    let size = slice.len().isqrt();
    slice.get(x + (y * size))
}
pub fn get_2d_mut<T>(slice: &mut [T], x: usize, y: usize) -> Option<&mut T>{
    let size = slice.len().isqrt();
    slice.get_mut(x + (y * size))
}
pub fn get_size<T>(slice: &[T]) -> usize {
    slice.len().isqrt()
}