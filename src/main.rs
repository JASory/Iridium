    
use Database::half_life::HALF_LIFE;
use Database::atomic_mass::ATOMIC_MASS;
use Database::index::SYMBOL;
use Database::index::SYMBOL_INDEX;
use Database::elemental::PAULING_ELECTRO;
use Database::elemental::THERMOCHEMICAL_ELECTRO_NEGATIVE;
use Database::elemental::ALLEN_ELECTRO;
use Database::elemental::COVALENT_RADII;
use Database::elemental::VAN_DER_WAAL_CRYSTAL;
use Database::elemental::VAN_DER_WAAL_ISOLATED;
use Database::elemental::IONIZATION_ENERGIES;
use Database::elemental::ELECTRON_AFFINITY;
use Database::decay_chain::DECAY_CHAIN;
use Database::update::table_print_f64;
use Database::update::table_print_u64;

use Database::update::decay_map;
use Database::update::update_vector_u64;
//use Database::elemental:: ;
//use crate::decay::Nuclide;

mod decay;


 fn nucleons_nuclide(x: &(usize, usize))-> usize{
    SYMBOL_INDEX[x.0-1].0 -(SYMBOL_INDEX[x.0 -1].1-x.0) + x.1 
 }

pub struct Nuclide{
   nuclide: usize,
}

impl Clone for Nuclide{
  fn clone(&self)->Nuclide{
  Nuclide{nuclide: self.nuclide.clone()}
  }
}

 //This function is expensive, so for modeling on a large scale clone the nuclide after initializing it. 
 //17.082583995s to initialize 1_000_000 nuclides of type Oganesson-294, this is unoptimized worst-case, cloning with optimized level 2 is only 3.3ms
 /*
   Do this 
    let oganesson_294 = Nuclide::new("Og",294); 
    let oganesson_one_million = vec![oganesson_294;1000000];
   Not this 
   let oganesson_one_million = (0..1000000).map(|_| Nuclide::new("Og",294)).collect::<Vec<Nuclide>>();
   
   Of course there are faster ways if you are willing to get out of your comfort zone. Directly cloning the nuclide index is only 1.7ms. For truly fast computation it
   is best to do away with OOP, and directly use the data. However this library focuses on ease of use and correctness. Just keep in mind compute once, clone or const whenever possible.
 
 */
 
impl Nuclide{

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
 
 
 
  pub  fn change(&mut self, x: usize){
          self.nuclide = x
     }
 
  pub  fn nuclide_index(&self)->usize{
          self.nuclide.clone()
     }
 
 
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
 
  pub  fn atomic_mass_dalton(&self)->f64{
          ATOMIC_MASS[self.nuclide]
       }
 
  pub  fn atomic_mass(&self)->f64{
          self.atomic_mass_dalton()*1.6605390666E-27
       }
 
  pub  fn isotope(&self)->(usize, usize){
          let element = self.atomic_num();
          (element, 
          (self.nuclide -SYMBOL_INDEX[element-1].0)+SYMBOL_INDEX[element-1].1
          )
       }
 
  pub  fn identity(&self)-> String{
          let iso = self.isotope();
          SYMBOL[iso.0-1].to_owned() + "-"+ &iso.1.to_string()
       }
 
  pub  fn proton_neutron(&self)->(usize, usize){
          (self.isotope().0,  self.isotope().1 - self.isotope().0)
       }
 
  pub  fn mass_deficit_dalton(&self)->f64{
          let nucleons = self.proton_neutron();
          (1.007276466621*nucleons.0 as f64 + 5.48579909065E-4*nucleons.0 as f64
           + 1.00866491588*nucleons.1 as f64)-self.atomic_mass_dalton()
       }
 
  pub  fn mass_deficit(&self)->f64{
       self.mass_deficit_dalton()* 1.6605390666E-27
       }
    
 //Result as electron volts, implement Myers Swiatecki semi-empirical equation
  pub  fn binding_energy(&self)->f64{
          self.mass_deficit_dalton()*931.49410242
       }
 
  pub  fn electron_affinity(&self)->f64{
          ELECTRON_AFFINITY[self.atomic_num()-1]
       }
  pub  fn electron_affinity_ev(&self)->f64{
          self.electron_affinity()*0.010364265
       }      
 
