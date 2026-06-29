use tauri::AppHandle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModifierShortcut {
    Ctrl,
    Alt,
    AltGr,
}

impl ModifierShortcut {
    fn label(self) -> &'static str {
        match self {
            Self::Ctrl => "Ctrl",
            Self::Alt => "Alt",
            Self::AltGr => "AltGr",
        }
    }
}

pub fn parse(shortcut: &str) -> Option<ModifierShortcut> {
    let normalized: String = shortcut
        .trim()
        .chars()
        .filter(|c| !matches!(c, ' ' | '-' | '_'))
        .flat_map(char::to_lowercase)
        .collect();

    match normalized.as_str() {
        "ctrl" | "control" | "strg" => Some(ModifierShortcut::Ctrl),
        "alt" => Some(ModifierShortcut::Alt),
        "altgr" | "altgraph" | "rightalt" | "ralt" => Some(ModifierShortcut::AltGr),
        _ => None,
    }
}

#[cfg(target_os = "windows")]
pub fn validate_supported(_shortcut: ModifierShortcut) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn validate_supported(shortcut: ModifierShortcut) -> Result<(), String> {
    Err(format!(
        "Modifier-only shortcut '{}' is only supported on Windows",
        shortcut.label()
    ))
}

#[cfg(target_os = "windows")]
pub fn configure(app: &AppHandle, shortcut: Option<ModifierShortcut>) -> Result<(), String> {
    windows::configure(app, shortcut)
}

#[cfg(not(target_os = "windows"))]
pub fn configure(_app: &AppHandle, shortcut: Option<ModifierShortcut>) -> Result<(), String> {
    match shortcut {
        Some(shortcut) => validate_supported(shortcut),
        None => Ok(()),
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use std::{
        sync::{Arc, Mutex, OnceLock},
        thread,
        time::Duration,
    };

    use tauri::AppHandle;
    use tauri_plugin_global_shortcut::ShortcutState;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_LCONTROL, VK_LMENU, VK_RCONTROL, VK_RMENU,
    };

    use super::ModifierShortcut;

    struct ListenerState {
        app: Option<AppHandle>,
        shortcut: Option<ModifierShortcut>,
        last_pressed: bool,
    }

    static STATE: OnceLock<Arc<Mutex<ListenerState>>> = OnceLock::new();

    pub fn configure(app: &AppHandle, shortcut: Option<ModifierShortcut>) -> Result<(), String> {
        let state = listener_state();
        {
            let mut state = state
                .lock()
                .map_err(|_| "Modifier shortcut state poisoned")?;
            state.app = Some(app.clone());
            state.shortcut = shortcut;
            state.last_pressed = false;
        }

        match shortcut {
            Some(shortcut) => log::info!(
                "Modifier-only shortcut listener enabled: {}",
                shortcut.label()
            ),
            None => log::debug!("Modifier-only shortcut listener disabled"),
        }

        Ok(())
    }

    fn listener_state() -> Arc<Mutex<ListenerState>> {
        STATE
            .get_or_init(|| {
                let state = Arc::new(Mutex::new(ListenerState {
                    app: None,
                    shortcut: None,
                    last_pressed: false,
                }));
                spawn_listener(Arc::clone(&state));
                state
            })
            .clone()
    }

    fn spawn_listener(state: Arc<Mutex<ListenerState>>) {
        thread::Builder::new()
            .name("modifier-shortcut-listener".to_string())
            .spawn(move || loop {
                thread::sleep(Duration::from_millis(20));

                let snapshot = {
                    let Ok(state) = state.lock() else {
                        log::error!("Modifier shortcut listener state is poisoned");
                        return;
                    };
                    (state.app.clone(), state.shortcut, state.last_pressed)
                };

                let (Some(app), Some(shortcut), last_pressed) = snapshot else {
                    continue;
                };

                let pressed = is_pressed(shortcut);
                if pressed == last_pressed {
                    continue;
                }

                {
                    let Ok(mut state) = state.lock() else {
                        log::error!("Modifier shortcut listener state is poisoned");
                        return;
                    };
                    state.last_pressed = pressed;
                }

                let shortcut_state = if pressed {
                    ShortcutState::Pressed
                } else {
                    ShortcutState::Released
                };
                crate::os::hotkeys::handle_shortcut_state(&app, shortcut_state);
            })
            .expect("failed to spawn modifier shortcut listener");
    }

    fn is_pressed(shortcut: ModifierShortcut) -> bool {
        match shortcut {
            ModifierShortcut::Ctrl => {
                !key_down(VK_RMENU) && (key_down(VK_LCONTROL) || key_down(VK_RCONTROL))
            }
            ModifierShortcut::Alt => key_down(VK_LMENU),
            ModifierShortcut::AltGr => key_down(VK_RMENU),
        }
    }

    fn key_down(vk: u16) -> bool {
        unsafe { (GetAsyncKeyState(vk as i32) as u16 & 0x8000) != 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, ModifierShortcut};

    #[test]
    fn parses_modifier_only_shortcuts() {
        assert_eq!(parse("Ctrl"), Some(ModifierShortcut::Ctrl));
        assert_eq!(parse("Control"), Some(ModifierShortcut::Ctrl));
        assert_eq!(parse("Strg"), Some(ModifierShortcut::Ctrl));
        assert_eq!(parse("Alt"), Some(ModifierShortcut::Alt));
        assert_eq!(parse("AltGr"), Some(ModifierShortcut::AltGr));
        assert_eq!(parse("Alt Graph"), Some(ModifierShortcut::AltGr));
    }

    #[test]
    fn ignores_non_modifier_shortcuts() {
        assert_eq!(parse("F8"), None);
        assert_eq!(parse("Ctrl+Space"), None);
        assert_eq!(parse("CommandOrControl+Shift+Space"), None);
    }
}
