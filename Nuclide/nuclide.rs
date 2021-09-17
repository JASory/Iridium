use crate::index::SYMBOL;
use crate::index::SYMBOL_INDEX;
use crate::index::NAME;
use crate::atomic_mass::ATOMIC_MASS;
use crate::particle::PROTONMASS;
use crate::particle::NEUTRONMASS;

/// Takes the proton_neutron count and returns the nuclide index, useful to pair with change and assign
pub fn nucleons_nuclide(x: &(usize, usize))-> usize{
    SYMBOL_INDEX[x.0-1].0 -(SYMBOL_INDEX[x.0 -1].1-x.0) + x.1 
 }
 
 
 /** Returns binding energy as computed by Bethe-Weizsacker formula using Benzaid et. al's parameters. This model is used for 
     computing the binding_energy, and  proton/neutron separation energies
 */
 
pub fn mass_model(a: usize, z: usize)->f64{

   let (af64, zf64) = (a as f64, z as f64);

   let even_odd_approx  = 14.6433*af64 - 14.0788*af64.powf(2.0/3.0) 
                         -0.66442*(zf64.powi(2)/af64.powf(1.0/3.0))  
                         - 21.068*((af64 - 2.0*zf64).powi(2))/af64; 
                      
   let correction = 11.5398*(af64.sqrt().recip());
   
   if  z%2 == 0 && a%2 == 0{
       return even_odd_approx + correction
    }                       
    if z%2 == 1 && a%2 == 1{
       return even_odd_approx - correction
   }
   else{
       return even_odd_approx 
   }     
} 



#[derive(Debug)]
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
If you input an abbreviation that isn't supported or an isotope that isn't supported you will receive an error saying so. There are currently 3585 nuclides supported, which will cover most needs.
*/



  pub fn new(x: &str, isotope: usize)-> Result<Nuclide,&'static str >{
 
         match SYMBOL.iter().position(|y| y ==&x){
          Some(x)=> if isotope >= SYMBOL_INDEX[x].1 &&  isotope <= SYMBOL_INDEX[x].2 {
                       return Ok(
                     Nuclide{nuclide: SYMBOL_INDEX[x].0+isotope-SYMBOL_INDEX[x].1}
                     )
                   }
                   else{
                   return Err("Not a known isotope")
                   }
         None=> return Err("Not a known element")
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
     /** Takes proton and neutron argument, returns a tuple of approximate mass and binding energy. Allows for approximation of theorectical nuclides, using the mass model
     
        ```
        let sr90 = Nuclide::create(38,52);
                              // real computed value is 89.907 amu,782 MeV
        assert_eq!(sr90, (89.893471169, 776.39792))
        ```
     */
  pub  fn create(z: usize, n: usize)-> (f64,f64){
      let b_e =  mass_model(z+n,z);
     ( (z as f64*PROTONMASS + n as f64*NEUTRONMASS) - (b_e/931.36808885),b_e)
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
   ///Returns the element name.     
  pub fn element_name(&self)->String{
         NAME[self.atomic_num()-1].to_string()
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
    /// Mass deficit as MeV   
  pub fn mass_deficit_ev(&self)->f64{
       self.mass_deficit()*931.36808885
       }
       
  pub fn mass_deficit_j(&self)->f64{
      self.mass_deficit_ev()*1.602177E-19
      }     
       
       /*Lighter evaluation   A < 50
       14.9297A - 15.058A^(2/3) - 0.6615*(z^2)/A^(1/3) - 21.6091*((a-z^2)^2) 
       +- 10.1744*(1/sqrt(A))
      */
  ///Returns the binding energy. Utilizing the mass model
  pub  fn binding_energy(&self)->f64{
   let (z,a) = (self.proton_neutron().0,self.proton_neutron().0 + self.proton_neutron().1);
       mass_model(a,z )
   
       }
     
 pub fn binding_energy_j(&self)->f64{
      self.binding_energy()*1.602176634E-19
     }      
       /// Approximate neutron separation energy
  pub fn neutron_separation(&self)->f64{
  let (z,n) = self.proton_neutron();
      mass_model(z+n,z)-mass_model(z+n-1,z)
  }     
     /// Approximate proton separation energy
  pub fn proton_separation(&self)->f64{
  let (z,n) = self.proton_neutron();
      mass_model(z+n,z)-mass_model(z+n-1,z-1)
  }
      
}
       
       
