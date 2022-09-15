#[rustfmt::skip]
pub(crate) const NAME: [&str; 118] = [

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
       "Lanthanum"    , "Cerium"       , "Praseodymium" , "Neodymium"     , 
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

#[rustfmt::skip]
pub const SYMBOL: [&str; 118] = [
    
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
#[rustfmt::skip]  // (Start,A-min,A-max)
pub const SYMBOL_INDEX: [(usize, usize, usize); 118] = [
    
//          H                He               Li               Be       
     (   0,  1,  7) , (   7,  2, 10) , (  16,  3, 13) , (  27,  5, 16) ,
//          B                C                N                O
     (  39,  6, 21) , (  55,  8, 23) , (  71, 10, 25) , (  87, 11, 28) ,
//          F                Ne               Na               Mg
     ( 105, 13, 31) , ( 124, 15, 34) , ( 144, 17, 39) , ( 167, 19, 41) , 
//          Al               Si               P                S 
     ( 190, 21, 43) , ( 213, 22, 45) , ( 237, 24, 47) , ( 261, 26, 49) , 
//          Cl               Ar               K                Ca          
     ( 285, 28, 52) , ( 310, 29, 54) , ( 336, 31, 59) , ( 365, 33, 61) ,   
//          Sc               Ti               V                Cr
     ( 394, 35, 63) , ( 423, 37, 65) , ( 452, 39, 67) , ( 481, 41, 70) ,   
//          Mn               Fe               Co               Ni
     ( 511, 43, 73) , ( 542, 45, 76) , ( 574, 47, 78) , ( 606, 48, 82) ,   
//          Cu               Zn               Ga               Ge       
     ( 641, 52, 84) , ( 674, 54, 86) , ( 707, 56, 88) , ( 740, 58, 90) ,   
//          As               Se               Br               Kr
     ( 773, 60, 92) , ( 806, 63, 95) , ( 839, 65,101) , ( 876, 67,102) ,   
//          Rb               Sr               Y                Zr                 
     ( 912, 71,106) , ( 948, 73,108) , ( 984, 75,111) , (1021, 77,114) ,   
//          Nb               Mo               Tc               Ru          
     (1059, 79,117) , (1098, 81,119) , (1137, 83,122) , (1177, 85,125) ,                                  
//          Rh               Pd               Ag               Cd              
     (1218, 88,128) , (1259, 90,131) , (1301, 92,133) , (1343, 94,135) ,   
//          In               Sn               Sb               Te                           
     (1385, 96,137) , (1427, 99,140) , (1469,102,142) , (1510,104,145) ,    
//          I                Xe               Cs               Ba                        
     (1552,106,147) , (1594,108,150) , (1637,111,152) , (1679,113,154) ,   
//          La               Ce               Pr               Nd                                 
     (1721,116,157) , (1763,119,159) , (1804,121,161) , (1845,124,163) ,                                  
//          Pm               Sm               Eu               Gd                       
     (1885,126,165) , (1925,128,168) , (1966,130,170) , (2007,133,172) ,    
//          Tb               Dy               Ho               Er    
     (2047,135,174) , (2087,138,176) , (2126,140,178) , (2165,142,180) ,   
//          Tm               Yb               Lu               Hf            
     (2204,144,182) , (2243,148,185) , (2281,150,188) , (2320,153,190) ,       
//          Ta               W                Re               Os                                               
     (2358,155,194) , (2398,157,197) , (2439,159,199) , (2480,161,203) ,                      
//          Ir               Pt               Au               Hg          
     (2523,163,205) , (2566,165,208) , (2610,168,210) , (2653,170,216) ,   
//          Tl               Pb               Bi               Po  
     (2700,176,218) , (2743,178,220) , (2786,184,224) , (2827,186,227) ,    
//          At               Rn               Fr               Ra    
     (2869,191,229) , (2908,193,231) , (2947,197,233) , (2984,201,235) ,   
//          Ac               Th               Pa               U      
     (3019,205,237) , (3052,208,239) , (3084,211,241) , (3115,215,243) ,    
//          Np               Pu               Am               Cm
     (3144,219,252) , (3178,221,252) , (3210,223,252) , (3240,231,252) ,  
//          Bk               Cf               Es               Fm         
     (3262,233,254) , (3284,237,256) , (3304,239,258) , (3324,241,260) ,    
//          Md               No               Lr               Rf 
     (3344,244,262) , (3363,248,264) , (3380,251,266) , (3396,253,268) ,   
//          Db               Sg               Bh               Hs  
     (3412,255,270) , (3428,258,273) , (3444,260,278) , (3463,263,280) ,    
//          Mt               Ds               Rg               Cn 
     (3481,265,282) , (3499,267,284) , (3517,272,286) , (3532,276,288) ,   
//          Nh               Fl               Mc               Lv 
     (3545,278,290) , (3558,284,291) , (3566,287,292) , (3572,289,294) ,    
//          Ts              Og 
     (3578,291,294) , (3582,293,295) ,    
];
