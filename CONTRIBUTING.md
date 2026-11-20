# Jak přidat obvod do knihovny

Děkujeme, že chcete knihovnu rozšířit. Přidat obvod můžete **formulářem na webu** (nejjednodušší) nebo **pull requestem** na GitHubu. Každý příspěvek před zveřejněním zkontroluje a vyzkouší správce knihovny.

## Co musí obvod splňovat

- **Funguje** – vyzkoušeli jste ho v LogicFlow, nekmitá a dělá, co popisuje.
- **Má popisky** – vstupy a výstupy jsou pojmenované (A, B, CLK, Y…), aby se v něm dalo vyznat.
- **Má název, popis a autora** – popis jednou až třemi větami řekne, co obvod dělá a jak ho ovládat.
- **Je vaše vlastní práce**, nebo máte souhlas autora.
- **Souhlasíte s licencí CC BY 4.0** – obvod smí kdokoli používat a upravovat, pokud uvede autora.

### Jméno autora

Zveřejní se jen jméno, které sami uvedete. **Žáci a studenti** prosím uvádějte jen křestní jméno, přezdívku nebo například „žák SŠSE Brno“ – ne celé jméno. Zveřejněný obvod na žádost autora odstraníme.

### Úlohy pro výuku

Řešení úloh je v knihovně veřejné. Pokud chcete úlohu zadat žákům, přidejte do kategorie `ulohy` jen zadání (`zadani.md`) a řešení si nechte.

## Formulář na webu

Na [www.logicflow-sim.cz](https://www.logicflow-sim.cz) na stránce **Knihovna obvodů** vyplňte formulář a přiložte soubor `.logicflow`. Formulář vyžaduje přihlášení účtem Google (školní stačí). Bez účtu Google pošlete obvod e-mailem uvedeným na webu. Správce pak obvod do knihovny přidá sám.

## Pull request

1. Vytvořte si fork repozitáře a v něm novou větev.
2. Založte složku `obvody/<kategorie>/<id>/`:
   - **kategorie**: `kombinacni`, `sekvencni`, `aplikace` nebo `ulohy`,
   - **id**: krátký název malými písmeny bez diakritiky, slova oddělená pomlčkou (např. `scitacka-4-bity`).
3. Uložte do ní obvod jako **`obvod.logicflow`** a doplňte do souboru (textový editor) část `info` hned za řádek `"app"`:

   ```json
   "info": {
     "title": "Sčítačka 4 bity",
     "authors": ["Jan"],
     "description": "Sečte dvě čtyřbitová čísla A a B, výsledek S a přenos CO.",
     "license": "CC BY 4.0",
     "created": "2026-11-20",
     "app_version": "0.3.1"
   },
   ```

   Od verze LogicFlow 0.4 se tyto údaje vyplňují přímo v programu (**Soubor → Vlastnosti obvodu**). Pozor: LogicFlow 0.3 část `info` při dalším uložení vynechá – doplňte ji až do hotového souboru.
4. Vedle obvodu vytvořte **`info.toml`**:

   ```toml
   kategorie = "kombinacni"
   obtiznost = 2            # 1 začátečník, 2 středně pokročilý, 3 pokročilý
   stitky = ["sčítačka", "funkční blok"]
   ```

5. Nepovinně přidejte **`zadani.md`** se zadáním úlohy.
6. Spusťte kontrolu a obnovte katalog (potřebujete [Rust](https://rustup.rs)):

   ```sh
   cargo run --manifest-path nastroje/kontrola/Cargo.toml -- katalog
   ```

   Pokud Rust nemáte, nevadí – kontrolu spustí GitHub a katalog doplní správce.
7. Otevřete pull request a vyplňte kontrolní seznam v šabloně.

## Pro správce

- Každý pull request schvaluje správce uvedený v `.github/CODEOWNERS` (zatím Stanislav Jan Fidrmuc).
- Příspěvek z formuláře: stáhnout soubor z Disku, doplnit `info` podle formuláře, vytvořit `info.toml`, spustit kontrolu s `katalog`, odeslat (vlastní commit nebo pull request).
- Před sloučením obvod otevřít v programu a vyzkoušet, zkontrolovat popis a vhodnost obsahu a u žáků i formu jména.
