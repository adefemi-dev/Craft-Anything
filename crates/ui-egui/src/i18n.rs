//! UI localisation: English strings in code are the lookup keys; a per-language
//! TSV catalog (`key<TAB>translation` per line, `#` comments) maps them to display text at render
//! time. A string with no translation is shown in English. Tool ids, the CLI and MCP never see
//! translated text.
//!
//! Adding a language: put `xx.tsv` in `i18n/` and add one row to [`LANGUAGES`].

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// One supported UI language.
pub struct LangInfo {
    /// BCP 47 code, lowercase. Also the `settings.language` value.
    pub code: &'static str,
    /// The language's name in itself.
    pub name: &'static str,
    /// Catalog contents (empty for the built-in English).
    pub source: &'static str,
}

pub static LANGUAGES: &[LangInfo] = &[
    LangInfo { code: "en", name: "English", source: "" },
    LangInfo { code: "de", name: "Deutsch", source: include_str!("i18n/de.tsv") },
    LangInfo { code: "ka", name: "ქართული", source: include_str!("i18n/ka.tsv") },
    LangInfo { code: "fr", name: "Français", source: include_str!("fr.tsv") },
    LangInfo { code: "it", name: "Italiano", source: include_str!("i18n/it.tsv") },
    LangInfo { code: "ja", name: "日本語", source: include_str!("ja.tsv") },
    LangInfo { code: "zh-hans", name: "简体中文", source: include_str!("i18n/zh-hans.tsv") },
    LangInfo { code: "zh-hant", name: "繁體中文", source: include_str!("i18n/zh-hant.tsv") },
    LangInfo { code: "ru", name: "Русский", source: include_str!("i18n/ru.tsv") },
    LangInfo { code: "ko", name: "한국어", source: include_str!("i18n/ko.tsv") },
    LangInfo { code: "cs", name: "Čeština", source: include_str!("i18n/cs.tsv") },
    LangInfo { code: "es", name: "Español", source: include_str!("i18n/es.tsv") },
    LangInfo { code: "vi", name: "Tiếng Việt", source: include_str!("i18n/vi.tsv") },
    LangInfo { code: "pt-br", name: "Português (Brasil)", source: include_str!("i18n/pt-br.tsv") },
    LangInfo { code: "tr", name: "Türkçe", source: include_str!("i18n/tr.tsv") },
    LangInfo { code: "th", name: "ไทย", source: include_str!("i18n/th.tsv") },
    LangInfo { code: "et", name: "Eesti", source: include_str!("i18n/et.tsv") },
    LangInfo { code: "he", name: "עברית", source: include_str!("i18n/he.tsv") },
];

/// Match a locale to a registered UI language. An explicit script wins over the region.
pub fn language_from_tag(tag: &str) -> Option<&'static str> {
    let tag = tag.trim().split(['.', '@']).next().unwrap_or_default().replace('_', "-").to_ascii_lowercase();
    if let Some(lang) = LANGUAGES.iter().find(|l| l.code == tag) {
        return Some(lang.code);
    }
    let mut parts = tag.split('-');
    let primary = match parts.next()? {
        "iw" => "he",
        other => other,
    };
    match primary {
        "zh" => {
            let parts: Vec<_> = parts.take_while(|p| p.len() != 1).collect();
            if parts.contains(&"hant") {
                Some("zh-hant")
            } else if parts.contains(&"hans") {
                Some("zh-hans")
            } else if parts.iter().any(|p| matches!(*p, "tw" | "hk" | "mo")) {
                Some("zh-hant")
            } else {
                Some("zh-hans")
            }
        }
        "pt" => Some("pt-br"),
        _ => LANGUAGES.iter().find(|l| l.code == primary).map(|l| l.code),
    }
}

fn preferred_language(tags: impl IntoIterator<Item = impl AsRef<str>>) -> &'static str {
    tags.into_iter().find_map(|tag| language_from_tag(tag.as_ref())).unwrap_or("en")
}

fn system_language() -> &'static str {
    #[cfg(not(target_arch = "wasm32"))]
    {
        static SYSTEM: OnceLock<&'static str> = OnceLock::new();
        SYSTEM.get_or_init(|| preferred_language(sys_locale::get_locales()))
    }
    #[cfg(target_arch = "wasm32")]
    {
        preferred_language(std::iter::empty::<&str>())
    }
}

