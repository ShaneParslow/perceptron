#![feature(iter_zip)]

pub mod activation;
pub mod loss;

#[derive(Clone, Default)]
struct Node {
    weights: Vec<f64>,
    bias: f64,
}

/// Has a set of weights that gets dotted with an input vector and added to a bias
/// Initialized to constant length of input data
impl Node {
    fn new(len: usize) -> Node {
        Node {
            weights: vec![1.0; len],
            bias: 0.0,
            ..Default::default()
        }
    }
    fn process(&self, data: &Vec<f64>) -> f64 {
        let mut sum: f64 = 0.0;
        if self.weights.len() != data.len() {
            panic!("data weights mismatch, this probably shouldnt panic");
        }
        for (weight, val) in std::iter::zip(self.weights.iter(), data.iter()) {
            sum += weight * val;
        }
        sum + self.bias
    }
}

//#[derive(Clone)]
pub struct DenseLayer {
    nodes: Vec<Node>,
    activation: fn(f64) -> f64,
}

impl DenseLayer {
    pub fn new(inlen: usize, nodelen: usize, activ: activation::Activation) -> DenseLayer {
        DenseLayer {
            nodes: vec![Node::new(inlen); nodelen],
            activation: activ,
        }
    }
}

impl Layer for DenseLayer {
    fn process(&self, data: &Vec<f64>) -> Vec<f64> {
        let mut ret = Vec::new();
        let mut node_out;
        for node in &self.nodes {
            node_out = node.process(data);
            node_out = (self.activation)(node_out);
            ret.push(node_out);
        }
        ret
    }
}

// trait Process? or something like that? more rustish
pub trait Layer {
    //fn new(inlen: usize, outlen: usize, activ: fn(f64) -> f64) -> Box<Self>;
    fn process(&self, data: &Vec<f64>) -> Vec<f64>;
}

// TODO: this maybe shouldnt have pub fields
pub struct Network {
    pub layers: Vec<Box<dyn Layer>>,
    pub loss: fn(&Vec<f64>, &Vec<f64>) -> f64
}

impl Network {
    /*
    pub fn process(&self, data: &Vec<f64>) -> Vec<f64> {
        let mut proc = self.layers[0].process(data);
        for box_layer in self.layers[ {
            proc = box_layer.process(&proc);
        }
        proc
    }
    */
    pub fn process(&self, data: &Vec<f64>) -> Vec<f64> {
        if let Some((first, layers_split)) = self.layers.split_first() {
            let mut processed = first.process(data);
            for layer in layers_split {
                processed = layer.process(&processed);
            }
            processed
        } else {
            panic!("empty layer list");
        }
    }

    /// Return 2d vector of weights for each layer
    fn gradient(input: &Vec<f64>) -> Vec<Vec<f64>> {
        vec![vec![0.0, 2.0]; 2]
    }
}
