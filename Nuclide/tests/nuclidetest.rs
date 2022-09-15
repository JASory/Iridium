use ::Nuclide::Atom;
use ::Nuclide::Nuclide;

/*
 Tests to perform 3
   Assignment
   Identity
   Deterministic decay
   Forced decay, each type


   Side tests to perform exhaustively
   decaymode
   from nucleons  Correct
   isotope
   isobar
   isotone
   identity

*/

const ISOTOPE_COUNT: [u8; 118] = [
    7, 9, 11, 12, 16, 16, 16, 18, 19, 20, 23, 23, 23, 24, 24, 24, 25, 26, 29, 29, 29, 29, 29, 30,
    31, 32, 32, 35, 33, 33, 33, 33, 33, 33, 37, 36, 36, 36, 37, 38, 39, 39, 40, 41, 41, 42, 42, 42,
    42, 42, 41, 42, 42, 43, 42, 42, 42, 41, 41, 40, 40, 41, 41, 40, 40, 39, 39, 39, 39, 38, 39, 38,
    40, 41, 41, 43, 43, 44, 43, 47, 43, 43, 41, 42, 39, 39, 37, 35, 33, 32, 31, 29, 34, 32, 30, 22,
    22, 20, 20, 20, 19, 17, 16, 16, 16, 16, 19, 18, 18, 18, 15, 13, 13, 8, 6, 6, 4, 3,
];

const _ISOTONE_COUNT: [u8; 179] = [
    3, 5, 6, 8, 9, 10, 11, 11, 13, 13, 14, 15, 16, 16, 17, 18, 19, 19, 19, 19, 21, 20, 20, 20, 21,
    21, 22, 23, 23, 23, 23, 23, 22, 21, 21, 22, 22, 22, 23, 24, 25, 25, 24, 24, 24, 24, 25, 25, 25,
    25, 25, 25, 25, 26, 27, 26, 26, 26, 25, 25, 24, 25, 25, 25, 26, 27, 28, 27, 27, 28, 28, 27, 28,
    28, 29, 29, 29, 28, 28, 29, 28, 29, 29, 30, 30, 31, 31, 31, 30, 30, 31, 30, 29, 29, 28, 28, 29,
    28, 27, 26, 26, 26, 26, 25, 24, 23, 24, 24, 23, 22, 22, 21, 20, 20, 19, 19, 19, 19, 19, 18, 19,
    19, 18, 19, 18, 17, 18, 19, 19, 18, 18, 17, 16, 16, 16, 17, 18, 17, 16, 16, 17, 18, 17, 18, 17,
    16, 16, 15, 15, 15, 15, 14, 14, 15, 15, 16, 17, 17, 15, 13, 11, 11, 10, 9, 9, 9, 8, 8, 7, 7, 8,
    8, 8, 8, 8, 8, 7, 6, 1,
];

const _ISOBAR_COUNT: [u8; 295] = [
    1, 2, 3, 3, 4, 5, 5, 5, 5, 6, 6, 6, 7, 6, 7, 7, 7, 7, 8, 8, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10,
    11, 10, 11, 11, 11, 11, 12, 12, 13, 12, 13, 12, 13, 12, 13, 12, 13, 13, 13, 12, 12, 13, 12, 13,
    12, 13, 13, 14, 14, 14, 14, 13, 14, 13, 14, 13, 14, 13, 13, 13, 13, 13, 14, 13, 14, 14, 14, 14,
    14, 14, 15, 15, 15, 15, 15, 15, 14, 15, 14, 15, 14, 15, 14, 15, 15, 15, 15, 15, 16, 16, 16, 16,
    15, 16, 16, 17, 16, 17, 16, 16, 17, 16, 17, 17, 16, 17, 17, 16, 17, 16, 17, 17, 16, 17, 17, 17,
    17, 18, 17, 18, 18, 17, 18, 17, 18, 17, 17, 17, 17, 18, 17, 18, 17, 18, 18, 17, 17, 17, 17, 18,
    17, 17, 17, 17, 17, 17, 18, 17, 18, 17, 18, 17, 18, 17, 18, 17, 17, 18, 17, 18, 17, 17, 16, 16,
    15, 16, 15, 16, 15, 15, 14, 14, 13, 14, 14, 14, 14, 14, 13, 13, 13, 13, 14, 14, 13, 13, 14, 13,
    13, 12, 13, 13, 13, 12, 13, 12, 12, 13, 12, 12, 12, 12, 12, 12, 13, 13, 12, 12, 12, 12, 12, 12,
    13, 13, 12, 12, 12, 11, 11, 10, 11, 10, 11, 10, 10, 9, 10, 9, 10, 9, 10, 9, 9, 9, 9, 9, 9, 10,
    10, 10, 11, 11, 8, 8, 8, 8, 7, 8, 7, 8, 7, 7, 7, 7, 7, 7, 7, 7, 6, 6, 5, 6, 6, 5, 5, 6, 6, 7,
    6, 6, 5, 5, 4, 5, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 1,
];