fn current() -> &'static RwLock<(String, HashMap<String, String>)> {
    static CUR: OnceLock<RwLock<(String, HashMap<String, String>)>> = OnceLock::new();
    CUR.get_or_init(|| RwLock::new(("en".into(), HashMap::new())))
}

fn parse(source: &str) -> HashMap<String, String> {
    source
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .filter_map(|l| l.split_once('\t'))
        .filter(|(k, v)| !k.is_empty() && !v.trim().is_empty())
        .map(|(k, v)| (k.replace("\\n", "\n"), v.trim_end().replace("\\n", "\n")))
        .collect()
}

/// Switch language. Empty means the system's preferred language; unsupported codes mean English.
pub fn set_language(code: &str) {
    let code = if code.trim().is_empty() { system_language() } else { language_from_tag(code).unwrap_or("en") };
    let lang = LANGUAGES.iter().find(|l| l.code == code);
    let map = lang.map(|l| parse(l.source)).unwrap_or_default();
    if let Ok(mut g) = current().write() {
        *g = (lang.map(|l| l.code).unwrap_or("en").to_string(), map);
    }
}

pub fn language() -> String {
    current().read().map(|g| g.0.clone()).unwrap_or_else(|_| "en".into())
}

/// Whether `code` names a right-to-left script or language (accepting legacy `iw` tags as Hebrew
/// and honoring explicit ISO 15924 script subtags before language defaults).
pub fn is_rtl_code(code: &str) -> bool {
    let tag = code.trim().split(['.', '@']).next().unwrap_or_default().replace('_', "-").to_ascii_lowercase();
    let mut parts = tag.split('-');
    let base = parts.next().unwrap_or("");
    if base == "x" {
        return false;
    }
    let next = parts.find(|p| !(p.len() == 3 && p.bytes().all(|b| b.is_ascii_alphabetic())));
    if let Some(script) = next.filter(|p| p.len() == 4 && p.bytes().all(|b| b.is_ascii_alphabetic())) {
        return matches!(script, "arab" | "hebr" | "nkoo" | "rohg" | "syrc" | "thaa" | "yezi");
    }
    matches!(base, "ar" | "ckb" | "dv" | "fa" | "he" | "iw" | "ps" | "sd" | "ug" | "ur" | "yi")
}

/// Whether the active UI language uses right-to-left script direction.
pub fn is_rtl() -> bool {
    is_rtl_code(&language())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BidiClass {
    L,
    R,
    En,
    N,
}

fn is_rtl_script(c: char) -> bool {
    matches!(
        u32::from(c),
        0x0590..=0x05FF | 0x0600..=0x065F | 0x066E..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFC | 0x1_0800..=0x1_0FFF | 0x1_E800..=0x1_EFFF
    )
}

fn is_bidi_digit(c: char) -> bool {
    c.is_ascii_digit() || matches!(u32::from(c), 0x0660..=0x0669 | 0x06F0..=0x06F9)
}

fn is_bidi_control(c: char) -> bool {
    matches!(c, '\u{061C}' | '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}')
}

fn is_joiner(c: char) -> bool {
    matches!(c, '\u{200C}' | '\u{200D}')
}

fn mirrored(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        '«' => '»',
        '»' => '«',
        other => other,
    }
}

fn bracket_close(open: char) -> Option<char> {
    match open {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        '«' => Some('»'),
        _ => None,
    }
}

fn is_closing_bracket(c: char) -> bool {
    matches!(c, ')' | ']' | '}' | '>' | '»')
}

const PLACEHOLDER_BASE: u32 = 0xE000;
const MAX_PLACEHOLDERS: usize = 64;

