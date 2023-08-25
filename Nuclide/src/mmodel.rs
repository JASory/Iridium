/*
    Mass model computing the binding energy
    
    In: total nucleons, number of protons
    Out: Binding energy in MeV
*/

pub(crate) fn mass_model(a: usize, z: usize) -> f64 {
    let (af64, zf64) = (a as f64, z as f64);

    let even_odd_approx = 14.6433 * af64
        - 14.0788 * af64.powf(2.0 / 3.0)
        - 0.66442 * (zf64.powi(2) / af64.powf(1.0 / 3.0))
        - 21.068 * ((af64 - 2.0 * zf64).powi(2)) / af64;

    let correction = 11.5398 * (af64.sqrt().recip());

    if z % 2 == 0 && a % 2 == 0 {
        return even_odd_approx + correction;
    }
    if z % 2 == 1 && a % 2 == 1 {
        even_odd_approx - correction
    } else {
        even_odd_approx
    }
}
