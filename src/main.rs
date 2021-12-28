extern crate deep_learning;
use deep_learning::activation;
use deep_learning::{DenseLayer, Network};
use deep_learning::loss;

fn main() {
    let data = vec![0.0, 2.0, 3.0, 4.0];
    let net = Network {
        layers: vec![
            Box::new(DenseLayer::new(4, 3, activation::relu)),
            Box::new(DenseLayer::new(3, 3, activation::relu)),
            Box::new(DenseLayer::new(3, 1, activation::linear)),
        ],
        loss: loss::ms_error,
    };
    let ret = net.process(&data);
    println!("{:?}", ret);
    let des = vec![4.0];
    let act = vec![2.0];
    println!("{:?}", loss::ms_error(&des, &act));
}
