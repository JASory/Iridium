#[test]
fn elements_have_valid_name_translations_in_polish() {
    //a few individual checks
    let sulfur = Nuclide::Element::from_protons(16);

    assert_eq!("Siarka", sulfur.element_name_by_lang("pl"));
    assert_eq!("Schwefel", sulfur.element_name_by_lang("de"));
    assert_eq!("Sulfur", sulfur.element_name_by_lang("??"));

    //make sure no call for element's name with "pl" or 'de' lang panics
    for element in Nuclide::Element::iter() {
        element.element_name_by_lang("pl");
        element.element_name_by_lang("de");
    }
}