fn mask_placeholders(text: &str) -> (String, Vec<&str>) {
    let reserved = text.chars().any(|c| (PLACEHOLDER_BASE..PLACEHOLDER_BASE.saturating_add(MAX_PLACEHOLDERS as u32)).contains(&u32::from(c)));
    if reserved {
        return (text.to_owned(), Vec::new());
    }
    let mut masked = String::with_capacity(text.len());
    let mut names = Vec::new();
    let mut rest = text;
    while names.len() < MAX_PLACEHOLDERS
        && let Some(start) = rest.find('{')
        && let Some(len) = rest.get(start..).and_then(|s| s.find('}'))
    {
        let end = start.saturating_add(len).saturating_add(1);
        let (Some(before), Some(atom), Some(after)) = (rest.get(..start), rest.get(start..end), rest.get(end..)) else {
            break;
        };
        let Some(stand_in) = u32::try_from(names.len()).ok().and_then(|i| char::from_u32(PLACEHOLDER_BASE.saturating_add(i))) else {
            break;
        };
        masked.push_str(before);
        masked.push(stand_in);
        names.push(atom);
        rest = after;
    }
    masked.push_str(rest);
    (masked, names)
}

fn push_unmasked(out: &mut String, c: char, names: &[&str]) {
    let idx = u32::from(c).checked_sub(PLACEHOLDER_BASE).and_then(|i| usize::try_from(i).ok());
    if let Some(name) = idx.and_then(|i| names.get(i)) {
        out.push_str(name);
    } else {
        out.push(c);
    }
}

fn push_visual_rtl_line(line: &str, out: &mut String) {
    let (masked, names) = mask_placeholders(line);
    let chars: Vec<char> = masked.chars().filter(|c| !is_bidi_control(*c)).collect();
    if chars.is_empty() {
        return;
    }
    let len = chars.len();
    let mut cls: Vec<BidiClass> = chars
        .iter()
        .map(|&c| {
            let is_placeholder = (PLACEHOLDER_BASE..PLACEHOLDER_BASE.saturating_add(MAX_PLACEHOLDERS as u32)).contains(&u32::from(c));
            if is_rtl_script(c) {
                BidiClass::R
            } else if is_bidi_digit(c) {
                BidiClass::En
            } else if is_placeholder || c.is_alphabetic() || matches!(c, '\u{2318}' | '\u{21E7}' | '\u{2325}' | '\u{2303}') {
                BidiClass::L
            } else {
                BidiClass::N
            }
        })
        .collect();

    // Keep non-space separators (.,:/-+_) inside numeric and LTR tokens.
    for i in 1..len.saturating_sub(1) {
        if cls.get(i) == Some(&BidiClass::N)
            && chars.get(i).is_some_and(|c| matches!(c, '.' | ',' | ':' | '/' | '-' | '+' | '_'))
            && let (Some(prev), Some(next)) = (cls.get(i - 1), cls.get(i + 1))
        {
            if *prev == BidiClass::En && *next == BidiClass::En {
                if let Some(slot) = cls.get_mut(i) {
                    *slot = BidiClass::En;
                }
            } else if matches!(prev, BidiClass::L | BidiClass::En)
                && matches!(next, BidiClass::L | BidiClass::En)
                && let Some(slot) = cls.get_mut(i)
            {
                *slot = BidiClass::L;
            }
        }
    }

    // Attach adjacent European terminators (e.g. %, $, px units, leading sign when not attached to RTL).
    for i in 0..len {
        let Some(&ch) = chars.get(i) else { continue };
        if cls.get(i) != Some(&BidiClass::N) {
            continue;
        }
        let next_en = cls.get(i + 1) == Some(&BidiClass::En);
        let prev_en = i > 0 && cls.get(i - 1) == Some(&BidiClass::En);
        let prev_is_r = i > 0 && cls.get(i - 1) == Some(&BidiClass::R);
        let attach = match ch {
            '%' | '°' | '$' | '€' | '£' | '₪' => prev_en || next_en,
            '+' | '-' => next_en && !prev_is_r,
            _ => false,
        };
        if attach && let Some(slot) = cls.get_mut(i) {
            *slot = BidiClass::En;
        }
    }

    // Resolve paired brackets (simplified UAX #9 N0).
    let mut stack: Vec<(usize, char)> = Vec::new();
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for (i, &c) in chars.iter().enumerate() {
        if cls.get(i) != Some(&BidiClass::N) {
            continue;
        }
        if let Some(close) = bracket_close(c) {
            if stack.len() < 63 {
                stack.push((i, close));
            }
        } else if is_closing_bracket(c)
            && let Some(pos) = stack.iter().rposition(|(_, expected)| *expected == c)
        {
            let (open_idx, _) = stack.remove(pos);
            pairs.push((open_idx, i));
        }
    }
    pairs.sort_by_key(|(open, _)| *open);
    for (open, close) in pairs {
        let inner = cls.get(open + 1..close).unwrap_or(&[]);
        let has_r = inner.contains(&BidiClass::R);
        let has_l = inner.iter().any(|c| matches!(c, BidiClass::L | BidiClass::En));
        let dir = if has_r {
            BidiClass::R
        } else if has_l {
            let prev_strong = cls.get(..open).unwrap_or(&[]).iter().rev().find(|c| **c != BidiClass::N).copied();
            if matches!(prev_strong, Some(BidiClass::L | BidiClass::En)) { BidiClass::L } else { BidiClass::R }
        } else {
            BidiClass::R
        };
        if let Some(s) = cls.get_mut(open) {
            *s = dir;
        }
        if let Some(s) = cls.get_mut(close) {
            *s = dir;
        }
    }

    // Fold En -> L, then resolve runs of N flanked on both sides by L to L; everything else to R.
    for c in &mut cls {
        if *c == BidiClass::En {
            *c = BidiClass::L;
        }
    }
    let mut i = 0;
    while i < len {
        if cls.get(i) == Some(&BidiClass::N) {
            let start = i;
            while cls.get(i) == Some(&BidiClass::N) {
                i += 1;
            }
            let left_l = start > 0 && cls.get(start - 1) == Some(&BidiClass::L);
            let right_l = cls.get(i) == Some(&BidiClass::L);
            let resolved = if left_l && right_l { BidiClass::L } else { BidiClass::R };
            if let Some(slice) = cls.get_mut(start..i) {
                slice.fill(resolved);
            }
        } else {
            i += 1;
        }
    }

    // Partition into maximal RTL vs LTR runs, then emit in visual RTL order (last run first).
    let mut runs: Vec<(bool, usize, usize)> = Vec::new();
    let mut r_start = 0;
    while r_start < len {
        let rtl = cls.get(r_start) == Some(&BidiClass::R);
        let mut r_end = r_start + 1;
        while cls.get(r_end).is_some_and(|c| (*c == BidiClass::R) == rtl) {
            r_end += 1;
        }
        runs.push((rtl, r_start, r_end));
        r_start = r_end;
    }

    for (rtl, start, end) in runs.into_iter().rev() {
        let Some(slice) = chars.get(start..end) else { continue };
        if rtl {
            // Walk visually left-to-right (reverse of logical); keep each contiguous RTL-script
            // stretch in logical order so epaint's harfrust shapes that stretch RTL.
            let mut stretch: Vec<char> = Vec::new();
            for &c in slice.iter().rev() {
                if is_rtl_script(c) || is_joiner(c) {
                    stretch.push(c);
                } else {
                    for sc in stretch.drain(..).rev() {
                        push_unmasked(out, sc, &names);
                    }
                    push_unmasked(out, mirrored(c), &names);
                }
            }
            for sc in stretch.drain(..).rev() {
                push_unmasked(out, sc, &names);
            }
        } else {
            for &c in slice {
                push_unmasked(out, c, &names);
            }
        }
    }
}

