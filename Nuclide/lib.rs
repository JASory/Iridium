//!  This crate is experimental and will have many breaks. For physics research projects use GEANT4. Otherwise the data here will meet or exceed that of most other libraries
            pub mod nuclide;
            
                mod electro;
                mod radii;
                mod index    ;
                mod update   ;
                mod half_life;
                mod decay    ;
                mod decay_chain;
                mod atomic_mass;
                mod elemental  ;
                mod ionization ;
                mod spinparity ;
                mod particle   ;
                use nuclide::Nuclide;
