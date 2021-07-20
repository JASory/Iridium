
use crate::elemental::VAN_DER_WAAL_CRYSTAL;
use crate::elemental::VAN_DER_WAAL_ISOLATED;
use crate::elemental::COVALENT_RADII;
use crate::nuclide::Nuclide;

impl Nuclide{

    ///Returns the approximate covalent radii of the nuclei in a single, double, or triple bond. Values other than 1,2, && 3 will return an error.  
   pub  fn covalent_radii(&self, bond: usize)->Result<f64, String>{
          if bond > 0 && bond < 4{
             return Ok(COVALENT_RADII[(self.atomic_num()-1)*3 + bond-1])
          }
          else{
            return Err("Not a tested bond type".to_string())
          }
 
       }

     ///Returns the Van Der Waal radius in crystalline structures. Values are in meters. 
  pub  fn vdr_crystal(&self)->f64{
              VAN_DER_WAAL_CRYSTAL[self.atomic_num()-1]
          }
       ///Returns the Van Der Waal radius of isolated atoms 
  pub  fn vdr_isolated(&self)->f64{
            VAN_DER_WAAL_ISOLATED[self.atomic_num()-1]
         }
}
