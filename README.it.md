# Craft Anything

[English](README.md) · [日本語](README.ja.md) · [简体中文](README.zh.md) · [繁體中文](README.zh-hant.md) · [한국어](README.ko.md) · [Español](README.es.md) · [Français](README.fr.md) · **Italiano** · [Tiếng Việt](README.vi.md) · [Português](README.pt.md) · [ไทย](README.th.md) · [Türkçe](README.tr.md)

Scopri come funziona qualsiasi sito web, app o gioco e lascia che l'IA faccia la build della tua versione; un engine open-source *clean-room* scritto interamente in Rust.

Puntalo verso un target, digli cosa vuoi ottenere e lui studierà l'originale con veri tool di ispezione, farà creare un nuovo progetto a un coding agent, confronterà il risultato con l'originale e lo rifinirà.

[Scarica Craft Anything](https://github.com/storytold/craft-anything/releases/latest) · [ArtCraft](https://artcraft.org) · [Tutte le Crafting Apps](https://artcraft.org/apps)

> [!NOTE]
> ArtCraft è una community di artisti provenienti da ogni ambito: pittori, fotografi, registi, illustratori, designer, animatori, hobbisti e persone che hanno preso in mano una matita la settimana scorsa. Se crei qualcosa, sei uno di noi. Passa a salutarci su Discord.

[Come funziona](#come-funziona) · [Due modi per usarlo](#due-modi-per-usarlo) · [IA](#ia) · [Agent](#fatto-per-gli-agenti) · [Sotto il cofano](#sotto-il-cofano) · [Per iniziare](#per-iniziare) · [Status](#status) · [Download](#download) · [Discord](https://discord.gg/artcraft)

### 🔍 Studia qualsiasi cosa
Siti web, desktop app, app mobile, giochi e file: veri tool di ispezione scoprono come sono costruiti e cosa fanno, fornendo l'*evidence* dietro ogni risposta.

### 🛠️ Crea la tua versione
Di' cosa vuoi (*"Ricrea questo sito web"*, *"Ricrea il gameplay di questo gioco in Rust"*) e un coding agent farà la build di un nuovo progetto nello stack che preferisci.

### ✅ Verificato, non tirato a indovinare
Ogni build viene confrontata con l'originale (pagine su tre larghezze di schermo, il self-test interno di un gioco, i run di un'app) e rifinita finché non risulta fedele.

### 🤖 Pronto per gli agent
320 tool, ognuno disponibile sia come comando CLI che come tool MCP, così qualsiasi agent può pilotare lo stesso engine usato dall'app. Per ispezionare non serve l'IA.

## Come funziona

Punta Craft Anything su un target, digli cosa vuoi e premi **Start**. Lavora in quattro step, chiedendoti in parole semplici quando spetta a te decidere:

1. **Study:** ispeziona l'originale — le sue pagine a ogni larghezza, il codice, i file, il comportamento a runtime.
2. **Build:** un coding agent crea un nuovo progetto nel linguaggio, framework o engine indicato nella tua richiesta (o in un default sensato).
3. **Check:** confronta la build con l'originale ed elenca ciò che non fa ancora match.
4. **Ready:** apri, gioca o visualizza la preview del risultato, oppure chiedi modifiche per rifinirlo.

## Due modi per usarlo

* **Simple (default):** *"What do you want to make?"* Scegli un sito web, un'app o un gioco, descrivilo e premi **Start**. La pagina del progetto mostra i quattro step (`Study → Build → Check → Ready`), cosa sta facendo l'agent in parole semplici, schede per domande e approvazioni, quanto il risultato fa match con l'originale e i pulsanti per aprire il risultato o la sua cartella. Chiedi modifiche nel riquadro in basso. **"Just look inside"** apre un target per fargli domande senza fare alcuna build (in modalità Deep).
* **Advanced:** il workbench completo — ogni tool (`Ctrl/Cmd+K`), funzioni, stringhe, pseudo-C, disassemblato, esadecimale, call graph, evidence, unknowns, job, log e la console dell'agent.

Entrambe le modalità pilotano lo stesso engine.

## Clean room o Deep

* **Clean room (default):** non viene letto nulla all'interno dell'originale — né il codice, né i file, né il page source. Craft Anything lavora solo su ciò che si può vedere: immagini che aggiungi tu, il sito web così come lo vede un visitatore (screenshot, testo visibile, layout misurato, colori, font, spaziature, menu aperti) e, quando lo consenti, un'immagine della finestra dell'app. I tool che guarderebbero all'interno vengono bloccati e spiegano come attivare la modalità Deep.
* **Deep:** vengono usati tutti i tool, per il tuo uso privato. Il codice e i file di un programma vengono letti con gli engine integrati (e con Ghidra, se installato), il JavaScript di un sito web viene letto e la sua pagina iniziale osservata, e il builder può usare tutte queste informazioni.

> *Clean room* è una rule su ciò che Craft Anything offre o rifiuta. Non è una sandbox a livello di OS: non isola il tuo computer. Un coding agent che esegui agisce comunque con i tuoi permessi, e ciò che può leggere, scrivere o eseguire dipende dalle sue autorizzazioni. Quali tool di un agent Craft Anything decida di bloccare varia da agent ad agent; per i tool di Claude Code nelle chat e nelle domande, consulta `docs/claude-code.md`.

Puoi switchare modalità dalla barra del titolo o in **Settings**, con `settings_set deep_mode true`, oppure per una singola esecuzione da CLI con `--deep` (o `--clean-room`).

## IA

Per le build serve un coding agent installato sul tuo computer. **Claude Code** è supportato per primo: Craft Anything esegue il tuo programma `claude` originale e non modificato, utilizzando il tuo login personale; Craft Anything non vede né gestisce mai il tuo account. Il tuo utilizzo rientra nel tuo contratto con Anthropic. Consulta `docs/claude-code.md` per scoprire come e perché. Esiste un adapter per **Codex**, ma non è ancora stato verificato su un'installazione reale. **GitHub Copilot** può usare la tua Copilot CLI installata tramite ACP: effettua il login separatamente con `copilot login`. **Grok Build** può usare il tuo tool `grok` installato tramite ACP (`grok agent stdio`): autenticati separatamente con `grok login`. Consulta `docs/grok.md`. Altri agent che parlano ACP (**Gemini CLI**, **Qwen Code**, **opencode**, **goose**, **Hermes**, **Maki**, **Devin**) possono essere switchati per singolo backend; tra questi, Maki e Devin sono stati eseguiti live (Devin si autentica con la chiave salvata nel backend — il suo login CLI autonomo non vale su ACP). **Anthropic**, **OpenAI**, **OpenRouter**, **xAI** (Grok, quando è impostata `XAI_API_KEY`) e i server locali compatibili con OpenAI sono disponibili per domande e analisi (**Settings → AI**).

## Fatto per gli agenti

Ogni tool è anche un sub-command CLI e un tool MCP, così un agent può studiare un target o lanciare una rebuild completa senza passare dall'app:

```bash
craft-anything-cli commands # every tool, by toolset
craft-anything-cli mcp # serve them to any MCP client
craft-anything-cli rebuild [https://example.com](https://example.com) "Recreate this website"
```

I risultati sono record di evidence completi di confidence, coverage e limitazioni; quelli più voluminosi tornano indietro prima come summary e possono essere paginati con `core_evidence_browse`. `craft-anything-cli setup` connette Craft Anything alle app dei coding agent.

## Sotto il cofano

Un engine di reverse-engineering scritto interamente in Rust con 320 tool (300 nell'engine principale, i restanti nei layer di AI e setup) per programmi nativi, .NET, Java, Android, Apple, Electron/JavaScript, siti web, firmware, packet capture, database e file di giochi. I suoi decoder, il disassembler e il decompiler integrati funzionano senza tool di terze parti; la modalità **Deep** li utilizza tutti, insieme a Ghidra quando installato. Alimenta la desktop app (`egui`), la CLI e il server MCP, e alcune sue parti girano nel browser come WebAssembly. L'elenco completo è disponibile in `docs/tools.md`.

## Per iniziare

Richiede **Rust 1.97** o versione successiva (necessario per una dipendenza; esegui `rustup update`). Su Debian o Ubuntu, l'app desktop richiede inoltre:

```bash
sudo apt-get install build-essential libxkbcommon-dev libxkbcommon-x11-0 libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev libgtk-3-dev
```

Successivamente:

```bash
git clone [https://github.com/storytold/craft-anything](https://github.com/storytold/craft-anything) && cd craft-anything
cargo run --release -p craft-anything # the desktop app
cargo install --path apps/craft-anything-cli # the CLI, as `craft-anything-cli`
craft-anything-cli doctor # checks the install, coding agents and engines
```

Per le build serve un coding agent loggato sul computer (vedi [IA](#ia)); `doctor` indica cosa manca. Puoi avviare una rebuild da CLI (le domande vengono poste nel terminale):

```bash
craft-anything-cli rebuild [https://example.com](https://example.com) "Recreate this website"
craft-anything-cli rebuild - "A snake game in Rust" --rounds 1
craft-anything-cli rebuild [https://example.com](https://example.com) "Make the header blue" --continue-in "<project folder>"
craft-anything-cli rebuild - "Add a contact form" --project-folder "<your folder>" --existing edit
```

I nuovi progetti vengono salvati in `Documents/Craft Anything` (modificabile in **Settings → General**). Ogni cartella di progetto contiene `craft.json` (la request, il target, lo status e la history), `capture/` (ciò che è stato registrato dal target), `app/` (il progetto generato), `checks/` (i check contro l'originale), `investigation.cany/` (l'evidence) e `inbox/` (le domande aperte).

Un progetto può anche partire da una cartella a tua scelta (**Choose a folder…** sotto **Start**, oppure `--project-folder`). Se contiene già dei file, puoi specificare cosa sono: l'app stessa, che il builder modificherà in place, oppure materiale di partenza, con l'app generata in `app/` accanto ad essi (**Settings → General** può ricordare la tua scelta). **Settings → General** elenca inoltre i progetti provenienti da altre cartelle; rimuoverne uno lo toglie soltanto dall'elenco.

## Status

> [!IMPORTANT]
> **Status:** Craft Anything è in versione *early alpha*. Cosa funziona oggi, cosa è parziale e la roadmap:
> 
> * **Funziona oggi:** l'app desktop (Simple e Advanced), la CLI e il server MCP; il workflow `Study → Build → Check → Ready` per siti web, app e giochi con Claude Code come builder; check di siti web su tre larghezze di schermo; rebuild di giochi in Unity, Unreal o Godot; modalità Deep con Ghidra; immagini e modelli 3D generati (opt-in, richiede la tua chiave fal.ai); plugin (server MCP e componenti WASM nelle build tramite l'opzione `wasm-plugins`).
> * **Parziale:** build con chiavi API e altri coding agent (implementato, ma **non ancora testato in produzione**), check del comportamento per app e giochi, alcuni tool dell'engine testati solo contro stand-in.
> * **Rilasciati:** nella pagina delle Releases sono pubblicate le release con gli installer per Windows, macOS, Linux e FreeBSD; sono ancora in fase alpha.
> * **In arrivo:** il managed tier.

`docs/STATUS.md` contiene i dettagli e la spiegazione di come ogni parte è stata testata, `docs/tool-status.md` la checklist per ogni tool, e `docs/ROADMAP.md` il piano step by step. Apri una issue o segnalaci cosa non va su Discord.

**Uso responsabile.** Usa Craft Anything su software di tua proprietà o che sei autorizzato a esaminare. Non bypassa mai DRM, login o anti-cheat. Le rebuild ricreano layout, stile e comportamento basandosi sull'evidence; l'agent è istruito a non copiare loghi, foto o testi, a meno che tu non confermi di esserne il proprietario o di averne i permessi.

**L'installazione delle dipendenze esegue del codice.** Quando un builder installa le dipendenze di un progetto, i comandi runnano sul tuo computer e i package possono eseguire del codice durante l'installazione. Di default, `npm` esegue gli script di installazione di ogni package, e anche i build script di Rust e i package sorgente Python eseguono il proprio setup code. Craft Anything chiede conferma prima di lanciare un tipo di comando che non ha ancora visto in questo progetto (`npm`, `cargo`…). Selezionare "always allow" per `npm` permetterà l'esecuzione di tutti i successivi comandi npm senza interruzioni, quindi fallo solo per un progetto di cui ti fidi. Per installare package npm senza i relativi script, esegui `npm install --ignore-scripts`.

## Download

Su GitHub, la `latest release` offre tutte le build elencate qui sotto, mentre `all releases` contiene le versioni precedenti con le relative note. `<ver>` nei nomi dei file indica la versione, e `SHA256SUMS.txt` fornisce il checksum di ogni file.

| | Build | Installer | Portable |
| --- | --- | --- | --- |
| **Windows** | x64 (64-bit Intel/AMD) | `craft-anything-<ver>-windows-x64.msi` | `craft-anything-<ver>-windows-x64-portable.zip` |
| | arm64 (Snapdragon e altri) | `craft-anything-<ver>-windows-arm64.msi` | `craft-anything-<ver>-windows-arm64-portable.zip` |
| **macOS** | Apple Silicon (M1/M2/M3/M4) | `craft-anything-<ver>-macos-arm64.dmg` | `craft-anything-<ver>-macos-arm64.tar.gz` |
| | Intel | `craft-anything-<ver>-macos-x64.dmg` | `craft-anything-<ver>-macos-x64.tar.gz` |
| **Linux** | x64 AppImage | | `craft-anything-<ver>-linux-x64.AppImage` |
| | x64 .deb (Debian, Ubuntu, Mint) | `craft-anything-<ver>-linux-x64.deb` | |
| | arm64 AppImage | | `craft-anything-<ver>-linux-arm64.AppImage` |
| | arm64 .deb | `craft-anything-<ver>-linux-arm64.deb` | |
| **FreeBSD** | x64 | | `craft-anything-<ver>-freebsd-x64.tar.gz` |

> *Su macOS, l'app è firmata e autenticata (notarized) da Storytold Ltd.*

## Contribuire

I contributi sono benvenuti! Le rule che ogni modifica deve seguire (sia per le persone che per gli agent AI) si trovano in `AGENTS.md`; gli errori da cui derivano queste rule sono documentati in `docs/pitfalls.md`. Tutto il lavoro deve essere originale e svolto in modalità clean-room; non estrarre codice, nomi o testi da altri progetti. La roadmap in `docs/ROADMAP.md` mostra il piano attuale (oppure esegui `cargo xtask pending`). Prima di scrivere codice, rivendica la tua task per evitare sovrapposizioni e leggi le regole complete in [CONTRIBUTING.md](CONTRIBUTING.md).

## Licenza

Craft Anything è rilasciato con doppia licenza [MIT](LICENSE-MIT) o [Apache-2.0](LICENSE-APACHE), a tua scelta.

## Riconoscimenti

Vedi [ATTRIBUTION.md](ATTRIBUTION.md) per l'elenco dei progetti open source su cui si basa Craft Anything. Craft Anything non è affiliato con Anthropic, GitHub, Epic Games, Unity o con i creatori di alcun target ispezionato.
