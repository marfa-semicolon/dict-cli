use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DictEntry {
    headword: Headword,
    /// part of speech
    pos: String,
    // senses: Vec<String>,
    groups: Vec<Group>,
    idioms: Vec<Idiom>,
    phrasals: Vec<Phrasal>,
    matches: Vec<Match>,
}
#[derive(Debug, Serialize)]
pub struct Group {
    /// name of the category
    category: Option<String>,
    senses: Vec<Sense>,
}

impl Group {
    pub fn new(category: Option<String>, senses: Vec<Sense>) -> Self {
        Self { category: category, senses }
    }
}

#[derive(Debug, Serialize)]
pub struct Sense {
    /// definition
    def: String,
    num: u8,
    examples: Vec<String>,
}
impl Sense {
    pub fn new(def: String, num: u8, examples: Vec<String>) -> Self {
        Self { def, num, examples }
    }
}
#[derive(Debug, Serialize)]
pub struct Match {
    headword: String,
    url: String,
    entry: u8,
}

impl Match {
    pub fn new(headword: String, url: String, entry: u8) -> Self {
        Self {
            headword,
            url,
            entry,
        }
    }

    pub fn headword(&self) -> &str {
        &self.headword
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn entry(&self) -> u8 {
        self.entry
    }
}

#[derive(Debug, Serialize)]
pub struct Phrasal {
    phrasal: String,
    url: String,
}

impl Phrasal {
    pub fn new(phrasal: String, url: String) -> Self {
        Self { phrasal, url }
    }
}

#[derive(Debug, Serialize)]
pub struct Idiom {
    idm: String,
    def: String,
}

impl Idiom {
    pub fn new(idm: String, def: String) -> Self {
        Self { idm, def }
    }
}

#[derive(Debug, Serialize)]
pub enum Headword {
    Verb {
        base_form: VForm,
        past_simple: VForm,
        past_participle: VForm,
        present_participle: VForm,
        // pronuns: VerbPronun,
    },
    Other {
        text: String,
        pronun: Pronun,
    },
}

#[derive(Debug, Serialize)]
pub enum VForm {
    Base { text: String, pronun: Pronun },
    PastSimple { text: String, pronun: Pronun },
    PastPart { text: String, pronun: Pronun },
    PresPart { text: String, pronun: Pronun },
}
// #[derive(Debug)]
// struct VerbPronun {
//     base_form: Pronun,
//     past_simple: Pronun,
//     past_participle: Pronun,
// }

#[derive(Debug, Serialize)]
pub struct Pronun {
    br: Vec<String>,
    n_am: Vec<String>,
}

impl Pronun {
    pub fn new(br: Vec<String>, n_am: Vec<String>) -> Self {
        Self { br, n_am }
    }
}

impl DictEntry {
    pub fn new(
        headword: Headword,
        groups: Vec<Group>,
        pos: String,
        idioms: Vec<Idiom>,
        phrasals: Vec<Phrasal>,
        matches: Vec<Match>,
    ) -> Self {
        Self {
            headword,
            groups,
            pos,
            idioms,
            phrasals,
            matches,
        }
    }
}