/// Rearrange `text` into visual right-to-left order for egui 0.36 while keeping contiguous RTL
/// script stretches in logical order for harfrust shaping.
fn visual_rtl(text: &str) -> String {
    if !text.chars().any(is_rtl_script) {
        return text.to_owned();
    }
    let mut out = String::with_capacity(text.len());
    for (idx, raw_line) in text.split('\n').enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        let (line, cr) = raw_line.strip_suffix('\r').map_or((raw_line, false), |l| (l, true));
        push_visual_rtl_line(line, &mut out);
        if cr {
            out.push('\r');
        }
    }
    out
}

fn tr_raw(s: &str) -> String {
    match current().read() {
        Ok(g) => g.1.get(s).cloned().unwrap_or_else(|| s.to_string()),
        Err(_) => s.to_string(),
    }
}

/// Translate a UI string.
pub fn tr(s: &str) -> String {
    let raw = tr_raw(s);
    if is_rtl() { visual_rtl(&raw) } else { raw }
}

/// Translate, then fill `{name}` placeholders in one pass (`platform::text::fill`), so an inserted
/// value stays literal even when it looks like a placeholder.
pub fn trf(s: &str, args: &[(&str, &str)]) -> String {
    let template = tr_raw(s);
    let filled = if template.contains('{') { craft_anything_platform::text::fill(&template, args) } else { template };
    if is_rtl() { visual_rtl(&filled) } else { filled }
}

