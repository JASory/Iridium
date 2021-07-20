//mod index;
use crate::index::SYMBOL;
use crate::index::SYMBOL_INDEX;
use crate::atomic_mass::ATOMIC_MASS;

/// Takes the proton_neutron count and returns the nuclide index, useful to pair with change and assign
pub fn nucleons_nuclide(x: &(usize, usize))-> usize{
    SYMBOL_INDEX[x.0-1].0 -(SYMBOL_INDEX[x.0 -1].1-x.0) + x.1 
 }
///Calls RDRAND to produce random numbers for decay
 pub fn rand()->u64{
   let mut x: u64 = 0;
  let k = unsafe{core::arch::x86_64::_rdrand64_step(&mut x)};
x
}
/*
pub fn rand()->u64{


}
*/

 pub struct Nuclide{
     nuclide: usize,
}


impl Clone for Nuclide{
  fn clone(&self)->Nuclide{
  Nuclide{nuclide: self.nuclide.clone()}
  }
}

impl Nuclide{
/**This function takes the symbol abbreviation and the isotope number.  For example to get the nuclide for Strontium-90
```
use Nuclide::nuclide::Nuclide;

 let strontium = Nuclide::new("Sr", 90).unwrap();
 ```
If you input an abbreviation that isn't supported or an isotope that isn't supported you will receive an error saying so. There are currently 3299 nuclides supported, which will cover most needs.
*/



  pub fn new(x: &str, isotope: usize)-> Result<Nuclide,String >{
 
         match SYMBOL.iter().position(|y| y ==&x){
          Some(x)=> if isotope >= SYMBOL_INDEX[x].1 &&  isotope <= SYMBOL_INDEX[x].2 {
                       return Ok(
                     Nuclide{nuclide: SYMBOL_INDEX[x].0+isotope-SYMBOL_INDEX[x].1}
                     )
                   }
                   else{
                   return Err("Not a known isotope".to_string())
                   }
         None=> return Err("Not a known element".to_string())
       }
       
       }
 
 /** Directly assigns the argument as the value. This function performs zero checks so it is up to the user to insure that the value given is correct. Pair with the nucleons_nuclide function to ensure that you have a correct value. 
 
 ```
 let nucleon_count = strontium.proton_neutron();
 let nuclide_index = nucleons_nuclide(&nucleon_count);
 
 assert_eq!(nuclide_index, strontium.nuclide_index());
 
 let strontium90   = Nuclide::assign(nuclide_index); 
 
 ```
 
 */
  pub fn assign(x: usize)->Nuclide{
      Nuclide{nuclide: x}
  }
 /** Mutates the element in place to the argument given. Zero checks are made, so verify that it is correct
 
 ```
 let radium_converter = radium.nuclide_index();
 
 strontium.change(radium_converter); 
 
 assert_eq!(strontium.identity(), radium.identity())
 ```
 
 */
  pub  fn change(&mut self, x: usize){
          self.nuclide = x
     }
 ///Returns the nuclide index used in the struct. Useful to use in conjunction with the assign or change functions. 
  pub  fn nuclide_index(&self)->usize{
          self.nuclide.clone()
     }
 
 ///Returns the atomic number
  pub  fn atomic_num(&self)->usize{
          let mut count: usize = 0;
          for i in SYMBOL_INDEX{
  
           if i.0 > self.nuclide{
              break;
           }
         count+=1;
         }
        count
       }
/// Returns the element and the nucleon count
pub  fn isotope(&self)->(usize, usize){
          let element = self.atomic_num();
          (element, 
          (self.nuclide -SYMBOL_INDEX[element-1].0)+SYMBOL_INDEX[element-1].1
          )
       }
       
       
     /** 
     Returns the name and isotope number of the nuclide
     
        ```
        let radium = Nuclide::new("Ra", 222).unwrap();
        
        assert_eq!(radium.identity(), "Ra-222");
        ```
     */
  pub  fn identity(&self)-> String{
          let iso = self.isotope();
          SYMBOL[iso.0-1].to_owned() + "-"+ &iso.1.to_string()
       }
 ///Returns the proton and neutron count 
  pub  fn proton_neutron(&self)->(usize, usize){
          (self.isotope().0,  self.isotope().1 - self.isotope().0)
       }
       /// Returns the atomic mass in daltons 
  pub  fn amu(&self)->f64{
          ATOMIC_MASS[self.nuclide]
       }
 ///Returns the atomic mass in kilograms
  pub  fn am_kg(&self)->f64{
          self.amu()*1.6605390666E-27
       }
       
  pub  fn mass_deficit(&self)->f64{
          let nucleons = self.proton_neutron();
          (1.007276466621*nucleons.0 as f64 + 5.48579909065E-4*nucleons.0 as f64
           + 1.00866491588*nucleons.1 as f64)-self.amu()
       }
 
  pub  fn mass_deficit_kg(&self)->f64{
       self.mass_deficit()* 1.6605390666E-27
       }
       
  //Result as electron volts, implement Myers Swiatecki semi-empirical equation
  ///Returns the binding energy. Utilizes basic mass deficit conversion to approximate.  
  pub  fn binding_energy(&self)->f64{
          self.mass_deficit()*931.49410242
       }
     
 pub fn binding_energy_j(&self)->f64{
      self.binding_energy()*1.602176634E-19
     }      
       
      
}
