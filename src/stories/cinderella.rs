// A1

use crate::classes::{Word, WordClass, Language, Level, VerbForms, NounForms, PronounForms, AdjectiveForms, NumeralForms};

use std::collections::HashMap;

pub struct Vocabulary(HashMap<String, Word>);
impl Vocabulary {
    pub fn dutch() -> Self {
        let language = Language::Dutch;
        Vocabulary(HashMap::from([
            ("er".to_string(), Word::new("er", "Introductory word meaning 'there'", "there", "Er staat een huis in het bos.", WordClass::Adverb, language)),
            ("was".to_string(), Word::new("was", "Past tense of zijn", "was", "Het was een koude dag.", WordClass::Verb(VerbForms::new("zijn","ben","bent","is","zijn","was","was","was","waren","geweest")), language)),
            ("eens".to_string(), Word::new("eens", "Once (storytelling)", "once", "Lang eens geleden leefde een koning.", WordClass::Adverb, language)),
            ("een".to_string(), Word::new("een", "Indefinite article", "a", "Ik zie een vogel.", WordClass::Article, language)),
            ("meisje".to_string(), Word::new("meisje", "Young girl", "girl", "Het meisje speelt buiten.", WordClass::Noun(NounForms { plural: "meisjes".into(), diminutive: Some("meisje".into()) }), language)),
            ("haar".to_string(), Word::new("haar", "Possessive pronoun", "her", "Haar jas ligt op de stoel.", WordClass::Pronoun(PronounForms { subject_form: Some("zij".into()), object_form: Some("haar".into()), possessive_form: Some("haar".into()), reflexive_form: None }), language)),
            ("naam".to_string(), Word::new("naam", "Name", "name", "Mijn naam is Anna.", WordClass::Noun(NounForms { plural: "namen".into(), diminutive: None }), language)),
            ("cinderella".to_string(), Word::new("Cinderella", "Proper name", "Cinderella", "Cinderella droomde van een beter leven.", WordClass::Noun(NounForms { plural: "Cinderellas".into(), diminutive: None }), language)),
            ("zij-she-".to_string(), Word::new("zij", "She", "she", "Zij leest een boek.", WordClass::Pronoun(PronounForms { subject_form: Some("zij".into()), object_form: Some("haar".into()), possessive_form: Some("haar".into()), reflexive_form: Some("zich".into()) }), language)),
            ("zij-they-".to_string(), Word::new("zij", "They", "they", "Zij spelen in de tuin.", WordClass::Pronoun(PronounForms { subject_form: Some("zij".into()), object_form: Some("hen".into()), possessive_form: Some("hun".into()), reflexive_form: Some("zich".into()) }), language)),
            ("woonde".to_string(), Word::new("woonde", "Lived", "lived", "Hij woonde vroeger in Amsterdam.", WordClass::Verb(VerbForms::new("wonen","woon","woont","woont","wonen","woonde","woonde","woonde","woonden","gewoond")), language)),
            ("met".to_string(), Word::new("met", "With", "with", "Ik ga met mijn vriend naar school.", WordClass::Preposition, language)),
            ("stiefmoeder".to_string(), Word::new("stiefmoeder", "Step-mother", "stepmother", "De stiefmoeder keek boos.", WordClass::Noun(NounForms { plural: "stiefmoeders".into(), diminutive: None }), language)),
            ("en".to_string(), Word::new("en", "And", "and", "Brood en kaas liggen op tafel.", WordClass::Conjunction, language)),
            ("twee".to_string(), Word::new("twee", "Number two", "two", "Ik heb twee handen.", WordClass::Numeral(NumeralForms { ordinal: "tweede".into() }), language)),
            ("stiefzussen".to_string(), Word::new("stiefzussen", "Step-sisters", "stepsisters", "De stiefzussen lachten hard.", WordClass::Noun(NounForms { plural: "stiefzussen".into(), diminutive: None }), language)),
            ("waren".to_string(), Word::new("waren", "Past plural of zijn", "were", "De bloemen waren mooi.", WordClass::Verb(VerbForms::new("zijn","ben","bent","is","zijn","was","was","was","waren","geweest")), language)),
            ("niet".to_string(), Word::new("niet", "Not", "not", "Ik begrijp het niet.", WordClass::Adverb, language)),
            ("lief".to_string(), Word::new("lief", "Kind/nice", "kind", "De hond is lief.", WordClass::Adjective(AdjectiveForms { comparative: "liever".into(), superlative: "liefst".into() }), language)),
            ("voor".to_string(), Word::new("voor", "For", "for", "Dit cadeau is voor jou.", WordClass::Preposition, language)),
            ("moest".to_string(), Word::new("moest", "Had to", "had to", "Ik moest vroeg opstaan.", WordClass::Verb(VerbForms::new("moeten","moet","moet","moet","moeten","moest","moest","moest","moesten","gemoeten")), language)),
            ("elke".to_string(), Word::new("elke", "Every", "every", "Elke ochtend drink ik koffie.", WordClass::Determiner, language)),
            ("dag".to_string(), Word::new("dag", "Day", "day", "Vandaag is een mooie dag.", WordClass::Noun(NounForms { plural: "dagen".into(), diminutive: Some("dagje".into()) }), language)),
            ("werken".to_string(), Word::new("werken", "To work", "work", "Mijn vader gaat werken.", WordClass::Verb(VerbForms::new("werken","werk","werkt","werkt","werken","werkte","werkte","werkte","werkten","gewerkt")), language)),
            ("schoonmaken".to_string(), Word::new("schoonmaken", "To clean", "clean", "Ik moet mijn kamer schoonmaken.", WordClass::Verb(VerbForms::new("schoonmaken","maak schoon","maakt schoon","maakt schoon","maken schoon","maakte schoon","maakte schoon","maakte schoon","maakten schoon","schoongemaakt")), language)),
        ]))
    }