#[cfg(test)]
mod tests {
    use super::*;

    static LANG_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn catalog_parse_and_fallback() {
        let _guard = LANG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        set_language("en");
        let m = parse("# comment\nHello\tHola\nEmpty\t \nbad line\n");
        assert_eq!(m.get("Hello").map(String::as_str), Some("Hola"));
        assert!(!m.contains_key("Empty"));
        assert_eq!(trf("Open {name}", &[("name", "a.exe")]), "Open a.exe");
        assert_eq!(trf("{a} {b}", &[("a", "{b}"), ("b", "x")]), "{b} x");
        assert_eq!(tr("Untranslated"), "Untranslated");
        assert_eq!(trf("Untranslated", &[("a", "1")]), "Untranslated");
    }

    #[test]
    fn hebrew_tr_and_trf_reorder_into_visual_rtl_order() {
        let _guard = LANG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        set_language("he");
        assert!(is_rtl());
        // Single Hebrew word stays logical (epaint/harfrust shapes the single RTL run).
        assert_eq!(tr("Apply"), "החלה");
        // Multi-word Hebrew reorders words for visual right-to-left display.
        assert_eq!(tr("Command palette"), "פקודות לוח");
        // Mixed Hebrew + Latin + numbers + parentheses keep LTR segments intact and mirror RTL brackets.
        assert_eq!(tr("Export a report (Markdown)"), "(Markdown) דוח ייצוא");
        assert_eq!(tr("Command palette (Ctrl/⌘ K)"), "(Ctrl/⌘ K) פקודות לוח");
        assert_eq!(visual_rtl("תמונה 1 מתוך 3"), "3 מתוך 1 תמונה");
        assert_eq!(visual_rtl("(תמונה 1 מצורפת)"), "(מצורפת 1 תמונה)");
        // Placeholder fill keeps inserted values literal before visual RTL reordering.
        assert_eq!(trf("Open {name}", &[("name", "demo.exe")]), "demo.exe פתיחת");
        set_language("en");
    }

    /// Script and region matching: an explicit script wins over a region, a registered language
    /// resolves from any of its regions, and a locale with no catalogue falls back to English.
    #[test]
    fn chinese_scripts_regions_and_fallback() {
        for tag in ["zh", "zh-CN", "zh_SG.UTF-8", "ZH-Hans", "zh-Hans-TW"] {
            assert_eq!(language_from_tag(tag), Some("zh-hans"), "{tag}");
        }
        for tag in ["zh-TW", "zh-HK", "zh-MO", "zh-Hant", "zh-Hant-CN"] {
            assert_eq!(language_from_tag(tag), Some("zh-hant"), "{tag}");
        }
        assert_eq!(language_from_tag("en-US"), Some("en"));
        assert_eq!(language_from_tag("ja-JP"), Some("ja"));
        assert_eq!(language_from_tag("fr-CA"), Some("fr"));
        assert_eq!(language_from_tag("de-DE"), Some("de"));
        assert_eq!(language_from_tag("de_AT.UTF-8"), Some("de"));
        assert_eq!(language_from_tag("es-ES"), Some("es"));
        assert_eq!(language_from_tag("es-MX"), Some("es"));
        assert_eq!(language_from_tag("sv-SE"), None);
        assert_eq!(preferred_language(["sv-SE", "zh-HK", "en-US"]), "zh-hant");
        assert_eq!(language_from_tag("vi-VN"), Some("vi"));
        assert_eq!(language_from_tag("vi_VN.UTF-8"), Some("vi"));
        assert_eq!(language_from_tag("pt_BR.UTF-8"), Some("pt-br"));
        assert_eq!(language_from_tag("pt-PT"), Some("pt-br"));
        assert_eq!(language_from_tag("tr-TR"), Some("tr"));
        assert_eq!(language_from_tag("th-TH"), Some("th"));
        assert_eq!(language_from_tag("th_TH.UTF-8"), Some("th"));
        assert_eq!(language_from_tag("he"), Some("he"));
        assert_eq!(language_from_tag("he-IL"), Some("he"));
        assert_eq!(language_from_tag("iw"), Some("he"));
        assert_eq!(language_from_tag("iw-IL"), Some("he"));
        assert_eq!(language_from_tag("iw_IL.UTF-8"), Some("he"));
        assert!(is_rtl_code("he"));
        assert!(is_rtl_code("iw"));
        assert!(is_rtl_code("iw-IL"));
        assert!(is_rtl_code("he-Hebr-IL"));
        assert!(is_rtl_code("he-u-nu-latn"));
        assert!(is_rtl_code("ckb"));
        assert!(is_rtl_code("ku-Arab-IQ"));
        assert!(!is_rtl_code("ku"));
        assert!(!is_rtl_code("ku-Latn-TR"));
        assert!(!is_rtl_code("sd-Deva-IN"));
        assert!(!is_rtl_code("en-x-hebr"));
        assert!(!is_rtl_code("x-Arab"));
        assert!(!is_rtl_code("x-Hebr"));
        assert!(!is_rtl_code("en"));
        assert_eq!(preferred_language(["de-DE", "zh-HK"]), "de");
        assert_eq!(preferred_language(["sv-SE", "vi-VN", "en-US"]), "vi");
        assert_eq!(preferred_language(["en-US", "zh-CN"]), "en");
        assert_eq!(preferred_language(["sv-SE"]), "en");
        assert_eq!(preferred_language(std::iter::empty::<&str>()), "en");
    }

