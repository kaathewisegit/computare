use computare_special::gamma::gamma;

use rug::{Float, az::Az};

#[test]
fn gamma_step() {
    for i in 1..765 {
        let rf: Float = Float::with_val(500, i) / 97;
        let rug_gamma = rf.clone().gamma().az::<f64>();
        let my_gamma = gamma(rf.az::<f64>());

        let rel = (rug_gamma - my_gamma).abs() / rug_gamma;
        assert!(rel < 1e-15, "{rel:?}, {i}");
    }

    for i in 33 * 97..100 * 97 {
        let rf: Float = Float::with_val(500, i) / 97;
        let rug_gamma = rf.clone().gamma().az::<f64>();
        let my_gamma = gamma(rf.az::<f64>());

        let rel = (rug_gamma - my_gamma).abs() / rug_gamma;
        assert!(rel < 1e-13, "{rel:?}, {i}");
    }
}
