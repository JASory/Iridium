use ::Nuclide::Nuclide;
use ::Nuclide::ChemElement;

/*
   Nuclide does not have a standard atomic weight for the elements however it can be trivially computed for a sample
   given a list of the  nuclides known to exist and a vector of the distribution ratios 
*/

fn standard_atomic_mass(isos: &[Nuclide], dist: &[f64]) -> f64{
  let mut mass = 0f64;
  for i in 0..isos.len(){
    mass+=isos[i].am()*dist[i];
  }
  mass
}


fn main(){
let hydro = vec![Nuclide::new("H-1").unwrap(), Nuclide::new("H-2").unwrap(), Nuclide::new("H-3").unwrap()];
let distr = vec![0.9998,0.000156,1E-18];
 println!("The standard atomic mass of hydrogen is {:?} ", standard_atomic_mass(&hydro[..], &distr[..]));

}
