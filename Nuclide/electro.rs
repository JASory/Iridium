use crate::elemental::ELECTRON_AFFINITY;
use crate::ionization::IONIZATION_ENERGIES;
use crate::elemental::THERMOCHEMICAL_ELECTRO_NEGATIVE;
use crate::elemental::ALLEN_ELECTRO;
use crate::elemental::PAULING_ELECTRO;
use crate::nuclide::Nuclide;


impl Nuclide{
        ///Returns electron affinity 
  pub  fn electron_affinity(&self)->f64{
          ELECTRON_AFFINITY[self.atomic_num()-1]
       }
  pub  fn electron_affinity_ev(&self)->f64{
          self.electron_affinity()*0.010364265
       }      
 ///Returns the ionization energies for all known levels. Values are in kj/mol
  pub  fn ionization_energies(&self, level: usize)->Result<f64, &'static str>{
          
          if self.atomic_num() > 110{
             return Ok(f64::NAN)
          }
          
          else if level > 0 && level < self.atomic_num()+1{
             return Ok(IONIZATION_ENERGIES[ (((self.atomic_num()*(self.atomic_num()+1))>>1) -self.atomic_num()) + level-1])
          }
          else{
             return Err("Not a supported value")
          }
       }
       
  pub fn ionization_energies_ev(&self, level: usize)->Result<f64, &'static str>{
         match self.ionization_energies(level){
               Ok(x)=> return Ok(x*0.010364265f64),
               Err(_)=> return Err("Not a supported value"),
         }
      }     
 /// Returns the thermochemical electronegativity as calculated by Oganov and Tantardini. Currently the best predictor of experimental values
  pub  fn electronegativity(&self)->f64{
          THERMOCHEMICAL_ELECTRO_NEGATIVE[self.atomic_num()-1]
       }
 ///Returns the Mullikan, or absolute, electronegativity in kj/mol
  pub  fn mullikan_en(&self)->f64{
          (self.ionization_energies(1).unwrap() + 
          ELECTRON_AFFINITY[self.atomic_num()-1])* 1.97E-3 + 0.19
       }
 ///Allen Electronegativity
  pub  fn allen_en(&self)->f64{
          ALLEN_ELECTRO[self.atomic_num()-1]
       }
 ///Pauling Electronegativity. A poor fit for experimental values, however it is here for completeness
  pub  fn pauling_en(&self)->f64{
          PAULING_ELECTRO[self.atomic_num()-1]
       }

}
