//! Kontrola knihovny obvodů LogicFlow a sestavení katalogu.
//!
//! Použití (z kořene repozitáře):
//!   cargo run --manifest-path nastroje/kontrola/Cargo.toml            # kontrola
//!   cargo run --manifest-path nastroje/kontrola/Cargo.toml -- katalog # + zápis KATALOG.md
//!
//! Každý obvod je ve složce `obvody/<kategorie>/<id>/` a obsahuje
//! `obvod.logicflow` (s částí `info`: název, autoři, popis, licence)
//! a `info.toml` (kategorie, obtížnost, štítky).

use logicflow_core::{ProjectFile, Simulator};
use serde::Deserialize;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const LICENCE: &str = "CC BY 4.0";
const KATEGORIE: [(&str, &str); 4] = [
    ("kombinacni", "Kombinační obvody"),
    ("sekvencni", "Sekvenční obvody"),
    ("aplikace", "Aplikace"),
    ("ulohy", "Úlohy pro výuku"),
];
/// Nejvíc událostí simulace, než se obvod prohlásí za kmitající.
const LIMIT_UDALOSTI: usize = 200_000;

/// Část `info` v souboru `.logicflow` (od verze 0.4 ji upravuje i program).
#[derive(Deserialize, Default)]
struct Info {
    #[serde(default)]
    title: String,
    #[serde(default)]
    authors: Vec<String>,
    #[serde(default)]
    description: String,
    #[serde(default)]
    license: String,
}

#[derive(Deserialize)]
struct SouborInfo {
    #[serde(default)]
    info: Option<Info>,
}

/// `info.toml` vedle obvodu.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Knihovna {
    kategorie: String,
    obtiznost: u8,
    #[serde(default)]
    stitky: Vec<String>,
    #[serde(default)]
    zdroj: Option<String>,
}

struct Obvod {
    kategorie: String,
    id: String,
    info: Info,
    obtiznost: u8,
    stitky: Vec<String>,
    zadani: bool,
}

fn platne_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !id.starts_with('-')
        && !id.ends_with('-')
}

fn zkontroluj(slozka: &Path, kategorie: &str, chyby: &mut Vec<String>) -> Option<Obvod> {
    let id = slozka.file_name()?.to_string_lossy().into_owned();
    let jmeno = format!("{kategorie}/{id}");
    let mut chyba = |text: String| chyby.push(format!("{jmeno}: {text}"));

    if !platne_id(&id) {
        chyba(
            "název složky smí obsahovat jen malá písmena bez diakritiky, číslice a pomlčky".into(),
        );
    }

    // obvod.logicflow
    let cesta = slozka.join("obvod.logicflow");
    let Ok(text) = fs::read_to_string(&cesta) else {
        chyba("chybí soubor obvod.logicflow".into());
        return None;
    };
    let (obvod, podobvody) = match ProjectFile::from_json(&text) {
        Ok(v) => v,
        Err(e) => {
            chyba(format!("obvod.logicflow nejde načíst: {e}"));
            return None;
        }
    };
    let info = match serde_json::from_str::<SouborInfo>(&text) {
        Ok(s) => s.info.unwrap_or_default(),
        Err(e) => {
            chyba(format!("část info nejde přečíst: {e}"));
            Info::default()
        }
    };
    if info.title.trim().is_empty() {
        chyba("v obvod.logicflow chybí info.title (název obvodu)".into());
    }
    if info.authors.iter().all(|a| a.trim().is_empty()) {
        chyba("v obvod.logicflow chybí info.authors (jméno autora)".into());
    }
    if info.description.trim().is_empty() {
        chyba("v obvod.logicflow chybí info.description (popis)".into());
    }
    if info.license != LICENCE {
        chyba(format!(
            "info.license musí být \"{LICENCE}\", je \"{}\"",
            info.license
        ));
    }
    if obvod.components.is_empty() {
        chyba("obvod neobsahuje žádné součástky".into());
    }
    let mut sim = Simulator::with_subcircuits(&obvod, &podobvody, None);
    if !sim.run(LIMIT_UDALOSTI) {
        chyba("obvod kmitá – nemá ustálený stav".into());
    }

    // info.toml
    let knihovna = match fs::read_to_string(slozka.join("info.toml")) {
        Err(_) => {
            chyba("chybí soubor info.toml".into());
            None
        }
        Ok(t) => match toml::from_str::<Knihovna>(&t) {
            Ok(k) => Some(k),
            Err(e) => {
                chyba(format!("info.toml: {e}"));
                None
            }
        },
    };
    let knihovna = knihovna?;
    if knihovna.kategorie != kategorie {
        chyba(format!(
            "kategorie v info.toml ({}) neodpovídá složce ({kategorie})",
            knihovna.kategorie
        ));
    }
    if !(1..=3).contains(&knihovna.obtiznost) {
        chyba("obtiznost musí být 1, 2 nebo 3".into());
    }
    let _ = knihovna.zdroj;

    Some(Obvod {
        kategorie: kategorie.to_owned(),
        id,
        info,
        obtiznost: knihovna.obtiznost,
        stitky: knihovna.stitky,
        zadani: slozka.join("zadani.md").is_file(),
    })
}