    /// Every registered catalogue parses, and all of them carry the same key set: a language that
    /// missed keys would fall back to English mid-interface.
    #[test]
    fn catalogs_are_valid_and_have_matching_keys() {
        fn placeholders(s: &str) -> Vec<&str> {
            let mut out: Vec<_> = s.split('{').skip(1).filter_map(|s| s.split_once('}').map(|(p, _)| p)).collect();
            out.sort_unstable();
            out
        }
        let mut catalogs = Vec::new();
        for lang in LANGUAGES
            .iter()
            .filter(|l| matches!(l.code, "zh-hans" | "zh-hant" | "ru" | "ko" | "cs" | "de" | "ka" | "es" | "it" | "vi" | "pt-br" | "tr" | "th" | "et" | "he"))
        {
            let mut keys = std::collections::BTreeSet::new();
            for (i, line) in lang.source.lines().enumerate() {
                if line.trim().is_empty() || line.starts_with('#') {
                    continue;
                }
                let (key, value) = line.split_once('\t').unwrap_or_else(|| panic!("{}:{}: missing tab", lang.code, i + 1));
                assert!(!key.is_empty() && !value.trim().is_empty(), "{}:{}: empty column", lang.code, i + 1);
                assert!(!value.contains('\t'), "{}:{}: extra column", lang.code, i + 1);
                assert!(keys.insert(key.to_string()), "{}:{}: duplicate {key}", lang.code, i + 1);
                assert_eq!(placeholders(key), placeholders(value), "{}: {key}", lang.code);
                assert_eq!(key.ends_with('…'), value.ends_with('…'), "{}: {key}", lang.code);
            }
            let parsed = parse(lang.source);
            assert_eq!(parsed.len(), keys.len());
            assert!(keys.len() > 500, "{}: incomplete catalog", lang.code);
            catalogs.push(keys);
        }
        assert!(catalogs.len() >= 2);
        for c in catalogs.iter().skip(1) {
            assert_eq!(catalogs.first(), Some(c));
        }
    }

    /// Every translation keeps its key's `{placeholders}`, or `trf` would leave them unfilled.
    #[test]
    fn catalogs_keep_placeholders() {
        let holes = |s: &str| {
            let mut v: Vec<String> = s.split('{').skip(1).filter_map(|p| p.split_once('}')).map(|(h, _)| h.to_string()).collect();
            v.sort();
            v
        };
        for l in LANGUAGES {
            let m = parse(l.source);
            assert!(l.source.is_empty() || m.len() > 100, "{} catalog barely parsed", l.code);
            for (k, v) in &m {
                assert_eq!(holes(k), holes(v), "{}: {k:?}", l.code);
            }
        }
    }
}
