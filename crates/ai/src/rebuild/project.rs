//! Starting a project from a request: the target, the folder, the stack the person asked for, the
//! asset budget from settings and whether assets may be generated. Shared by the CLI's `rebuild` and the `rebuild_start` tool.

use super::*;
use craft_anything_engine::Settings;
use craft_anything_engine::workspace::{ExistingFiles, suggest_name};

/// Where a new project goes.
#[derive(Debug, Clone, Copy, Default)]
pub enum Place<'a> {
    /// A new folder in the projects folder from settings.
    #[default]
    Projects,
    /// A new folder inside this one.
    In(&'a Path),
    /// This folder itself ([`Workspace::create_at`]), with what its files are if it has any.
    At(&'a Path, Option<ExistingFiles>),
}

/// A stack picked by name for `--build-with` / `build_with` (an engine id or a stack's full name).
pub fn build_with(name: &str) -> Result<&'static stacks::Stack, ToolError> {
    stacks::resolve(name).ok_or_else(|| {
        let known = stacks::catalogue().iter().map(|s| s.engine.clone().unwrap_or_else(|| s.name.clone())).collect::<Vec<_>>().join(", ");
        ToolError::invalid(format!("Unknown build-with {name:?}.")).with_remediation(format!("Use one of: {known}."))
    })
}

/// The stack a project starts with. One edited in place keeps what its files are built with,
/// unless its request names another; any other goes by its request and target.
pub(super) fn first_stack(kinds: &KindRegistry, ws: &Workspace, deep: bool) -> Option<&'static str> {
    let m = &ws.manifest;
    let own = m.app_in_root && kinds.named(&m.request, &m.target).is_none();
    let found = own.then(|| stacks::found_in(&ws.app_dir())).flatten();
    found.map(|s| s.name.as_str()).or_else(|| kinds.choose(&m.request, &m.target, deep).map(|(_, stack)| stack))
}

/// A new game built with the builder's choice instead of its default engine: the request names a
/// game technology from the table's list (`[[game_tech]]`), no engine or game stack in any form and no
/// stack the way the table's phrases name one ("in Python"), and the default
/// engine isn't on this computer (R2.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineSkipped {
    /// The engine the game would have been built with.
    pub engine: game::Builder,
    /// The technology the request names, as the list shows it ("Three.js").
    pub named: &'static str,
}

impl EngineSkipped {
    /// What the person is told, with how to get the engine back.
    pub fn notice(&self) -> String {
        let engine = self.engine.name();
        let id = stacks::get(self.engine.stack()).and_then(|s| s.engine.as_deref()).unwrap_or(engine);
        format!(
            "{engine} isn't installed, so this game is built with what your request names ({}) instead. To build it with {engine}, get {engine} (Settings → Engines in the app) and start a new project with \"Build with\" set to {engine} (`--build-with {id}` in the CLI).",
            self.named
        )
    }
}

/// Whether a new game whose stack would be its kind's default `engine`
/// (nothing picked, nothing named) is built with the builder's choice instead: only when the request names a technology from the
/// table's list and `installed` (asked last, as it looks through the computer) says the engine isn't
/// on this computer at all: an engine that is here but not ready (Unreal without its C++ tools)
/// keeps its own way to finish setting it up. A request that names
/// nothing keeps its engine, so the person is still offered to get it.
pub fn listed_tech_instead(
    kinds: &KindRegistry,
    request: &str,
    target: &TargetRef,
    engine: game::Builder,
    installed: impl FnOnce(game::Builder) -> bool,
) -> Option<EngineSkipped> {
    if engine == game::Builder::Rust {
        return None;
    }
    // A game in the request's own words (a picked folder or a ticked box alone isn't kept: the run
    // decides the kind from the request and the stack, and Builder's choice is no engine).
    if kinds.detect(request, target, "")?.id() != GameKind::ID || kinds.named(request, target).is_some() {
        return None;
    }
    let words = stacks::words(request);
    // An engine or a game stack named any way at all ("a Unity WebGL game", "Unity3D", "UE5",
    // "a Phaser game", "a Rust game"), not only "in Unity": nothing changes here. When the table's
    // phrases don't catch the word, that means the kind's default as before, so "a Phaser game
    // with WebGL" still gets Unity and stops for it if it's missing.
    if words.split_whitespace().any(names_an_engine_or_game_stack) {
        return None;
    }
    let named = stacks::game_tech_named(&words)?;
    if installed(engine) {
        return None;
    }
    Some(EngineSkipped { engine, named: named.name.as_str() })
}