    pub fn inner(&self) -> &HashMap<String, Word> {&self.0}
}

pub struct CinderellaStory;
impl CinderellaStory {
    pub fn get(language: Language, level: Level, part: usize) -> Vec<Vec<Word>> {
        match language {
            Language::Dutch => {
                let vocab = Vocabulary::dutch();
                let paragraph = match (level, part) {
                    (Level::A1, 0) => "Er was eens een meisje. Haar naam was Cinderella. Zij-she- woonde met haar stiefmoeder en twee stiefzussen. Zij-they- waren niet lief voor haar. Cinderella moest elke dag werken en schoonmaken.",
                    _ => todo!()
                };

                paragraph.split('.').map(str::trim)
                    .filter(|s| !s.is_empty()).map(|s| s.split_whitespace()
                    .filter_map(|w| vocab.inner().get(&w.to_string().to_lowercase()).cloned())
                    .collect()).collect()
            }
            _ => todo!(),
        }


            
                    // (language, Level::A1, 1) => vec![

                // Er was eens een meisje. Haar naam was Cinderella.
                // Zij woonde met haar stiefmoeder en twee stiefzussen.
                // Zij waren niet lief voor haar.
                // Cinderella moest elke dag werken en schoonmaken.

                // Op een dag was er een groot bal in het paleis.
                // De prins wilde een vrouw vinden.
                // De stiefzussen gingen naar het bal, maar Cinderella mocht niet mee.

                // Ze was verdrietig.
                // Toen kwam er een fee.
                // De fee gaf haar een mooie jurk en glazen schoenen.
                // Maar ze moest voor middernacht terug zijn.

                // Op het bal danste de prins met Cinderella.
                // Ze waren blij.
                // Om middernacht rende ze weg en verloor haar schoen.

                // De prins vond de schoen.
                // Hij zocht het meisje.
                // De schoen paste alleen bij Cinderella.
                // Ze trouwden en leefden gelukkig.
                // // fill this vector with each word
                // Word::new(word, english_definition, english_translation, dutch_example, language)),
            // ]


    }
}


// // A2

// Cinderella was een vriendelijk meisje dat met haar stiefmoeder en twee stiefzussen woonde.
// Na de dood van haar vader moest zij al het werk in huis doen. Haar stiefmoeder behandelde haar slecht en haar zussen lachten haar uit.

// Op een dag kondigde de koning een groot bal aan. De prins wilde een vrouw kiezen. Alle jonge vrouwen uit het land werden uitgenodigd.

// De stiefzussen waren enthousiast en kochten nieuwe jurken. Cinderella wilde ook gaan, maar haar stiefmoeder verbood het. Toen zij alleen was, begon ze te huilen.

// Plotseling verscheen haar goede fee. Met magie veranderde zij een pompoen in een koets en gaf Cinderella een prachtige jurk. Ze waarschuwde haar: om middernacht zou de betovering verdwijnen.

// Op het bal werd de prins meteen verliefd. Ze dansten de hele avond samen. Maar toen de klok twaalf uur sloeg, rende Cinderella weg. Ze verloor één glazen schoen.

// De prins ging het hele land door om het meisje te vinden van wie de schoen was. Uiteindelijk paste de schoen alleen bij Cinderella. Ze trouwden en leefden gelukkig samen.


// // B1

// Cinderella verloor haar vader toen ze nog jong was. Haar stiefmoeder nam de leiding in huis over en behandelde haar als een dienares. Hoewel Cinderella hard werkte en vriendelijk bleef, werd ze dagelijks vernederd door haar stiefzussen.

// Toen de koning een koninklijk bal organiseerde zodat de prins een geschikte bruid kon vinden, zag Cinderella dit als een kans om even aan haar moeilijke leven te ontsnappen. Haar stiefmoeder verbood haar echter om mee te gaan en liet haar extra werk doen.

