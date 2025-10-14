use Nuclide::Lang;

#[test]
fn elements_have_valid_name_translations_in_polish() {
    //a few individual checks
    let sulfur = Nuclide::Element::from_protons(16);

    //using enum
    assert_eq!("Siarka", sulfur.element_name_by_lang(Lang::Pol));
    assert_eq!("Schwefel", sulfur.element_name_by_lang(Lang::De));
    assert_eq!("Sulfur", sulfur.element_name_by_lang(Lang::Eng));

    //using string representation
    assert_eq!("Siarka", sulfur.element_name_by_lang_str("pl"));
    assert_eq!("Schwefel", sulfur.element_name_by_lang_str("de"));
    assert_eq!("Sulfur", sulfur.element_name_by_lang_str("??"));

    //make sure no call for element's name with Lang::Pol or Lang::De panics
    for element in Nuclide::Element::iter() {
        element.element_name_by_lang(Lang::Pol);
        element.element_name_by_lang(Lang::De);
    }
}