  pub  fn ionization_energies(&self, level: usize)->Result<f64, String>{
          if level > 0 && level < 4{
             return Ok(IONIZATION_ENERGIES[(self.atomic_num()-1)*3 + level-1])
          }
          else{
             return Err("Not a supported value".to_string())
          }
       }
       
  pub fn ionization_energies_ev(&self, level: usize)->Result<f64, String>{
         match self.ionization_energies(level){
               Ok(x)=> return Ok(x*0.010364265f64),
               Err(_)=> return Err("Not a supported value".to_string()),
         }
      }     
 
  pub  fn electronegativity(&self)->f64{
          THERMOCHEMICAL_ELECTRO_NEGATIVE[self.atomic_num()-1]
       }
 // kj/mol
  pub  fn mullikan_en(&self)->f64{
          (self.ionization_energies(1).unwrap() +       ELECTRON_AFFINITY[self.atomic_num()-1])* 1.97E-3 + 0.19
       }
 
  pub  fn allen_en(&self)->f64{
          ALLEN_ELECTRO[self.atomic_num()-1]
       }
 
  pub  fn pauling_en(&self)->f64{
          PAULING_ELECTRO[self.atomic_num()-1]
       }

  pub  fn covalent_radii(&self, bond: usize)->Result<f64, String>{
          if bond > 0 && bond < 4{
             return Ok(COVALENT_RADII[(self.atomic_num()-1)*3 + bond-1])
          }
          else{
            return Err("Not a tested bond type".to_string())
          }
 
       }
 
  pub  fn vdr_crystal(&self)->f64{
          VAN_DER_WAAL_CRYSTAL[self.atomic_num()-1]
       }
 
  pub  fn vdr_isolated(&self)->f64{
          VAN_DER_WAAL_ISOLATED[self.atomic_num()-1]
       }
 
 
  pub  fn half_life(&self)->f64{
          HALF_LIFE[self.nuclide]
       }
 
  pub fn mean_lifetime(&self)->f64{   //reciprocal of ln(2) average lifespan of a particle
         self.half_life()*1.4426950408889634f64
      }
     
     //approximation of decay constant
  pub  fn decay_constant(&self)->f64{
          self.mean_lifetime().recip()
       }
 
 //lowest probability is 1/u64::MAX
  pub  fn decay_time(&self, time: f64)->bool{
          let  prob =((1.0- (-self.decay_constant()*time).exp()) * 1.8446744073709551616E+19) as u64;

           if prob > rand(){
              return true 
           }
           else{
              return false
           }
       }

 

}
//         Not portable as it calls the RDRAND instruction set for hardware RNG, very high quality. Not secure for cryptography and fairly slow,but eliminates crate dependency. If there is an error then just import an rng that produces integers in the interval [0;2^64-1] and recompile. Should work on AMD and Intel though, as it has been supported since 2015. 
fn rand()->u64{
   let mut x: u64 = 0;
  let k = unsafe{core::arch::x86_64::_rdrand64_step(&mut x)};
x
}

//*1.66053906660E-27
fn  main(){ 
/*
let mut decaychain = DECAY_CHAIN.iter().map(|x| *x).collect::<Vec<u64>>();

let test = decay_map(1.0," ",0," ", 0,
                     1.0," ",0," ", 0,
                     1.0," ",0," ", 0, 
                     1.0," ",0," ", 0,
                     1.0," ",0," ", 0
                     );

  update_vector_u64(&mut decaychain[..],&test[..],Nuclide::new("Nd",146).unwrap().nuclide_index()*10);

  table_print_u64(decaychain, 5, "DECAY_CHAIN", "decay_chain.rs");

*/
/*
for i in 15..35{
let k = Nuclide::new("Ne",i).unwrap();
println!(" Atomic Number: {} Half-life {} atomic mass {}",i,k.half_life(),k.atomic_mass_dalton());
}*/
let start = std::time::Instant::now();
 let mut protactinium= Nuclide::new("Po",192).unwrap();
 let carbon = Nuclide::new("C",12).unwrap();
let mut nuclear_vector = vec![protactinium;1000];

let energies = nuclear_vector.iter_mut().map(|x| x.decay(1.0E+25).0).sum::<f64>();
let transform = nuclear_vector.iter_mut().map(|x| x.identity() ).collect::<Vec<String>>();
//println!("{}",berylium)
 println!(" Daughter : {:?}  mass {}  halflife{}",transform,carbon.atomic_mass_dalton(), carbon.nuclide_index())
 

  
}
