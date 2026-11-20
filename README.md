# Knihovna obvodů LogicFlow

Sdílená knihovna hotových obvodů pro simulátor logických obvodů [LogicFlow](https://github.com/SJFidrmuc/LogicFlow). Obvody můžete stáhnout, otevřít v programu, upravit a použít ve výuce – a přidat do knihovny i své vlastní.

**→ [Katalog všech obvodů](KATALOG.md)**

Web projektu: [www.logicflow-sim.cz](https://www.logicflow-sim.cz)

## Jak obvod použít

1. V [katalogu](KATALOG.md) klikněte na název obvodu.
2. Na stránce souboru stáhněte soubor tlačítkem *Download raw file* (ikona šipky vpravo nahoře).
3. Otevřete ho v LogicFlow: **Soubor → Otevřít** (Ctrl+O).

Ke stažení celé knihovny najednou: zelené tlačítko *Code → Download ZIP*.

## Licence

Obvody jsou pod licencí **[CC BY 4.0](LICENSE)**: smíte je používat, upravovat a šířit, i ve škole, jen **uveďte autora**. Jméno autora je u každého obvodu v katalogu a přímo v souboru obvodu (část `info`). Od verze LogicFlow 0.4 ho uvidíte i v programu v nabídce **Soubor → Vlastnosti obvodu**.

## Jak přispět

Přidat obvod můžete dvěma způsoby:

- **formulářem na webu** [www.logicflow-sim.cz](https://www.logicflow-sim.cz) (stránka Knihovna obvodů) – nejjednodušší, stačí účet Google,
- **pull requestem** zde na GitHubu.

Každý příspěvek před zveřejněním zkontroluje správce knihovny. Podrobný postup a pravidla jsou v [CONTRIBUTING.md](CONTRIBUTING.md).

## Kategorie

| Složka | Kategorie | Co obsahuje |
| --- | --- | --- |
| [`obvody/kombinacni`](obvody/kombinacni) | Kombinační obvody | hradla, převodníky kódů, sčítačky, komparátory, multiplexory |
| [`obvody/sekvencni`](obvody/sekvencni) | Sekvenční obvody | klopné obvody, čítače, registry |
| [`obvody/aplikace`](obvody/aplikace) | Aplikace | řízení výtahu, alarm, semafor, hrací kostka |
| [`obvody/ulohy`](obvody/ulohy) | Úlohy pro výuku | zadání úloh (`zadani.md`), případně s řešením |

## Struktura obvodu

```
obvody/<kategorie>/<id>/
├── obvod.logicflow   obvod; část "info" obsahuje název, autory, popis a licenci
├── info.toml         kategorie, obtížnost (1–3), štítky
└── zadani.md         (nepovinné) zadání úlohy
```

Automatická kontrola (GitHub Actions) u každé změny ověří, že se obvod v LogicFlow načte, nekmitá, má vyplněného autora, popis a licenci CC BY 4.0 a že katalog odpovídá obsahu knihovny.

## Plán

- **Teď:** knihovna v tomto repozitáři, katalog, formulář na webu.
- **Později:** web knihovny s náhledy obvodů a filtrem (obvody.logicflow-sim.cz) a online knihovna přímo v panelu Knihovna součástek v programu.

---

## In English

A shared library of ready-made circuits for the [LogicFlow](https://github.com/SJFidrmuc/LogicFlow) logic simulator. Browse the [catalogue](KATALOG.md), download a `.logicflow` file and open it with **File → Open**. All circuits are licensed under **CC BY 4.0** – use and adapt them freely, just credit the author named in the file and in the catalogue. Contributions are welcome via pull request, see [CONTRIBUTING.md](CONTRIBUTING.md) (in Czech).
