use ::Nuclide::element::{Element, NuclideFraction};

#[test]
fn elements_have_valid_nuclide_fractions() {
    ::Nuclide::Nuclide::new("S-34").expect("S-34 valid");

    for e in Element::iter() {
        dbg!(e);
        // No panics in NuclideFraction construction
        e.abundant_nuclides();
    }
}

// TODO more tests

