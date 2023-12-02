/*
    Mass model computing the binding energy
    
    In: total nucleons, number of protons
    Out: Binding energy in MeV
*/

pub(crate) fn mass_model(a: usize, z: usize) -> f64 {
    
  const COEF : [f64;10] = [
        0.7043,17.7418,16.2562,37.5562,53.9017,
        0.4711,2.1307, 0.0210,40.5356,6.0632
        ];
        
  let mut y : [f64;2] = [0f64,0f64];
  let mut oei : [f64;2] = [0f64,0f64];
  let mut dei : [f64;2] = [0f64,0f64];
  let mut qx : [f64;2] = [0f64,0f64];
  let mut dx : [f64;2] = [0f64,0f64];    
  let mut pp : [f64;2] = [0f64,0f64];     
  let mut n2 : [f64;2] = [0f64,0f64];                           
  let mut op : [f64;2] = [0f64,0f64]; 
  let mut onp :  [[[f64;9];2];2] = [[[0f64;9];2];2];
  
  let n = a-z;
  let nuclei : [usize;2] = [n,z]; //= vec![n,z];
  
  //let n = a-z;
  let a = n+z;
  let t = n.abs_diff(z);
  let r = (a as f64).cbrt();
  let rc = r * (1.0-0.25*(t as f64/a as f64).powi(2));
  let ra = rc.powi(2)/r;
  let z2 = (z*(z-1)) as f64;
  let initial_term = (-z2 + 0.76 * z2.powf(2.0/3.0))/rc;
  for deformed in 0..2{
  
   let mut ju = 0f64;
   
    if deformed == 1{
      ju = 4f64
    }
    
    let mut term : [f64;10] = [0f64;10];
    let mut noc : [[f64;18];2] = [[0f64;18];2];
    let mut os : [f64;2] = [0f64,0f64]; 
                        
    term[0] = initial_term;
    
    for idx in 0..2{
      n2[idx] = (2*(nuclei[idx]/2)) as f64;
      let mut ncum = 0i64;
      let mut i = 0i64;
      let mut idd = 0i64;

        loop{
       
         i+=1;
         if i%2==1{
            idd = i+1;
         }
         else if i%2==0{
           idd = (i*(i-2))/4;
         
         }
         ncum+=idd;
       
            if ncum < nuclei[idx] as i64{
               noc[idx][(i-1) as usize] = idd as f64;
            }
            else if ncum >= nuclei[idx] as i64{
           
               break;
            }
         }
         
       let imax = i+1;
       let mut ip = (i-1)/2;
       let ipm = i/2;
       pp[idx] = ip as f64;
       let moc : f64 = nuclei[idx] as f64 - ncum as f64 + idd as f64;
       noc[idx][(i-1) as usize] = moc-ju;
       noc[idx][i as usize] = ju;
                           
       if i&1 == 1{
         oei[idx] = (moc as i64+ (ip as i64*(ip as i64-1i64))) as f64;
                
         dei[idx] = (ip*(ip+1)+2) as f64;
       }
       else if i&1 == 0{
         oei[idx] = ((moc as i64)-(ju as i64)) as f64;
         dei[idx] = ((ip+1)*(ip+2)+2) as f64;
       }
         
     
       qx[idx] = oei[idx]*(dei[idx]-oei[idx]-ju)/dei[idx];
       
       dx[idx] = qx[idx]*(2.0*oei[idx]-dei[idx]);
       if deformed == 1{
         qx[idx] = qx[idx]/dei[idx].sqrt();
       }
       
       if deformed == 1{
         for i in 1..(imax+1){
           let ip2 = (i-1)/2;
           onp[idx][0][ip2 as usize]=0f64;
           onp[idx][1][ip2 as usize]=0f64;
         }
     
       }
       
      
       for i in 1..(imax+1){
         ip = (i-1)/2;
         let fact = (((ip+1)*(ip+2)) as f64).sqrt();

         onp[idx][0][ip as usize]+=noc[idx][(i-1) as usize]/fact;
   
         let mut vm = -1.0;
         if i&1==1{
           vm=0.5* ip as f64;
         }
         onp[idx][1][ip as usize]+=noc[idx][(i-1) as usize]*vm;
       }

      op[idx] = 0f64;
      os[idx] = 0f64; 
       for ip in 0..(ipm+1){
         let den = (((ip+1)*(ip+2)) as f64).powf(3.0/2.0);
         let one = onp[idx][1][ip as usize];
         let zero = onp[idx][0][ip as usize];
       
         op[idx] = op[idx]+zero;
      
         os[idx] = os[idx] + one* (1.0+zero)* ((ip*ip) as f64/den) 
                   + one*(1.0-zero)*((4i64*ip as i64-5i64) as f64/den);
            
       }
    
       op[idx] = op[idx]*op[idx];
       } // end loop
      
       term[1] = op[0]+op[1];
       
       term[2] = -term[1]/ra;
       term[1] += os[0]+os[1];
       term[3] = -(t as f64)*(t as f64+2.0)/r.powi(2);
       term[4] = -term[3]/ra;
       
       if deformed == 0{
          term[5] = dx[0]+dx[1];
          term[6] = -term[5]/ra;
          let px = pp[0].sqrt()+pp[1].sqrt();
          term[7] = qx[0] * qx[1]*(2f64.powf(px));
       }
      else if deformed == 1{
        term[8] = qx[0]*qx[1];
      }
      term[4] = t as f64*(1.0-t as f64)/(a as f64*ra.powi(3)) + term[4];
      
    
      
      if(n2[0] != nuclei[0] as f64) && (n2[1] != nuclei[1] as f64){ term[9]= t as f64/a as f64;}                        
      if n > z{                                                          
        if(n2[0] == nuclei[0] as f64)&&(n2[1] != nuclei[1] as f64){term[9]= 1.0-t as f64/a as f64;}                    
        if(n2[0] != nuclei[0] as f64)&&(n2[1]==nuclei[1] as f64){term[9]= 1.0;}
        }                        
      else {                                                                     
        if(n2[0]==nuclei[0] as f64)&&(n2[1] != nuclei[1] as f64){term[9]= 1.0;}                        
        if(n2[0]!=nuclei[0] as f64)&&(n2[1]==nuclei[1] as f64){term[9]= 1.0-t as f64/a as f64;}                    
      }                                                                     
      if(n2[1]==nuclei[1] as f64) && (n2[0] == nuclei[0] as f64){term[9]= 2.0-t as f64/a as f64;}            
      
     
      for i in term[1..].iter_mut(){
         *i=*i/ra;
      }
      for i in 0..10{
         y[deformed]+=term[i]*COEF[i];     
       }
    }// end loop
    
    let de = y[1]-y[0];
    if de > 0f64 || z < 50{
      return y[0] 
    }  
    y[1]
  }