/// Whether one word of a request (through [`stacks::words`]) names a game engine or a game stack the
/// table has, with or without a version ("unity", "unity2d", "godot4", "ue5", "phaser3", "bevy"):
/// such a request is left as it was without this rule (the table's phrases may still not pick that
/// stack, so "a Phaser game with WebGL" keeps the default engine), and Builder's choice isn't put in
/// its place. Other stacks
/// named only as a word ("a Python game with Pygame") don't count: the builder then uses what the
/// request names, as Builder's choice would.
fn names_an_engine_or_game_stack(word: &str) -> bool {
    let version = |rest: &str| ["2d", "3d"].contains(&rest) || rest.chars().all(|c| c.is_ascii_digit());
    ["unity", "unreal", "godot", "ue", "phaser", "bevy", "rust", "macroquad"].iter().any(|e| word.strip_prefix(e).is_some_and(version))
}

/// Whether `folder` already holds a project (its `craft.json` is there).
pub fn has_project(folder: &Path) -> bool {
    folder.join(craft_anything_engine::workspace::MANIFEST).is_file()
}

/// What a run says after opening a folder: a request typed for one that already held a project
/// is a follow-up, not a new project's request.
pub fn attached_request(attached: bool, request: &str) -> Option<String> {
    attached.then(|| request.to_string())
}

/// What a created project starts with: its stack, the asset budget from settings, whether its
/// pictures and models may be generated, and its builder. A folder that already held a project
/// keeps all of them; attaching never rewrites what the person set.
pub fn apply_new_project_fields(
    ws: &mut Workspace,
    attached: bool,
    stack: Option<&str>,
    budget: f64,
    generate_assets: bool,
    builder: Option<&str>,
) -> Result<(), ToolError> {
    if attached {
        return Ok(());
    }
    if let Some(name) = stack {
        ws.manifest.stack = name.to_string();
    }
    ws.set_person(|m| {
        m.asset_budget_usd = budget;
        m.generate_assets = generate_assets;
        m.builder = builder.map(str::to_string);
    })
}

/// A new project for `request` about `target` (a web address, an absolute path, or `-` for none),
/// at `place`, with the stack the person
/// picked (`build_with`), the asset budget from settings, and `generate_assets`: the person lets the
/// builder generate pictures and 3D models.
pub fn new_project(
    settings: &Settings,
    target: &str,
    request: &str,
    place: Place,
    build_with: Option<&str>,
    generate_assets: bool,
    builder: Option<&str>,
) -> Result<Workspace, ToolError> {
    let stack = build_with.map(self::build_with).transpose()?;
    let target_ref = if target.trim() == "-" { TargetRef::None } else { TargetRef::parse(target) };
    if target_ref == TargetRef::None && target.trim() != "-" {
        return Err(ToolError::invalid(format!("{target:?} is neither a web address nor an existing absolute path"))
            .with_remediation("Pass https://… for a website, a full path for a program, or - for none."));
    }
    let name = suggest_name(request, &target_ref);
    // A folder that already holds a project is attached, not recreated: its saved stack and the
    // fields the person owns stay as they are.
    let attached = matches!(&place, Place::At(folder, _) if has_project(folder));
    let mut ws = match place {
        Place::Projects => Workspace::create(&settings.projects_dir(), &name, request, target_ref)?,
        Place::In(dir) => Workspace::create(dir, &name, request, target_ref)?,
        Place::At(folder, existing) => {
            if attached {
                Workspace::open(folder)?
            } else {
                Workspace::create_at(folder, &name, request, target_ref, existing)?
            }
        }
    };
    apply_new_project_fields(&mut ws, attached, stack.map(|s| s.name.as_str()), settings.assets.budget(), generate_assets, builder)?;
    Ok(ws)
}
