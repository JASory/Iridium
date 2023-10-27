/*
  Molecular mass example as an application for Nuclide
*/

use ::Nuclide::Nuclide;
use ::Nuclide::ChemElement;

struct Water {
  oxygen : Nuclide,
  hydroone : Nuclide,
  hydrotwo : Nuclide, 
}

impl Water{

fn new(oxygen: Nuclide, hydroone: Nuclide, hydrotwo: Nuclide) -> Self{
   Self {oxygen, hydroone, hydrotwo}
}

fn mass(&self) -> f64{
  self.oxygen.am()+ self.hydroone.am() + self.hydrotwo.am()
}



}

fn main(){
  
  let protium = Nuclide::new("H-1").unwrap();
  let deuterium = Nuclide::new("H-2").unwrap();
  let tritium = Nuclide::new("H-3").unwrap();
  let oxy = Nuclide::new("O-16").unwrap();
  let h_oxy = Nuclide::new("O-18").unwrap(); 
  
  let potable = Water::new(oxy, protium, protium);
  let hdo = Water::new(oxy, deuterium, protium);
  let heavy_water = Water::new(oxy, deuterium, deuterium);
  let tritiated_water = Water::new(oxy, tritium, tritium);
  let super_heavy_water = Water::new(h_oxy, deuterium, deuterium);

  println!("Some masses of different types of \"water\" \n ");
  println!("Regular water H2O : {} daltons", potable.mass());
  println!("Semi-heavy water HDO : {} daltons", hdo.mass());
  println!("heavy water D2O : {} daltons", heavy_water.mass());
  println!("Tritiated water T2O : {} daltons", tritiated_water.mass());
  println!("Super heavy water D2O-18 : {} daltons", super_heavy_water.mass());

}
