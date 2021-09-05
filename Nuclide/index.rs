 pub const NAME: [&str; 118] = [
 
       "Hydrogen"     , "Helium"       , "Lithium"      , "Berylium"      ,
       "Boron"        , "Carbon"       , "Nitrogen"     , "Oxygen"        , 
       "Flourine"     , "Neon"         , "Sodium"       , "Magnesium"     , 
       "Aluminium"    , "Silicon"      , "Phosphorus"   , "Sulfur"        , 
       "Chlorine"     , "Argon"        , "Potassium"    , "Calcium"       ,
       "Scandium"     , "Titanium"     , "Vanadium"     , "Chromium"      , 
       "Manganese"    , "Iron"         , "Cobalt"       , "Nickel"        ,
       "Copper"       , "Zinc"         , "Gallium"      , "Germanium"     ,
       "Arsenic"      , "Selenium"     , "Bromine"      , "Krypton"       ,
       "Rubidium"     , "Strontium"    , "Yttrium"      , "Zirconium"     , 
       "Niobium"      , "Molybdenum"   , "Technetium"   , "Ruthenium"     ,
       "Rhodium"      , "Palladium"    , "Silver"       , "Cadmium"       , 
       "Indium"       , "Tin"          , "Antimony"     , "Tellurium"     ,
       "Iodine"       , "Xenon"        , "Caesium"      , "Barium"        , 
       "Lathanum"     , "Cerium"       , "Praseodymium" , "Neodymium"     , 
       "Promethium"   , "Samarium"     , "Europium"     , "Gadolinium"    , 
       "Terbium"      , "Dysprosium"   , "Holmium"      , "Erbium"        , 
       "Thulium"      , "Ytterbium"    , "Lutetium"     , "Hafnium"       ,
       "Tantalum"     , "Tungsten"     , "Rhenium"      , "Osmium"        , 
       "Iridium"      , "Platinum"     , "Gold"         , "Mercury"       , 
       "Thallium"     , "Lead"         , "Bismuth"      , "Polonium"      , 
       "Astatine"     , "Radon"        , "Francium"     , "Radium"        ,
       "Actinium"     , "Thorium"      , "Protactinium" , "Uranium"       , 
       "Neptunium"    , "Plutonium"    , "Americium"    , "Curium"        , 
       "Berkelium"    , "Californium"  , "Einsteinium"  , "Fermium"       ,
       "Mendelevium"  , "Nobelium"     , "Lawrencium"   , "Rutherfordium" ,
       "Dubnium"      , "Seaborgium"   , "Bohrium"      , "Hassium"       ,
       "Meitnerium"   , "Darmstadtium" , "Roentgenium"  , "Copernicium"   , 
       "Nihonium"     , "Flerovium"    , "Moscovium"    , "Livermorium"   , 
       "Tennesine"    , "Oganesson"    ,
 
 ];




 pub const SYMBOL: [&str;118] = [
  
       "H"  , "He" , "Li" , "Be" , "B"  , "C"  , "N"  , "O"  , "F" ,
       "Ne" , "Na" , "Mg" , "Al" , "Si" , "P"  , "S"  , "Cl" , "Ar",
       "K"  , "Ca" , "Sc" , "Ti" , "V"  , "Cr" , "Mn" , "Fe" , "Co",
       "Ni" , "Cu" , "Zn" , "Ga" , "Ge" , "As" , "Se" , "Br" , "Kr",
       "Rb" , "Sr" , "Y"  , "Zr" , "Nb" , "Mo" , "Tc" , "Ru" , "Rh",
       "Pd" , "Ag" , "Cd" , "In" , "Sn" , "Sb" , "Te" , "I"  , "Xe",
       "Cs" , "Ba" , "La" , "Ce" , "Pr" , "Nd" , "Pm" , "Sm" , "Eu",
       "Gd" , "Tb" , "Dy" , "Ho" , "Er" , "Tm" , "Yb" , "Lu" , "Hf",
       "Ta" , "W"  , "Re" , "Os" , "Ir" , "Pt" , "Au" , "Hg" , "Tl",
       "Pb" , "Bi" , "Po" , "At" , "Rn" , "Fr" , "Ra" , "Ac" , "Th",
       "Pa" , "U"  , "Np" , "Pu" , "Am" , "Cm" , "Bk" , "Cf" , "Es",
       "Fm" , "Md" , "No" , "Lr" , "Rf" , "Db" , "Sg" , "Bh" , "Hs",
       "Mt" , "Ds" , "Rg" , "Cn" , "Nh" , "Fl" , "Mc" , "Lv" , "Ts",
       "Og" ,

];
//
pub const SYMBOL_INDEX : [(usize,usize,usize);118] = [
//          H                He               Li               Be       
     (   0,  1,  7) , (   7,  3, 10) , (  15,  3, 13) , (  26,  5, 16) ,
//          B                C                N                O
     (  38,  6, 21) , (  54,  8, 23) , (  70, 10, 25) , (  86, 11, 28) ,
//          F                Ne               Na               Mg
     ( 104, 13, 31) , ( 123, 15, 34) , ( 143, 17, 39) , ( 166, 19, 41) , 
//          Al               Si               P                S 
     ( 189, 21, 43) , ( 212, 22, 45) , ( 236, 24, 47) , ( 260, 26, 49) , 
//          Cl               Ar               K                Ca          
     ( 284, 28, 52) , ( 309, 29, 54) , ( 335, 31, 59) , ( 364, 33, 61) ,   
//          Sc               Ti               V                Cr
     ( 393, 35, 63) , ( 422, 37, 65) , ( 451, 39, 67) , ( 480, 41, 70) ,   
//          Mn               Fe               Co               Ni
     ( 510, 43, 73) , ( 541, 45, 76) , ( 573, 47, 78) , ( 605, 48, 82) ,   
//          Cu               Zn               Ga               Ge       
     ( 640, 52, 84) , ( 673, 54, 86) , ( 706, 56, 88) , ( 739, 58, 90) ,   
//          As               Se               Br               Kr
     ( 772, 60, 92) , ( 805, 63, 95) , ( 838, 65,101) , ( 875, 67,102) ,   
//          Rb               Sr               Y                Zr                 
     ( 911, 71,106) , ( 947, 73,108) , ( 983, 75,111) , (1020, 77,114) ,   
//          Nb               Mo               Tc               Ru          
     (1058, 79,117) , (1097, 81,119) , (1136, 83,122) , (1176, 85,125) ,                                  
//          Rh               Pd               Ag               Cd              
     (1217, 88,128) , (1258, 90,131) , (1300, 92,133) , (1342, 94,135) ,   
//          In               Sn               Sb               Te                           
     (1384, 96,137) , (1426, 99,140) , (1468,102,142) , (1509,104,145) ,    
//          I                Xe               Cs               Ba                        
     (1551,106,147) , (1593,108,150) , (1636,111,152) , (1678,113,154) ,   
//          La               Ce               Pr               Nd                                 
     (1720,116,157) , (1762,119,159) , (1803,121,161) , (1844,124,163) ,                                  
//          Pm               Sm               Eu               Gd                       
     (1884,126,165) , (1924,128,168) , (1965,130,170) , (2006,133,172) ,    
//          Tb               Dy               Ho               Er    
     (2046,135,174) , (2086,138,176) , (2125,140,178) , (2164,142,180) ,   
//          Tm               Yb               Lu               Hf            
     (2203,144,182) , (2242,148,185) , (2280,150,188) , (2319,153,190) ,       
//          Ta               W                Re               Os                                               
     (2357,155,194) , (2397,157,197) , (2438,159,199) , (2479,161,203) ,                      
//          Ir               Pt               Au               Hg          
     (2522,163,205) , (2565,165,208) , (2609,168,210) , (2652,170,216) ,   
//          Tl               Pb               Bi               Po  
     (2699,176,218) , (2742,178,220) , (2785,184,224) , (2826,186,227) ,    
//          At               Rn               Fr               Ra    
     (2868,191,229) , (2907,193,231) , (2946,197,233) , (2983,201,235) ,   
//          Ac               Th               Pa               U      
     (3018,205,237) , (3051,208,239) , (3083,211,241) , (3114,215,243) ,    
//          Np               Pu               Am               Cm
     (3143,219,252) , (3177,221,252) , (3209,223,252) , (3239,231,252) ,  
//          Bk               Cf               Es               Fm         
     (3261,233,254) , (3283,237,256) , (3303,239,258) , (3323,241,260) ,    
//          Md               No               Lr               Rf 
     (3343,244,262) , (3362,248,264) , (3379,251,266) , (3395,253,268) ,   
//          Db               Sg               Bh               Hs  
     (3411,255,270) , (3427,258,273) , (3443,260,278) , (3462,263,280) ,    
//          Mt               Ds               Rg               Cn 
     (3480,265,282) , (3498,267,284) , (3516,272,286) , (3531,276,288) ,   
//          Nh               Fl               Mc               Lv 
     (3544,278,290) , (3557,284,291) , (3565,287,292) , (3571,289,294) ,    
//          Ts              Og 
     (3577,291,294) , (3581,293,295) ,    

  
];
