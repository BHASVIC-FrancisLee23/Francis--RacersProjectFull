use core::f64;
use std::iter;

#[derive(Clone)]
pub struct Layer {
    pub weights: Vec<Vec<f64>>,
    pub bias: Vec<f64>,
    activation: Option<fn(f64) -> f64>,
    pub output: Option<Vec<f64>>,
}

#[derive(Clone)]
pub struct Network {
    pub layers: Vec<Layer>,
}

impl Network {
    pub fn new_empty() -> Self {
        Self { layers: Vec::new() } // return empty network
    }

    pub fn add_layer(mut self, layer: Layer) -> Self {
        self.layers.push(layer);
        return self;
    }

    pub fn run(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        // run first layer
        let mut prev_output: Vec<f64> = inputs;
        for layer in self.layers.iter_mut() {
            prev_output = layer.calculate(prev_output);
        }
        return prev_output;
    }
}

impl Layer {
    pub fn new(weights: Vec<Vec<f64>>, bias: Vec<f64>, activation: Option<fn(f64) -> f64>) -> Self {
        Self {
            weights,
            bias,
            activation,
            output: None,
        }
    }

    pub fn new_random(inputs: usize, outputs: usize, activation: Option<fn(f64) -> f64>) -> Self {
        // generating the weights between 0.75 and -0.75
        let mut weights: Vec<Vec<f64>> = vec![];
        for i in 0..outputs {
            let mut inner = vec![];
            for j in 0..inputs {
                let val = macroquad::rand::gen_range(-0.75, 0.75);
                inner.push(val);
            }
            weights.push(inner);
        }

        let mut bias: Vec<f64> = vec![];
        for i in 0..outputs {
            let val = macroquad::rand::gen_range(0.25, -0.25);
            bias.push(val);
        }

        return Self::new(weights, bias, activation);
    }

    pub fn calculate(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        // make sure that the inputs and weights are compatable sizes
        if (inputs.len() != self.weights[0].len()) {
            panic!("[network.rs] - Incompatable input and weights sizes!\n");
        }

        // create empty list of zeros
        let mut outputs: Vec<f64> = iter::repeat(0.0).take(self.bias.len()).collect();

        // perform the matrix multiplication of inputs and weights
        for output_index in 0..outputs.len() {
            let bias = self.bias[output_index];
            for input_index in 0..inputs.len() {
                let input = inputs[input_index];
                let weight = self.weights[output_index][input_index];
                outputs[output_index] += weight * input;
            }
            outputs[output_index] += bias;
        }

        // if there is an activation function return the values with the function applied
        if (self.activation.is_some()) {
            return self.apply_activation(outputs);
        }

        self.output = Some(outputs.clone());

        return outputs;
    }

    fn apply_activation(&self, mut inputs: Vec<f64>) -> Vec<f64> {
        for i in 0..inputs.len() {
            inputs[i] = self.activation.unwrap()(inputs[i]);
        }
        return inputs;
    }
}

pub fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + f64::consts::E.powf(-x));
}
