use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};

use crate::dict::{DictEntry, Group, Idiom, Match, Phrasal, Pronun, Sense, VForm};

const BASE_URL: &'static str = "https://www.oxfordlearnersdictionaries.com/search/english/?q=";

enum PronunType {
    /// British
    Br,
    /// North american
    NAm,
}

enum VFormType {
    Base,
    PastSimple,
    PastPart,
    PresPart,
}

fn pronun_sel(pron: PronunType, form: &str) -> Selector {
    let pron = match pron {
        PronunType::Br => "br",
        PronunType::NAm => "n_am",
    };

    Selector::parse(format!("tr.verb_form[form=\"{form}\"] div.phons_{pron} div.sound").as_str())
        .unwrap()
}

fn get_phrasals(parent: &ElementRef) -> Vec<Phrasal> {
    let phrases_sel = Selector::parse("aside.phrasal_verb_links > ul.pvrefs > li > a").unwrap();
    let phrases = parent.select(&phrases_sel);

    phrases
        .map(|i| {
            let link = i.attr("href").unwrap();
            println!("phras_link: {link}");

            let phras_sel = Selector::parse("span > span.xh").unwrap();

            let phras = i.select(&phras_sel).next().unwrap().text().next().unwrap();

            Phrasal::new(phras.to_string(), link.to_string())
        })
        .collect()
}

fn get_vform(parent: &ElementRef, form: VFormType) -> VForm {
    let v_forms_sel = Selector::parse(".verb_forms_table").unwrap();
    let v_forms = parent.select(&v_forms_sel).next().unwrap();

    let form_str = match form {
        VFormType::Base => "root",
        VFormType::PastSimple => "past",
        VFormType::PastPart => "pastpart",
        VFormType::PresPart => "prespart",
    };

    let text_sel =
        Selector::parse(format!("tr.verb_form[form=\"{form_str}\"] > td.verb_form").as_str())
            .unwrap();

    let text = parent
        .select(&text_sel)
        .map(|i| {
            println!("{i:?}");
            i
        })
        .next()
        .unwrap().text().nth(2).unwrap().to_string()
        // .to_string();
        ;

    let br_sel = pronun_sel(PronunType::Br, form_str);
    let n_am_sel = pronun_sel(PronunType::NAm, form_str);
    let pronun = Pronun::new(
        v_forms
            .select(&br_sel)
            .map(|i| i.attr("data-src-mp3").unwrap().to_string())
            .collect(),
        v_forms
            .select(&n_am_sel)
            .map(|i| i.attr("data-src-mp3").unwrap().to_string())
            .collect(),
    );
    return match form {
        VFormType::Base => VForm::Base { text, pronun },
        VFormType::PastSimple => VForm::PastSimple { text, pronun },
        VFormType::PastPart => VForm::PastPart { text, pronun },
        VFormType::PresPart => VForm::PresPart { text, pronun },
    };
    // VForm::Base {
    //     text: text,
    //     pronun: pronun,
    // }
}

