use crate::stories::cinderella::CinderellaStory;
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum Language {
    English,
    Dutch,
}

pub enum Story {
    Cinderella,
}

impl Story {
    pub fn get(&self, language: Language, level: Level, part: usize) -> Vec<Vec<Word>> {
        match self {
            Story::Cinderella => CinderellaStory::get(language, level, part),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level { A1, A2, B1, B2, C1, C2 }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word {
    pub word: String, // dutch
    pub definition: String, // english definition
    pub translation: String, // english translation
    pub example: String, // dutch example
    pub class: WordClass,
    pub language: Language, // Language::Dutch
}

impl Word {
    pub fn new(word: &str, def: &str, eng: &str, ex: &str, class: WordClass, language: Language) -> Self {
        Word {
            word: word.to_string(),
            definition: def.to_string(),
            translation: eng.to_string(),
            example: ex.to_string(),
            class,
            language,
        }
    }

    pub fn forms(&self) -> Vec<(String, String)> {
        match &self.class {
            WordClass::Noun(forms) => forms.get(),
            WordClass::Verb(forms) => forms.get(),
            WordClass::Adjective(forms) => forms.get(),
            WordClass::Pronoun(forms) => forms.get(),
            WordClass::Numeral(forms) => forms.get(),
            _ => Vec::new()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordClass {
    Noun(NounForms),
    Verb(VerbForms),
    Adjective(AdjectiveForms),
    Adverb,
    Pronoun(PronounForms),
    Preposition,
    Conjunction,
    Interjection,
    Determiner,
    Article,
    Numeral(NumeralForms),
    Particle,
}

impl WordClass {
    pub fn get(&self) -> String {
        match self {
            WordClass::Noun(..) => "Noun",
            WordClass::Verb(..) => "Verb",
            WordClass::Adjective(..) => "Adjective",
            WordClass::Adverb => "Adverb",
            WordClass::Pronoun(..) => "Pronoun",
            WordClass::Preposition => "Preposition",
            WordClass::Conjunction => "Conjunction",
            WordClass::Interjection => "Interjection",
            WordClass::Determiner => "Determiner",
            WordClass::Article => "Article",
            WordClass::Numeral(..) => "Numeral",
            WordClass::Particle => "Particle"
        }.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NounForms {
    pub plural: String,
    pub diminutive: Option<String>, // Dutch-specific (e.g., dag → dagje)
}

impl NounForms {
    pub fn get(&self) -> Vec<(String, String)> {
        let mut forms = vec![("Plural".to_string(), self.plural.clone())];
        self.diminutive.clone().map(|d| forms.push(("Diminutive".to_string(), d.to_string())));
        forms
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VerbForms {
    pub infinitive: String,
    pub present_first_person_singular: String,
    pub present_second_person_singular: String,
    pub present_third_person_singular: String,
    pub present_plural: String,
    pub past_first_person_singular: String,
    pub past_second_person_singular: String,
    pub past_third_person_singular: String,
    pub past_plural: String,
    pub past_participle: String,
}

impl VerbForms {
    pub fn get(&self) -> Vec<(String, String)> {
        vec![
            ("Infinitive".to_string(), self.infinitive.clone()),
            ("Present First Person Singular".to_string(), self.present_first_person_singular.clone()),
            ("Present Second Person Singular".to_string(), self.present_second_person_singular.clone()),
            ("Present Third Person Singular".to_string(), self.present_third_person_singular.clone()),
            ("Present Plural".to_string(), self.present_plural.clone()),
            ("Past First Person Singular".to_string(), self.past_first_person_singular.clone()),
            ("Past Second Person Singular".to_string(), self.past_second_person_singular.clone()),
            ("Past Third Person Singular".to_string(), self.past_third_person_singular.clone()),
            ("Past Plural".to_string(), self.past_plural.clone()),
            ("Past Participle".to_string(), self.past_participle.clone()),
        ]
    }

    pub fn new(
        infinitive: &str,
        present_first_person_singular: &str,
        present_second_person_singular: &str,
        present_third_person_singular: &str,
        present_plural: &str,
        past_first_person_singular: &str,
        past_second_person_singular: &str,
        past_third_person_singular: &str,
        past_plural: &str,
        past_participle: &str,
    ) -> Self {
        Self {
            infinitive: infinitive.into(),
            present_first_person_singular: present_first_person_singular.into(),
            present_second_person_singular: present_second_person_singular.into(),
            present_third_person_singular: present_third_person_singular.into(),
            present_plural: present_plural.into(),
            past_first_person_singular: past_first_person_singular.into(),
            past_second_person_singular: past_second_person_singular.into(),
            past_third_person_singular: past_third_person_singular.into(),
            past_plural: past_plural.into(),
            past_participle: past_participle.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdjectiveForms {
    pub comparative: String,
    pub superlative: String,
}

impl AdjectiveForms {
    pub fn get(&self) -> Vec<(String, String)> {
        vec![
            ("Comparative".to_string(), self.comparative.to_string()),
            ("Superlative".to_string(), self.superlative.to_string()),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumeralForms {
    pub ordinal: String,
}

impl NumeralForms {
    pub fn get(&self) -> Vec<(String, String)> {
        vec![
            ("Ordinal".to_string(), self.ordinal.to_string()),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PronounForms {
    pub subject_form: Option<String>,
    pub object_form: Option<String>,
    pub possessive_form: Option<String>,
    pub reflexive_form: Option<String>,
}

impl PronounForms {
    pub fn get(&self) -> Vec<(String, String)> {
        let mut forms = Vec::new();
        if let Some(subject) = &self.subject_form { forms.push(("Subject Form".to_string(), subject.clone())); }
        if let Some(object) = &self.object_form { forms.push(("Object Form".to_string(), object.clone())); }
        if let Some(possessive) = &self.possessive_form { forms.push(("Possessive Form".to_string(), possessive.clone())); }
        if let Some(reflexive) = &self.reflexive_form { forms.push(("Reflexive Form".to_string(), reflexive.clone())); }
        forms
    }
}