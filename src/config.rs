use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

pub struct Config {
    /// Enable AUR features: helper search/install, AUR DB load, foreign-pkg detail fetch.
    pub aur: bool,
    pub theme_name: String,
    pub theme_bg: String,
    pub theme_fg: String,
    pub theme_border: String,
    pub theme_highlight_fg: String,
    pub theme_highlight_bg: String,
    pub theme_accent: String,
    pub theme_selected: String,
    pub theme_success: String,
    pub theme_warning: String,
    pub theme_error: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            aur: true,
            theme_name: "default".to_string(),
            theme_bg: "reset".to_string(),
            theme_fg: "white".to_string(),
            theme_border: "darkgray".to_string(),
            theme_highlight_fg: "black".to_string(),
            theme_highlight_bg: "cyan".to_string(),
            theme_accent: "cyan".to_string(),
            theme_selected: "yellow".to_string(),
            theme_success: "green".to_string(),
            theme_warning: "yellow".to_string(),
            theme_error: "red".to_string(),
        }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn cfg() -> &'static Config {
    CONFIG.get_or_init(load)
}

fn load() -> Config {
    let path = config_path();

    // Existing file is the source of truth: manual on/off is always respected.
    if let Some(p) = &path
        && let Ok(text) = std::fs::read_to_string(p) {
            return parse(&text);
        }

    // First run: seed a config. AUR on only if a helper is actually installed.
    let aur = installed_helper().is_some();
    let c = Config {
        aur,
        ..Default::default()
    };
    if path.is_some() {
        let _ = save_config(aur, &crate::theme::THEME_DEFAULT);
    }
    c
}

fn parse(text: &str) -> Config {
    let mut c = Config::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        let clean_val = v.trim().trim_matches('"').to_string();
        match k.trim() {
            "aur" => c.aur = parse_bool(&clean_val).unwrap_or(c.aur),
            "theme" => c.theme_name = clean_val,
            "theme_bg" => c.theme_bg = clean_val,
            "theme_fg" => c.theme_fg = clean_val,
            "theme_border" => c.theme_border = clean_val,
            "theme_highlight_fg" => c.theme_highlight_fg = clean_val,
            "theme_highlight_bg" => c.theme_highlight_bg = clean_val,
            "theme_accent" => c.theme_accent = clean_val,
            "theme_selected" => c.theme_selected = clean_val,
            "theme_success" => c.theme_success = clean_val,
            "theme_warning" => c.theme_warning = clean_val,
            "theme_error" => c.theme_error = clean_val,
            _ => {}
        }
    }
    c
}

pub fn save_config(aur: bool, theme: &crate::theme::Theme) -> std::io::Result<()> {
    let Some(path) = config_path() else { return Ok(()) };
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let values: [(&str, String); 12] = [
        ("aur", aur.to_string()),
        ("theme", format!("\"{}\"", theme.name)),
        ("theme_bg", format!("\"{}\"", crate::theme::color_to_string(theme.background))),
        ("theme_fg", format!("\"{}\"", crate::theme::color_to_string(theme.foreground))),
        ("theme_border", format!("\"{}\"", crate::theme::color_to_string(theme.border))),
        ("theme_highlight_fg", format!("\"{}\"", crate::theme::color_to_string(theme.highlight_fg))),
        ("theme_highlight_bg", format!("\"{}\"", crate::theme::color_to_string(theme.highlight_bg))),
        ("theme_accent", format!("\"{}\"", crate::theme::color_to_string(theme.accent))),
        ("theme_selected", format!("\"{}\"", crate::theme::color_to_string(theme.selected))),
        ("theme_success", format!("\"{}\"", crate::theme::color_to_string(theme.success))),
        ("theme_warning", format!("\"{}\"", crate::theme::color_to_string(theme.warning))),
        ("theme_error", format!("\"{}\"", crate::theme::color_to_string(theme.error))),
    ];

    let body = match std::fs::read_to_string(&path) {
        Ok(existing) => merge_config(&existing, &values),
        Err(_) => {
            let mut body = String::from(
                "# pkgman configuration\n\
                 # aur: enable AUR helper (yay/paru) features. Set false for pacman-only.\n",
            );
            for (i, (k, v)) in values.iter().enumerate() {
                if i == 1 {
                    body.push_str("\n# Theme Settings\n");
                }
                body.push_str(&format!("{} = {}\n", k, v));
            }
            body
        }
    };
    std::fs::write(path, body)
}

/// Update known keys in place, preserving comments, unknown keys, and
/// formatting of everything else. Missing keys are appended at the end.
fn merge_config(existing: &str, values: &[(&str, String)]) -> String {
    let mut written: Vec<bool> = vec![false; values.len()];
    let mut out = String::with_capacity(existing.len());

    for line in existing.lines() {
        let trimmed = line.trim();
        let key = if trimmed.is_empty() || trimmed.starts_with('#') {
            None
        } else {
            trimmed.split_once('=').map(|(k, _)| k.trim())
        };
        match key.and_then(|k| values.iter().position(|(vk, _)| *vk == k)) {
            Some(i) => {
                out.push_str(&format!("{} = {}\n", values[i].0, values[i].1));
                written[i] = true;
            }
            None => {
                out.push_str(line);
                out.push('\n');
            }
        }
    }

    for (i, (k, v)) in values.iter().enumerate() {
        if !written[i] {
            out.push_str(&format!("{} = {}\n", k, v));
        }
    }
    out
}

fn config_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join("pkgman").join("config.toml"))
}

fn parse_bool(v: &str) -> Option<bool> {
    match v {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn which(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn installed_helper() -> Option<&'static str> {
    ["paru", "yay"].into_iter().find(|h| which(h))
}

/// AUR helper to use, or None when AUR is disabled or no helper is installed.
pub fn aur_helper() -> Option<&'static str> {
    if !cfg().aur {
        return None;
    }
    installed_helper()
}