fn obtiznost(n: u8) -> &'static str {
    match n {
        1 => "začátečník",
        2 => "středně pokročilý",
        _ => "pokročilý",
    }
}

fn bunka(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', " ")
}

fn katalog(obvody: &[Obvod]) -> String {
    let mut s = String::new();
    s.push_str("# Katalog obvodů\n\n");
    s.push_str(
        "<!-- Tento soubor vytváří nástroj nastroje/kontrola – neupravujte ho ručně. -->\n\n",
    );
    let _ = writeln!(
        s,
        "Počet obvodů v knihovně: {}. Všechny jsou pod licencí [CC BY 4.0](LICENSE): smíte je používat, upravovat a šířit, jen uveďte autora.\n",
        obvody.len()
    );
    for (kat, nazev) in KATEGORIE {
        let v: Vec<_> = obvody.iter().filter(|o| o.kategorie == kat).collect();
        let _ = writeln!(s, "## {nazev}\n");
        if v.is_empty() {
            s.push_str("Zatím žádné obvody – [přidejte první](CONTRIBUTING.md).\n\n");
            continue;
        }
        s.push_str(
            "| Obvod | Autor | Obtížnost | Popis | Štítky |\n| --- | --- | --- | --- | --- |\n",
        );
        for o in v {
            let zadani = if o.zadani {
                format!(" · [zadání](obvody/{}/{}/zadani.md)", o.kategorie, o.id)
            } else {
                String::new()
            };
            let _ = writeln!(
                s,
                "| [{}](obvody/{}/{}/obvod.logicflow){} | {} | {} | {} | {} |",
                bunka(&o.info.title),
                o.kategorie,
                o.id,
                zadani,
                bunka(&o.info.authors.join(", ")),
                obtiznost(o.obtiznost),
                bunka(&o.info.description),
                bunka(&o.stitky.join(", ")),
            );
        }
        s.push('\n');
    }
    s
}

fn main() -> ExitCode {
    let psat_katalog = std::env::args().any(|a| a == "katalog");
    let koren = PathBuf::from(".");
    let mut chyby = Vec::new();
    let mut obvody = Vec::new();

    let Ok(kategorie) = fs::read_dir(koren.join("obvody")) else {
        eprintln!("Spusťte z kořene repozitáře (chybí složka obvody).");
        return ExitCode::FAILURE;
    };
    let mut kategorie: Vec<_> = kategorie.flatten().map(|e| e.path()).collect();
    kategorie.sort();
    for kat in kategorie.iter().filter(|p| p.is_dir()) {
        let nazev = kat.file_name().unwrap().to_string_lossy().into_owned();
        if !KATEGORIE.iter().any(|(k, _)| *k == nazev) {
            chyby.push(format!(
                "obvody/{nazev}: neznámá kategorie (povolené: kombinacni, sekvencni, aplikace, ulohy)"
            ));
            continue;
        }
        let mut slozky: Vec<_> = fs::read_dir(kat)
            .unwrap()
            .flatten()
            .map(|e| e.path())
            .collect();
        slozky.sort();
        for slozka in slozky.iter().filter(|p| p.is_dir()) {
            if let Some(o) = zkontroluj(slozka, &nazev, &mut chyby) {
                obvody.push(o);
            }
        }
    }

    for o in &obvody {
        println!("ok  {}/{} – {}", o.kategorie, o.id, o.info.title);
    }
    if psat_katalog {
        if let Err(e) = fs::write(koren.join("KATALOG.md"), katalog(&obvody)) {
            eprintln!("KATALOG.md nejde zapsat: {e}");
            return ExitCode::FAILURE;
        }
        println!("Zapsán KATALOG.md, počet obvodů: {}.", obvody.len());
    }
    if chyby.is_empty() {
        println!("Kontrola prošla, počet obvodů: {}.", obvody.len());
        ExitCode::SUCCESS
    } else {
        for c in &chyby {
            eprintln!("CHYBA {c}");
        }
        eprintln!("Kontrola neprošla, počet chyb: {}.", chyby.len());
        ExitCode::FAILURE
    }
}
