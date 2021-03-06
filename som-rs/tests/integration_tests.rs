use ndarray::prelude::*;
use ndarray_rand::rand::SeedableRng;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use rand_isaac::isaac64::Isaac64Rng;

use som_rs::default::*;
use som_rs::{NeuralLayer, Neurons, SelfOrganizing};

#[test]
fn test_kohonen(){
    let seed = 42;

    let mut rng = Isaac64Rng::seed_from_u64(seed);

    let mut som = NeuralLayer {
        neurons: Neurons {
            // lateral: Array2::<f64>::zeros((0,0)),
            patterns: Array::random_using((100, 3), Uniform::new(0., 10.), &mut rng),
            ..Default::default()
        },
        adaptivity: KohonenAdaptivity {},
        topology: CartesianTopology::new((10, 10)),
        responsiveness: CartesianResponsiveness {},
        training: BatchTraining {
            radii: (2.0, 0.2),
            rates: (0.7, 0.1),
            epochs: 1,
        },
    };

    // println!("{}", som.neurons.lateral);

    som.init_lateral();
    let training = Array::random_using((5000, 2), Uniform::new(0., 9.), &mut rng);
    som.train(&training.view());
    som.adapt(&training.row(0), 0.7, 0.7);
}