#[test]
fn decay_test() {
    let mut uranium = Nuclide::new("U-235").unwrap();
    uranium.alpha_emission(1);
    assert_eq!(uranium.identity(), "Th-231");
    uranium.proton_emission(2);
    assert_eq!(uranium.identity(), "Ra-229");
    uranium.neutron_emission(2);
    assert_eq!(uranium.identity(), "Ra-227");
    //assert_eq!();
    //assert_eq!();
}

#[test] // Verifies that proton-neutron and from_nucleons are correct inverses
fn initialization_test() {
    for i in 0..3585 {
        let nuclide = Nuclide::assign(i);
        let (z, n) = nuclide.proton_neutron();
        let testnuclide = Nuclide::from_nucleons(z, n).unwrap();
        assert_eq!(nuclide.nuclide_index(), testnuclide.nuclide_index());
        // Verifies that new and identity are inverses
        assert_eq!(
            nuclide.nuclide_index(),
            Nuclide::new(&nuclide.identity()).unwrap().nuclide_index()
        )
    }

    let mut count = 0u32; //Bruteforce check of from_nucleons. Ensures that only unique values are counted
    for z in 0..300 {
        // generously exceeds the limit of 118 protons
        for a in 0..300 {
            // Exceeds the limit of 178 neutrons as well
            match Nuclide::from_nucleons(z, a) {
                Some(_x) => count += 1,
                None => (),
            }
        }
    }

    assert_eq!(3585, count); // Total of 3585 nuclides means that this is correct
}

#[test] // verifies that isotope lists are accurate
fn isotope_test() {
    for i in 1..119 {
        let p = Nuclide::from_nucleons(i, (i as f64 * 1.5).round() as usize).unwrap();
        let isocount = p.isotope_list().len();
        assert_eq!(isocount as u8, ISOTOPE_COUNT[i - 1])
    }
    /*
    for i in 1..178{

      let p = Nuclide::from_nucleons((i as f64/1.5).ceil() as usize,i).unwrap();
     // println!("{}",p.identity());
      let isocount = p.isotone_list().len();
      assert_eq!(isocount as u8,ISOTONE_COUNT[i-1])
    }
    */
}

//#[ignore]
#[test]
fn mirror() {
    // Construct all mirrorable nuclides and show that the function correctly swaps the nucleons.
    // This does not test that the mirror function selects all mirrorable nuclides however

    let nuclides = Nuclide::list();
    let mut mirrors = vec![];
    let mut mirrorable = vec![];
    for atom in nuclides {
        match atom.mirror() {
            Some(mir) => {
                mirrors.push(mir);
                mirrorable.push(atom);
            }

            None => (),
        }
    }

    for (og, mir) in mirrorable.iter().zip(mirrors.iter()) {
        assert_eq!(*og, mir.mirror().unwrap());
        assert_eq!(
            og.proton_neutron(),
            (mir.proton_neutron().1, mir.proton_neutron().0)
        )
    }

    assert_eq!(mirrors.len(), 514); // Currently believed by the author to be 514 mirror nuclide pairs
}

/* Checks that the mass model meets the error bounds, binding energy is currently computed using the mass model
so mass_defect as EV is used instead. Note that this means that the binding energy error is not a comparison against
empirical values. The mass error however is, and one can see that the current model has -+ 0.2968 amu maximum error.

*/
#[test]
fn mass_model() {
    const MASS_ERROR: f64 = 0.29677;
    const BE_ERROR: f64 = 235.01;

    let nuclides = Nuclide::list(); // List of all nuclides

    for atom in nuclides.iter() {
        let (proton, neutron) = atom.proton_neutron();
        let (mass, be) = Nuclide::create(proton, neutron);

        let masshi = atom.am() + MASS_ERROR;
        let masslo = atom.am() - MASS_ERROR;

        let be_hi = atom.mass_deficit_ev() + BE_ERROR;
        let be_lo = atom.mass_deficit_ev() - BE_ERROR;

        assert!(mass < masshi && mass > masslo);
        assert!(be < be_hi && be > be_lo);
    }
}

/*
   Checks that all potential daughter nuclides are documented and exist within the nuclide dataset

   This will panic if there are decays that include undocumented nuclides. This will also detect any decay looping
   errors where an isotope has a mode set to decay back into it's mother resulting in an infinite loop
*/

#[test]
fn no_ghost_nuclide() {
    let nuclides = Nuclide::list();

    for atom in nuclides.iter() {
        if atom.half_life() == f64::INFINITY {
            // Skips nuclide if it is stable
            continue;
        }

        let mut radio_isotope = *atom;
        radio_isotope.decay(f64::MAX); // sufficient to decay any nuclide to a stable state
    }
}