pub async fn fetch(query: String, entry: u8) -> DictEntry {
    let url = if entry != 1 {
        format!("https://www.oxfordlearnersdictionaries.com/definition/english/{query}_{entry}")
    } else {
        format!("{}{}", BASE_URL, query)
    };

    println!("base: {url}");

    let res = reqwest::get(url).await.unwrap();
    // let doc = include_str!("../cast.html").to_string();
    let doc = res.text().await.unwrap();

    let html = Html::parse_document(doc.as_str());

    std::fs::write("./tmp/page.html", doc).unwrap();

    let entry_sel = Selector::parse(".entry").unwrap();
    let entry = html.select(&entry_sel).next().unwrap();

    // let headword_sel = Selector::parse(".headword").unwrap();
    // let headword = entry
    //     .select(&headword_sel)
    //     .next()
    //     .unwrap()
    //     .text()
    //     .next()
    //     .unwrap();

    // every senses except idioms
    // let senses_sel = Selector::parse("ol.senses_multiple li.sense span.def").unwrap();

    let senses_sel = Selector::parse("ol.senses_multiple li.sense").unwrap();
    let senses: Vec<Sense> = entry
        .select(&senses_sel)
        .enumerate()
        .map(|(idx, i)| {
            let group_name_sel = Selector::parse("h2.shcut:has(+ li.sense)").unwrap();
            let group_name = i
                .select(&group_name_sel)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .to_string();

            let sense_sel = Selector::parse("span.def").unwrap();
            // let sense = i
            //     .select(&senses_sel)
            //     .enumerate()
            //     .map(|(idx, i)| {
            //         // let def_sel = Selector::parse("span.def").unwrap();

            //         // let def = i.select(&def_sel).next().unwrap().text().next().unwrap();

            //         println!("examples: {}", exmpls.len());

            //         Sense::new(def.to_string(), (idx + 1) as u8, exmpls)
            //     })
            // .collect();
            let sense = i.select(&sense_sel).next().unwrap();
            let def = sense.text().next().unwrap().to_string();
            let num = idx + 1;
            let exmpls_sel = Selector::parse("ul.examples li").unwrap();
            let exmpls: Vec<String> = i.select(&exmpls_sel).map(|i| i.text().join(" ")).collect();

            Sense::new(def, num as u8, exmpls)
            // todo!()

            // itertools::join(i.text(), " ")
        })
        .collect();
    // todo!();
    Group::new(group_name.into(), senses);

    // let groups_sel = Selector::parse("ol.senses_multiple span.shcut-g").unwrap();
    // let groups: Vec<Group> = entry
    //     .select(&groups_sel)
    //     .map(|i| {
    //         let group_name_sel = Selector::parse("h2.shcut").unwrap();
    //         let group_name = i
    //             .select(&group_name_sel)
    //             .next()
    //             .unwrap()
    //             .text()
    //             .next()
    //             .unwrap()
    //             .to_string();

    //         let senses_sel = Selector::parse("li.sense").unwrap();
    //         let senses = i
    //             .select(&senses_sel)
    //             .enumerate()
    //             .map(|(idx, i)| {
    //                 let def_sel = Selector::parse("span.def").unwrap();

    //                 let def = i.select(&def_sel).next().unwrap().text().next().unwrap();

    //                 let exmpls_sel = Selector::parse("ul.examples li").unwrap();
    //                 let exmpls: Vec<String> =
    //                     i.select(&exmpls_sel).map(|i| i.text().join(" ")).collect();

    //                 println!("examples: {}", exmpls.len());

    //                 Sense::new(def.to_string(), (idx + 1) as u8, exmpls)
    //             })
    //             .collect();

    //         todo!();
    //         Group::new(group_name.into(), senses)
    //         // todo!()

    //         // itertools::join(i.text(), " ")
    //     })
    //     .collect();

    // let senses: Vec<String> = entry
    //     .select(&senses_sel)
    //     .map(|i| itertools::join(i.text(), " "))
    //     .collect();

    let pos_sel = Selector::parse("span.pos").unwrap();
    let pos = entry
        .select(&pos_sel)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap();

    let idioms_sel = Selector::parse("div.idioms .idm-g").unwrap();
    let idioms = entry
        .select(&idioms_sel)
        .map(|i| {
            // let idm_sel = Selector::parse("");
            let idm_sel = Selector::parse("span.idm").unwrap();
            let idm = i.select(&idm_sel).next().unwrap().text().next().unwrap();
            let idm_def_sel = Selector::parse("span.def").unwrap();
            let idm_def = i
                .select(&idm_def_sel)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap();
            // itertools::join(i.text(), " ")

            Idiom::new(idm.to_string(), idm_def.to_string())
        })
        .collect::<Vec<Idiom>>();

    // let mut past_simple = None;
    // let past_participle = None;
    // let present_participle = None;
    // let past_simple_pronun = None;
    // let past_participle_pronun = None;
    // let present_participle_pronun = None;

    // if pos.to_lowercase() == "verb" {
    //     let v_forms_sel = Selector::parse(".verb_forms_table").unwrap();
    //     let v_forms = entry.select(&v_forms_sel).next().unwrap();

    // past_simple = Some(Pronun::new(
    //     v_forms
    //         .select(&Selector::parse("[].sound.pron-uk").unwrap())
    //         .map(|i| i.attr("verb_forms_table").unwrap().to_string())
    //         .collect(),
    //     v_forms
    //         .select(&Selector::parse(".sound.pron-us").unwrap())
    //         .map(|i| i.attr("verb_forms_table").unwrap().to_string())
    //         .collect(),
    // ));
    // past_simple = Some(Pronun::new(
    //     v_forms
    //         .select(
    //             &Selector::parse("tr.verb_form[form=\"past\"] div.phons_br div.sound").unwrap(),
    //         )
    //         .map(|i| i.attr("data-src-mp3").unwrap().to_string())
    //         .collect(),
    //     v_forms
    //         .select(&Selector::parse("tr.verb_form[form=\"past\"] div.phons_n_am").unwrap())
    //         .next()
    //         .unwrap()
    //         .text()
    //         .map(|i| i.to_string())
    //         .collect(),
    // ));
    // println!("pastsimp: {:#?}", past_simple);
    // }

    // println!("sense 1: {}", senses[0]);
    // std::fs::write("res", doc).unwrap();
    // println!("{:#?}", doc);

    let term_width = terminal_size::terminal_size().unwrap().0.0 as usize;
    let mut phrasals = Vec::new();
    let matches_sel = Selector::parse("#relatedentries > dl > dt:first-of-type + dd ul a").unwrap();
    // let matches_sel = Selector::parse("#relatedentries > dl > dd:nth-child(2) > ul > li:nth-child(20) a").unwrap();

    let matches: Vec<_> = html
        .select(&matches_sel)
        .map(|i| {
            let headword = i.text().next().unwrap().to_string();
            println!("i: {:?}", i);
            let url = i.attr("href").unwrap().to_string();
            // https://www.oxfordlearnersdictionaries.com/definition/english/cast_2
            let entry = {
                println!("url: {url}");
                let first = url
                    .split_once(
                        format!("https://www.oxfordlearnersdictionaries.com/definition/english/",)
                            .as_str(),
                    )
                    .unwrap()
                    .1;
                println!("first: {first}");
                let second = first.split_once("#").unwrap_or((first, "")).0;
                let third = (second.split_once("_").unwrap_or((second, "")).1)
                    .parse::<u8>()
                    .unwrap_or(1);
                third
            };

            Match::new(headword, url, entry)
        })
        .collect();
    println!("len: {:?}", matches.len());
    println!("mathes: {:#?}", matches);

    let longest_match_headword = matches.iter().fold(0, |acc: usize, i| {
        if i.headword().len() > acc {
            i.headword().len()
        } else {
            acc
        }
    });

    matches.iter().for_each(|i| {
        //         let word = i.headword().trim_end();
        // let entry = i.entry();

        // let left_padding =
        //     term_width.saturating_sub(longest_match_headword);

        // let word_padding =
        //     longest_match_headword.saturating_sub(word.chars().count());

        // println!(
        //     "{}\x1b]8;;{}\x1b\\{}{}\x1b]8;;\x1b\\ - {}",
        //     " ".repeat(left_padding),
        //     i.url(),
        //     word,
        //     " ".repeat(word_padding),
        //     entry,
        // );
        // =========================
        let word = i.headword().trim_end();
        let entry = i.entry();

        let word_padding = longest_match_headword.saturating_sub(word.chars().count());

        let content_width = longest_match_headword + 3 + format!("{} ", entry).chars().count(); // " - "

        let left_padding = term_width.saturating_sub(content_width);

        println!(
            "{}\x1b]8;;{}\x1b\\{}{}\x1b]8;;\x1b\\ - {}",
            " ".repeat(left_padding),
            i.url(),
            word,
            " ".repeat(word_padding),
            entry,
        );
        //================
        // let word = i.headword().trim_end();
        // let entry = i.entry();

        // let text = format!("{} - {}", word, entry);
        // let padding = term_width.saturating_sub(text.chars().count());

        // println!(
        //     "{}\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\ - {}",
        //     " ".repeat(padding),
        //     i.url(),
        //     word,
        //     entry,
        // );

        // let headword = format!("{} - {}",i.headword().trim_end(), i.entry());
        // let padding = longest_match_headword.saturating_sub(headword.chars().count());
        // let left_padding = term_width.saturating_sub(longest_match_headword);

        // println!(
        //     "{}\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\{}",
        //     " ".repeat(left_padding),
        //     i.url(),
        //     headword,
        //     // format!(" - {}{}", headword, i.entry()),
        //     " ".repeat(padding),

        // );
        // let field = format!("{:<longest_match_headword$}", i.headword());

        // let left_padding = term_width.saturating_sub(longest_match_headword);

        // println!(
        //     "{}\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
        //     " ".repeat(left_padding),
        //     i.url(),
        //     field,
        // );
        // println!("{:>term_width$}", format!("\x1b]8;;{}\x1b\\{:<longest_match_headword$}\x1b]8;;\x1b\\", i.url(), i.headword()));
    });

    println!("matches: {}", matches.len());

    return DictEntry::new(
        if pos.to_lowercase() == "verb" {
            phrasals = get_phrasals(&entry);
            crate::dict::Headword::Verb {
                base_form: get_vform(&entry, VFormType::Base),
                past_simple: get_vform(&entry, VFormType::PastSimple),
                past_participle: get_vform(&entry, VFormType::PastPart),
                present_participle: get_vform(&entry, VFormType::PresPart),
            }

            // crate::dict::Headword::Verb {
            //     base_form: headword.to_string(),
            //     past_simple: entry
            //         .select(&Selector::parse("").unwrap())
            //         .next()
            //         .unwrap()
            //         .text()
            //         .next()
            //         .unwrap()
            //         .to_string(),
            //     past_participle: (),
            //     present_participle: (),
            //     pronuns: (),
            // }
        } else {
            crate::dict::Headword::Other {
                text: String::new(),
                pronun: Pronun::new(["".to_string()].into(), ["".to_string()].into()),
            }
        },
        senses,
        pos.to_string(),
        idioms,
        phrasals,
        // todo!()
        matches,
    );
}
