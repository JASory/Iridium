/*


*/


/// Languages
///
/// English Spanish,French, Russian, German, and Polish are supported
#[repr(u8)]
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Lang{
   En, 
   Es,
   Fr,
   Ru,
   Pl,
   De
}


impl std::str::FromStr for Lang{

     type Err = &'static str;
     
  fn from_str(input: &str) -> Result<Self,Self::Err>{
     let x = input.to_lowercase();
     
     match x.as_str(){
     "en" => Ok(Self::En),
     "english" => Ok(Self::En),
     "es" => Ok(Self::Es),
     "español" => Ok(Self::Es), // Native language 
     "espanol" => Ok(Self::Es), // Native language using English keyboard
     "fr" => Ok(Self::Fr),
     "français" => Ok(Self::Fr),
     "francais" => Ok(Self::Fr), // With English keyboard
     "pl" => Ok(Self::Pl),
     "polski" => Ok(Self::Pl),
     "de" => Ok(Self::De),
     "deutsch" => Ok(Self::De), 
     "ru" => Ok(Self::Ru),
     "русский" => Ok(Self::Ru),
     _=> Err("Not a supported language; Try using ISO-639-1:2002 notation")
     }
  }  
}


#[rustfmt::skip]
pub(crate) const NAME_EN: [&str; 118] = [

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
pub(crate) const NAME_ES : [&str; 118] = [

       "Hidrógeno"    , "Helio"        , "Litio"        , "Berilio"      ,
       "Boro"         , "Carbono"      , "Nitrógeno"    , "Oxigeno"        , 
       "Flúor"        , "Neón"         , "Sodio"        , "Magnesio"     , 
       "Aluminio"     , "Silicio"      , "Fósforo"      , "Azufre"        , 
       "Cloro"        , "Argón"        , "Potasio"      , "Calcio"       ,
       "Escandio"     , "Titanio"      , "Vanadio"      , "Cromo"      , 
       "Manganeso"    , "Hierro"       , "Cobalto"      , "Niquel"        ,
       "Cobre"        , "Zinc"         , "Gallio"       , "Germanio"     ,
       "Arsénico"     , "Selenio"      , "Bromo"        , "Kriptón"       ,
       "Rubidio"      , "Estroncio"    , "Ytrio"        , "Zirconio"     , 
       "Niobio"       , "Molibdeno"    , "Tecnecio"     , "Rutenio"     ,
       "Rodio"        , "Palladio"     , "Palta"        , "Cadmio"       , 
       "Indio"        , "Estaño"       , "Antimonio"    , "Telurio"     ,
       "Yodo"         , "Xenón"        , "Cesio"        , "Bario"        , 
       "Lantano"      , "Cerio"        , "Praseodimio"  , "Neodimio"     , 
       "Prometio"     , "Samario"      , "Europio"      , "Gadolinio"    , 
       "Terbio"       , "Disprosio"    , "Holmio"       , "Erbio"        , 
       "Tulio"        , "Yterbio"      , "Lutecio"      , "Hafnio"       ,
       "Tantalio"     , "Tungsteno"    , "Renio"        , "Osmio"        , 
       "Iridio"       , "Platino"      , "Oro"          , "Mercurio"     , 
       "Talio"        , "Plomo"        , "Bismuto"      , "Polonio"      , 
       "Astato"       , "Radón"        , "Francio"      , "Radio"        ,
       "Actinio"      , "Torio"        , "Protactinio"  , "Uranio"       , 
       "Neptunio"     , "Plutonio"     , "Americio"     , "Curio"        , 
       "Berkelio"     , "Californio"   , "Einsteinio"   , "Fermio"       ,
       "Mendelevio"   , "Nobelio"      , "Laurencio"    , "Rutherfordio" ,
       "Dubnium"      , "Seaborgio"    , "Bohrio"       , "Hassio"       ,
       "Meitnerio"    , "Darmstadtio"  , "Roentgenio"   , "Copernicio"   , 
       "Nihonio"      , "Flerovio"     , "Moscovio"     , "Livermorio"   , 
       "Teneso"       , "Oganesón"     ,
];

#[rustfmt::skip]
pub(crate) const NAME_FR: [&str; 118] = [

       "Hydrogène"    , "Hélium"       , "Lithium"      , "Béryllium"     ,
       "Bore"         , "Carbone"      , "Azote"        , "Oxygène"       , 
       "Fluor"        , "Néon"         , "Sodium"       , "Magnésium"     , 
       "Aluminium"    , "Silicium"     , "Phosphore"    , "Soufre"        , 
       "Chlore"       , "Argon"        , "Potassium"    , "Calcium"       ,
       "Scandium"     , "Titane"       , "Vanadium"     , "Chrome"        , 
       "Manganèse"    , "Fer"          , "Cobalt"       , "Nickel"        ,
       "Cuivre"       , "Zinc"         , "Gallium"      , "Germanium"     ,
       "Arsenic"      , "Sélénium"     , "Brome"        , "Krypton"       ,
       "Rubidium"     , "Strontium"    , "Yttrium"      , "Zirconium"     , 
       "Niobium"      , "Molybdène"    , "Technétium"   , "Ruthénium"     ,
       "Rhodium"      , "Palladium"    , "Argent"       , "Cadmium"       , 
       "Indium"       , "Étain"        , "Antimoine"    , "Tellure"       ,
       "Iode"         , "Xénon"        , "Césium"       , "Baryum"        , 
       "Lanthane"     , "Cérium"       , "Praséodyme"   , "Néodyme"     , 
       "Prométhium"   , "Samarium"     , "Europium"     , "Gadolinium"    , 
       "Terbium"      , "Dysprosium"   , "Holmium"      , "Erbium"        , 
       "Thulium"      , "Ytterbium"    , "Lutécium"     , "Hafnium"       ,
       "Tantale"      , "Tungsténe"    , "Rhenium"      , "Osmium"        , 
       "Iridium"      , "Platine"      , "Or"           , "Mercure"       , 
       "Thallium"     , "Plomb"        , "Bismuth"      , "Polonium"      , 
       "Astate"       , "Radon"        , "Francium"     , "Radium"        ,
       "Actinium"     , "Thorium"      , "Protactinium" , "Uranium"       , 
       "Neptunium"    , "Plutonium"    , "Américium"    , "Curium"        , 
       "Berkélium"    , "Californium"  , "Einsteinium"  , "Fermium"       ,
       "Mendélévium"  , "Nobélium"     , "Lawrencium"   , "Rutherfordium" ,
       "Dubnium"      , "Seaborgium"   , "Bohrium"      , "Hassium"       ,
       "Meitnérium"   , "Darmstadtium" , "Roentgenium"  , "Copernicium"   , 
       "Nihonium"     , "Flérovium"    , "Moscovium"    , "Livermorium"   , 
       "Tennesse"     , "Oganesson"    ,
];



#[rustfmt::skip]
pub(crate) const NAME_PL: [&str; 118] = [

       "Wodór"     , "Hel"         , "Lit"        , "Beryl"       ,
       "Bor"       , "Węgiel"      , "Azot"       , "Tlen"        ,
       "Fluor"     , "Neon"        , "Sód"        , "Magnez"      ,
       "Glin"      , "Krzem"       , "Fosfor"     , "Siarka"      ,
       "Chlor"     , "Argon"       , "Potas"      , "Wapń"        ,
       "Skand"     , "Tytan"       , "Wanad"      , "Chrom"       ,
       "Mangan"    , "Żelazo"      , "Kobalt"     , "Nikiel"      ,
       "Miedź"     , "Cynk"        , "Gal"        , "German"      ,
       "Arsen"     , "Selenium"    , "Bromine"    , "Krypton"     ,
       "Rubid"     , "Stront"      , "Itr"        , "Cyrkon"      ,
       "Niob"      , "Molibden"    , "Technet "   , "Ruten"       ,
       "Rod"       , "Pallad"      , "Srebro"     , "Kadm"        ,
       "Ind"       , "Cyna"        , "Antymon"    , "Tellur"      ,
       "Jod"       , "Ksenon"      , "Cez"        , "Bar"         ,
       "Lantan"    , "Cer"         , "Prazeodym"  , "Neodym"      ,
       "Promet"    , "Samar"       , "Europ"      , "Gadolin"     ,
       "Terb"      , "Dysproz"     , "Holm"       , "Erb"         ,
       "Thul"      , "Iterb"       , "Lutet"      , "Hafn"        ,
       "Tantal"    , "Wolfram"     , "Ren"        , "Osm "        ,
       "Iryd"      , "Platyna"     , "Złoto"      , "Rtęć"        ,
       "Tal"       , "Ołów"        , "Bizmut"     , "Polon"       ,
       "Astat"     , "Radon"       , "Frans"      , "Rad"         ,
       "Aktyn"     , "Tor"         , "Protaktyn"  , "Uran"        ,
       "Neptun"    , "Pluton"      , "Ameryk"     , "Kiur"        ,
       "Berkel"    , "Kaliforn"    , "Einstein"   , "Ferm"        ,
       "Mendelew"  , "Nobel"       , "Lorens"     , "Rutherford"  ,
       "Dubn"      , "Seaborg"     , "Bohr"       , "Has"         ,
       "Meitner"   , "Darmsztadt"  , "Roentgen"   , "Kopernik"    ,
       "Nihon"     , "Flerow"      , "Moscow"     , "Liwermor"    ,
       "Tenes"     , "Oganeson"    ,
];

#[rustfmt::skip]
pub(crate) const NAME_DE: [&str; 118] = [

       "Hydrogen"    , "Helium"       , "Lithium"      , "Beryllium"     ,
       "Bor"         , "Kohlenstoff"  , "Stickstoff"   , "Sauerstoff"    ,
       "Fluor"       , "Neon"         , "Natrium"      , "Magnesium"     ,
       "Aluminium"   , "Silicium"     , "Phosphor"     , "Schwefel"      ,
       "Chlor"       , "Argon"        , "Kalium"       , "Calcium"       ,
       "Scandium"    , "Titan"        , "Vanadium"     , "Chrom"         ,
       "Mangan"      , "Eisen"        , "Cobalt"       , "Nickel"        ,
       "Kupfer"      , "Zink"         , "Gallium"      , "Germanium"     ,
       "Arsen"       , "Selen"        , "Brom"         , "Krypton"       ,
       "Rubidium"    , "Strontium"    , "Yttrium"      , "Zirkonium"     ,
       "Niob"        , "Molybdän"     , "Technetium"   , "Ruthenium"     ,
       "Rhodium"     , "Palladium"    , "Silber"       , "Cadmium"       ,
       "Indium"      , "Zinn"         , "Antimon"      , "Tellur"        ,
       "Iod"         , "Xenon"        , "Cesium"       , "Barium"        ,
       "Lanthan"     , "Cer"          , "Praseodym"    , "Neodym"        ,
       "Promethium"  , "Samarium"     , "Europium"     , "Gadolinium"    ,
       "Terbium"     , "Dysprosium"   , "Holmium"      , "Erbium"        ,
       "Thulium"     , "Ytterbium"    , "Lutetium"     , "Hafnium"       ,
       "Tantal"      , "Wolfram"      , "Rhenium"      , "Osmium"        ,
       "Iridium"     , "Platin"       , "Gold"         , "Quecksilber"   ,
       "Thallium"    , "Blei"         , "Bismut"       , "Polonium"      ,
       "Astat"       , "Radon"        , "Francium"     , "Radium"        ,
       "Actinium"    , "Thorium"      , "Protactinium" ,  "Uran"         ,
       "Neptunium"   , "Plutonium"    , "Americium"    , "Curium"        ,
       "Berkelium"   , "Californium"  , "Einsteinium"  , "Fermium"       ,
       "Mendelevium" , "Nobelium"     , "Lawrencium"   , "Rutherfordium" ,
       "Dubnium"     , "Seaborgium"   , "Bohrium"      , "Hassium"       ,
       "Meitnerium"  , "Darmstadtium" , "Roentgenium"  , "Copernicium"   ,
       "Nihonium"    , "Flerovium"    , "Moscovium"    , "Livermorium"   ,
       "Tennessine"  , "Oganesson"
];

#[rustfmt::skip]
pub(crate) const NAME_RU: [&str; 118] = [

       "Водород"      , "Гелий"        , "Литий"        , "Бериллий"      ,
       "Бор"          , "Углерод"      , "Азот"         , "Кислород"      , 
       "Фтор"         , "Неон"         , "Натрий"       , "Магний"        , 
       "Алюминий"     , "Кремний"      , "Фосфор"       , "Сера"          , 
       "Хлор"         , "Аргон"        , "Калий"        , "Кальций"       ,
       "Скандий"      , "Титан"        , "Ванадий"      , "Хром"          , 
       "Марганец"     , "Железо"       , "Кобальт"      , "Никель"        ,
       "Медь"         , "Цинк"         , "Галлий"       , "Германий"      ,
       "Мышьяк"       , "Селен"        , "Бром"         , "Криптон"       ,
       "Рубидий"      , "Стронций"     , "Иттрий"       , "Цирконий"      , 
       "Ниобий"       , "Молибден"     , "Технеций"     , "Рутений"       ,
       "Родий"        , "Палладий"     , "Серебро"      , "Кадмий"        , 
       "Индий"        , "Олово"        , "Сурьма"       , "Теллур"        ,
       "Иод"          , "Ксенон"       , "Цезий"        , "Барий"         , 
       "Лантан"       , "Церий"        , "Празеодим"    , "Неодим"        , 
       "Прометий"     , "Самарий"      , "Европий"      , "Гадолиний"     , 
       "Тербий"       , "Диспрозий"    , "Гольмий"      , "Эрбий"         , 
       "Тулий"        , "Иттербий"     , "Лютеций"      , "Гафний"        ,
       "Тантал"       , "Вольфрам"     , "Рений"        , "Осмий"         , 
       "Иридий"       , "Платина"      , "Золото"       , "Ртуть"         , 
       "Таллий"       , "Свинец"       , "Висмут"       , "Полоний"       , 
       "Астат"        , "Радон"        , "Франций"      , "Радий"         ,
       "Актиний"      , "Торий"        , "Протактиний"  , "Уран"          , 
       "Нептуний"     , "Плутоний"     , "Америций"     , "Кюрий"         , 
       "Берклий"      , "Калифорний"   , "Эйнштейний"   , "Фермий"        ,
       "Менделевий"   , "Нобелий"      , "Лоуренсий"    , "Резерфордий"   ,
       "Дубний"       , "Сиборгий"     , "Борий"        , "Хассий"        ,
       "Мейтнерий"    , "Дармштадтий"  , "Рентгений"    , "Коперниций"    , 
       "Нихоний"      , "Флеровий"     , "Московий"     , "Ливерморий"    , 
       "Теннессин"    , "Оганесон"     ,
];
