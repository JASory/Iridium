// Used to write and update the data arrays. 

use crate::index::SYMBOL;
use crate::index::SYMBOL_INDEX;


//Formatting for f64
 fn formatstyle_f64(x: f64, index_length: usize)->String{

       let mut k = String::new();
              if x.is_nan() {
                k = "f64::NAN".to_string();
              }
             else if !x.is_finite(){
               k= "f64::INFINITY".to_string();
             }
            else if x == 0f64{
              k="0f64".to_string();
             }
            else{
             k=x.to_string()+"f64";
            }
            
           (0..(index_length-k.len())).map(|_|" ").collect::<String>() + &k + " , "

      }
      
   fn   formatstyle_u64(x: &u64)-> String{
        let mut k = String::new();
        
         k = x.to_string();
         (0..(20-k.len())).map(|_|" ").collect::<String>() + &k + " , "
         }
         
      //Formatting for u8 tuple
 fn stringstyle()->String{

        "(0u8, 99u8, 99u8) , ".to_string()

}

// opens and writes file
 fn note(x:&str,y:&str){
   use std::fs::File;
   use std::io::Write;
   
  let mut file = File::create(y).expect("Whoopsie, ran out of paper . . .");

   println!("Wrote to {} in the local directory",y );
   file.write_all(x.as_bytes()).expect("Whoopsie ran out of ink . . .");
}
/*
   Formatting styles tablename: (index length, column count) 
   HALF_LIFE : (35,4), ATOMIC_MASS : (17,5), (50,3) for Kilogram Elemental arrays of length 118: (7;6)


*/


 pub fn table_string_f64(data: Vec<f64>, index_length: usize, col_count: usize, name: &str)-> String{
  let mut t = data.iter().map(|x| formatstyle_f64(*x,index_length)).collect::<Vec<String>>();
       
   for i in 0..t.len(){
       if i%col_count == col_count-1{
        t[i]+="\n  "
       }
   }
  let k = t.iter().map(|x| x.clone()).collect::<String>();
  "pub const ".to_owned() + name +" : [f64;" + &data.len().to_string() + "] =    [ \n \n  " + &k + "\n \n ];"   
 }
 //
 pub fn table_string_u64(data: Vec<u64>, col_count: usize, name: &str)->String{
 
 let mut t = data.iter().map(|x| formatstyle_u64(x)).collect::<Vec<String>>();
 for i in 0..t.len(){
       if i%col_count == col_count-1{
        t[i]+="\n  "
       }
   }
  let k = t.iter().map(|x| x.clone()).collect::<String>();
  "pub const ".to_owned() + name +" : [u64;" + &data.len().to_string() + "] =    [ \n \n  " + &k + "\n \n ];"   
 
 }

  //prints data vector with spaces of index_length and columns = col_count, some useful ones are Half-Life = 35, 4 and atomic mass 15,5 
pub  fn table_print_f64(data: Vec<f64>, index_length: usize,col_count: usize, name: &str, output: &str){
 let table = table_string_f64(data, index_length, col_count, name);
  
 note(&table,output)
}

pub fn table_print_u64(data: Vec<u64>, col_count: usize, name: &str, output: &str){
      let table = table_string_u64(data, col_count, name);
      note(&table, output)
  }


// changes the value at the index for a vector x
 fn update(x: &mut[f64],value:f64 , index: usize){

    x[index]=value

   }

//changes the value for all in x between index and the length of the updating vector, a superior version of update
pub fn update_vector(x: &mut[f64], value: &[f64],index:usize){
let start_slice =  &mut x[index..(value.len()+index)];

for (i,j) in start_slice.iter_mut().zip(value.iter()){

*i=*j;
}
}

pub fn update_vector_u64(x : &mut[u64], value: &[u64], index: usize){
let start_slice =  &mut x[index..(value.len()+index)];

for (i,j) in start_slice.iter_mut().zip(value.iter()){

*i=*j;
}
}


//Finds the nuclide index to start from, this is identical to Nuclide::new

fn bounds_check(x: &str, isotope:usize)->Result<usize, String>{

  match SYMBOL.iter().position(|y| y ==&x){
        Some(x)=> if isotope >= SYMBOL_INDEX[x].1 &&  isotope <= SYMBOL_INDEX[x].2 {
                     return Ok(SYMBOL_INDEX[x].0+isotope-SYMBOL_INDEX[x].1)
                   }
                   else{
                   return Err("Not a known isotope".to_string())
                   }
       None=> return Err("Not a known element".to_string())
       }
}

pub const PROTON_DECAY_MASK     : u16 = 8192 ; // 1
pub const NEUTRON_DECAY_MASK    : u16 = 16384; // 2
pub const BETA_DECAY_MASK       : u16 = 24576; // 3 24576
pub const POSITRON_DECAY_MASK   : u16 = 32768; // 4 32768
pub const ELECTRON_CAPTURE_MASK : u16 = 40960; // 5    Particle 5 == Electron Neutrino
pub const ALPHA_DECAY_MASK      : u16 = 49152; // 6
pub const CLUSTER_DECAY_MASK    : u16 = 57344; // 7  (7,14) is a particle of C-14



 fn join(typ : &str, number: u16)->u64{
    match typ{
    "P" => return (PROTON_DECAY_MASK + number ) as u64,
    "N" => return (NEUTRON_DECAY_MASK + number ) as u64,
    "B" => return (BETA_DECAY_MASK + number) as u64,
    "Pos"=> return (POSITRON_DECAY_MASK + number) as u64,
    "EC"=> return (ELECTRON_CAPTURE_MASK + number) as u64,
    "A" => return (ALPHA_DECAY_MASK + number) as u64,
    "CD"=> return (CLUSTER_DECAY_MASK + number) as u64,
    _=> return 0u64,
    }
 }


pub  fn decay_map(prob_1: f64, decay_type_1: &str, number_1: u16, decay_12: &str, numar  : u16,
                  prob_2: f64, decay_type_2: &str, number_2: u16, decay_22: &str, numar_2: u16,
                  prob_3: f64, decay_type_3: &str, number_3: u16, decay_32: &str, numar_3: u16,
                  prob_4: f64, decay_type_4: &str, number_4: u16, decay_42: &str, numar_4: u16,
                  prob_5: f64, decay_type_5: &str, number_5: u16, decay_52: &str, numar_5: u16
 )-> Vec<u64>{
 
// println!("{}", (join(decay_12,numar)<<16) + join(decay_type_1, number_1));
 let mut array = vec![0u64;10];
 array[0] = (prob_1*18446744073709551615f64).round() as u64;
 array[1] = (prob_2*18446744073709551615f64).round() as u64;
 array[2] = (prob_3*18446744073709551615f64).round() as u64;
 array[3] = (prob_4*18446744073709551615f64).round() as u64;
 array[4] = (prob_5*18446744073709551615f64).round() as u64;
 array[5] = join(decay_type_1, number_1) + (join(decay_12,numar)<<16)    ;
 array[6] = join(decay_type_2, number_2) + (join(decay_22,numar_2)<<16)  ;
 array[7] = join(decay_type_3, number_3) + (join(decay_32,numar_3)<<16)  ;
 array[8] = join(decay_type_4, number_4) + (join(decay_42,numar_4)<<16)  ;
 array[9] = join(decay_type_5, number_5) + (join(decay_52,numar_5)<<16)  ;
 array
 }

