use rand::{RngExt as _, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::players::stardustz_bots::cnnml_bot::helper::*;






#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model{
    first: ConvolutionLayer, // 7x7 -> 5x5
    second: ConvolutionLayer, // 7x7 -> 5x5

    third: DenseLayer, // 5x5 + 5x5 + 15 memory + 3 neighbor squares (left up right) -> same as in size
    fourth: DenseLayer, // same length as thrid out -> 3 output direction (left up right) + 15 memory
}
impl Model {

    const OUT_FIRST_SIZE: usize = 5*5;
    pub const MEMORY_SIZE: usize = 15;
    const IN_THIRD_SIZE: usize = Self::OUT_FIRST_SIZE + Self::OUT_FIRST_SIZE + Self::MEMORY_SIZE + 3;
    const OUT_FOURTH_SIZE: usize = 15 + 3;

    pub fn forward(&self, x: ModelInput) -> ModelOutput {
        let mut dense_layer_input = Vec::new();

        dense_layer_input.append(&mut self.first.forward(&x.image));
        dense_layer_input.append(&mut self.second.forward(&x.image));
        dense_layer_input.append(&mut Vec::from(x.memory));
        dense_layer_input.append(&mut Vec::from(x.neighboring_cells));
        
        let dense_output = self.third.forward(dense_layer_input.as_slice());
        let dense_output = self.fourth.forward(dense_output.as_slice());
        
        let direction = &dense_output[0..3];
        let memory = &dense_output[3..3+Self::MEMORY_SIZE];
        ModelOutput {
            memory: Vec::from(memory),
            direction: ModelOutputDirection::from_slice(direction)
        }
    }
    pub fn randomize(mut self, rng: &mut ThreadRng, max: f32)->Self{
        self.first = self.first.randomize(rng, max);
        self.second = self.second.randomize(rng, max);
        self.third = self.third.randomize(rng, max);
        self.fourth = self.fourth.randomize(rng, max);
        self
    }
}
impl Default for Model {
    fn default() -> Self {
        Self {
            first: Default::default(),
            second: Default::default(),
            third: DenseLayer::new(Self::IN_THIRD_SIZE, Self::IN_THIRD_SIZE),
            fourth: DenseLayer::new(Self::IN_THIRD_SIZE, Self::OUT_FOURTH_SIZE)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ConvolutionLayer{
    weights: [f32; 9], // 3x3
}
impl ConvolutionLayer {
    // invariant: slice has length equal to a perfect squaremut
    fn forward(&self, slice: &[f32])->Vec<f32>{
        let size = get_size(slice);
        let mut out = vec![0f32; (size - 2) * (size - 2)];
        for x in 1..(size-1) {
            for y in 1..(size-1) {
                (*get_2d_mut(out.as_mut_slice(), x - 1, y - 1).unwrap()) = 
                    *get_2d(&self.weights, 0, 0).unwrap() as f32 * *get_2d(slice, x - 1, y - 1).unwrap() as f32 +
                    *get_2d(&self.weights, 1, 0).unwrap() as f32 * *get_2d(slice, x + 0, y - 1).unwrap() as f32 +
                    *get_2d(&self.weights, 2, 0).unwrap() as f32 * *get_2d(slice, x + 1, y - 1).unwrap() as f32 +
                    *get_2d(&self.weights, 0, 1).unwrap() as f32 * *get_2d(slice, x - 1, y + 0).unwrap() as f32 +
                    *get_2d(&self.weights, 1, 1).unwrap() as f32 * *get_2d(slice, x + 0, y + 0).unwrap() as f32 +
                    *get_2d(&self.weights, 2, 1).unwrap() as f32 * *get_2d(slice, x + 1, y + 0).unwrap() as f32 +
                    *get_2d(&self.weights, 0, 2).unwrap() as f32 * *get_2d(slice, x - 1, y + 1).unwrap() as f32 +
                    *get_2d(&self.weights, 1, 2).unwrap() as f32 * *get_2d(slice, x + 0, y + 1).unwrap() as f32 +
                    *get_2d(&self.weights, 2, 2).unwrap() as f32 * *get_2d(slice, x + 1, y + 1).unwrap() as f32
            }
        }
        out
    }
    fn randomize(mut self, rng: &mut ThreadRng, max: f32)->Self{
        for weight in &mut self.weights {
            *weight += rng.random_range(-max..max)
        }
        self
    }
}
impl Default for ConvolutionLayer{
    fn default() -> Self {
        Self { weights: [1.0; 9] }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct DenseLayer{
    weights: Vec<f32>,
    bias: Vec<f32>,

    in_dim: usize,
    out_dim: usize
}
impl DenseLayer {
    fn new(in_dim: usize, out_din: usize) -> Self {
        Self { weights: vec![1.0; in_dim*out_din], bias: vec![0.0; out_din], in_dim, out_dim: out_din }
    }
    fn forward(&self, slice: &[f32]) -> Vec<f32> {
        assert_eq!(slice.len(), self.in_dim);

        let mut output = vec![0.0; self.out_dim];

        for o in 0..self.out_dim {
            let mut sum = 0.0;

            // weights for this output neuron start at o * in_dim
            let row_start = o * self.in_dim;

            for i in 0..self.in_dim {
                sum += self.weights[row_start + i] * slice[i];
            }

            output[o] = relu(sum + self.bias[o]);
        }

        output
    }
    fn randomize(mut self, rng: &mut ThreadRng, max: f32)->Self{
        for weight in &mut self.weights {
            *weight += rng.random_range(-max..max)
        }
        for weight in &mut self.bias {
            *weight += rng.random_range(-max..max)
        }
        self
    }

}


fn relu(x: f32)->f32{
    if x > 0.0{
        x
    }else{
        x/5.0
    }
}


pub struct ModelInput{
    pub image: Vec<f32>, // 7*7
    pub memory: Vec<f32>, // 15
    pub neighboring_cells: Vec<f32>, // 3 (left up right)
}
pub struct ModelOutput{
    pub memory: Vec<f32>, //5
    pub direction: ModelOutputDirection
}
pub enum ModelOutputDirection{
    Left, Up, Right,
}
impl ModelOutputDirection {
    fn from_slice(slice: &[f32]) -> Self {
        match slice
            .into_iter()
            .enumerate()
            .max_by(|(_, x), (_,y)|
                x.total_cmp(y)
            )
            .map(|(i,_)|i)
            .unwrap()
        {
            0 => ModelOutputDirection::Left,
            1 => ModelOutputDirection::Up,
            2 => ModelOutputDirection::Right,
            _ => panic!()
        }
    }
}