// Toen alles verloren leek, verscheen haar peetmoeder, een fee. Met behulp van magie veranderde ze eenvoudige voorwerpen in een prachtige koets, een schitterende jurk en glazen muiltjes. Toch stelde ze één belangrijke voorwaarde: voor middernacht moest Cinderella terugkeren.

// Op het bal trok ze onmiddellijk de aandacht van de prins. Ze voelden een bijzondere band en spraken urenlang met elkaar. Maar toen de klok twaalf uur sloeg, herinnerde Cinderella zich de waarschuwing en vluchtte haastig weg, waarbij ze één schoen verloor.

// De prins, vastbesloten haar terug te vinden, bezocht elk huis in het koninkrijk. Uiteindelijk paste het glazen muiltje perfect. Cinderella werd zijn vrouw en haar leven veranderde voorgoed.

// // B2

// Na de dood van haar vader werd Cinderella het slachtoffer van systematische vernedering door haar stiefmoeder en stiefzussen. Ze werd gereduceerd tot huishoudelijke arbeid en leefde in een toestand van sociale isolatie. Toch behield ze haar innerlijke waardigheid en vriendelijkheid.

// Het koninklijk bal symboliseerde voor haar niet alleen romantiek, maar ook sociale mobiliteit: een kans om haar lot te veranderen. Haar uitsluiting benadrukte de machtsverhoudingen binnen het gezin.

// De verschijning van de fee kan worden gezien als een moment van hoop en transformatie. De magie bood haar tijdelijk toegang tot een wereld waar ze anders nooit zou binnentreden. De beperking van middernacht onderstreepte echter de vergankelijkheid van deze kans.

// Tijdens het bal ontstond er een authentieke verbinding tussen haar en de prins. Zijn zoektocht na haar verdwijning toont vastberadenheid, maar ook het belang van symboliek: de glazen schoen fungeerde als bewijs van identiteit.

// Wanneer de schoen uiteindelijk past, wordt gerechtigheid hersteld. Het verhaal eindigt niet alleen met romantisch geluk, maar met de overwinning van karakter boven onderdrukking.

// // C1

// Het verhaal van Cinderella kan worden geïnterpreteerd als een narratief over onderdrukking, veerkracht en sociale transformatie. Na het overlijden van haar vader wordt zij onderworpen aan emotionele en fysieke uitbuiting binnen haar eigen huishouden. Haar identiteit wordt gemarginaliseerd, maar haar morele integriteit blijft intact.

// Het koninklijk bal functioneert als katalysator: een institutioneel georganiseerde gelegenheid waarin sociale hiërarchieën tijdelijk vloeibaar worden. De magische interventie van de fee representeert externe hulpbronnen die individuen in staat stellen structurele beperkingen te overstijgen.

// De beperking van middernacht introduceert existentiële spanning. De transformatie is fundamenteel tijdelijk, wat de fragiliteit van verworven kansen benadrukt.

// De glazen schoen, een ogenschijnlijk triviaal object, krijgt semiotische betekenis als instrument van herkenning en legitimatie. De prins’ zoektocht kan worden gelezen als een poging om authenticiteit te reconstrueren in een wereld van façade en uiterlijk vertoon.

// De uiteindelijke hereniging suggereert niet slechts romantische vervulling, maar symboliseert maatschappelijke erkenning en herstel van rechtvaardigheid.

// // C2

// Cinderella’s verhaal overstijgt het niveau van een eenvoudig sprookje en ontwikkelt zich tot een allegorie van sociale stratificatie, morele standvastigheid en existentiële hoop. Haar onderdrukking binnen het huishouden weerspiegelt machtsstructuren waarin afhankelijkheid wordt uitgebuit en identiteit systematisch wordt ontkend.

// De magische interventie kan worden opgevat als metafoor voor onverwachte contingentie: het moment waarop omstandigheden zich radicaal herschikken en nieuwe mogelijkheden ontstaan. Toch is deze transformatie inherent precair, begrensd door tijd en voorwaarde.

// Het bal vormt een theatrale ruimte waarin identiteit performatief wordt. Cinderella beweegt zich tussen twee werelden: de zichtbare wereld van pracht en de verborgen realiteit van vernedering. De glazen schoen fungeert als paradoxaal symbool: fragiel maar doorslaggevend, individueel maar universeel herkenbaar.

// De prins’ zoektocht vertegenwoordigt een epistemologische queeste: het verlangen om waarheid te onderscheiden van illusie. Wanneer de schoen past, wordt niet alleen een bruid gevonden, maar een onrecht hersteld.

// Zo blijft het sprookje resoneren als reflectie op menselijke waardigheid, rechtvaardigheid en de mogelijkheid tot transcendentie binnen beperkende structuren.