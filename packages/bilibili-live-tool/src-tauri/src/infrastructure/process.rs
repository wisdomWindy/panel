//! Spawning console programs from a GUI application.
//!
//! On Windows a GUI process has no console, so every console child (Python,
//! FFmpeg, tar) would otherwise allocate one and show up as a black window in
//! front of the application.

/// `DETACHED_PROCESS | CREATE_NO_WINDOW`.
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000 | 0x0000_0008;

pub fn quiet(program: impl AsRef<std::ffi::OsStr>) -> std::process::Command {
    let mut command = std::process::Command::new(gui_interpreter(program.as_ref()));
    hide_console(&mut command);
    command
}

pub fn quiet_tokio(program: impl AsRef<std::ffi::OsStr>) -> tokio::process::Command {
    let mut command = tokio::process::Command::new(gui_interpreter(program.as_ref()));
    hide_console_tokio(&mut command);
    command
}

/// Windows 11 hands console launches to Windows Terminal, and that hand-off
/// still opens a visible window even for `CREATE_NO_WINDOW`/`DETACHED_PROCESS`
/// children. The GUI-subsystem sibling (`pythonw.exe`) has no console at all,
/// which removes the window entirely; redirected stdout/stderr keep working.
fn gui_interpreter(program: &std::ffi::OsStr) -> std::ffi::OsString {
    let path = std::path::Path::new(program);
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    if file_name != "python.exe" {
        return program.to_os_string();
    }
    let sibling = path.with_file_name("pythonw.exe");
    if sibling.is_file() {
        return sibling.into_os_string();
    }
    program.to_os_string()
}

#[cfg(windows)]
fn hide_console(command: &mut std::process::Command) {
    std::os::windows::process::CommandExt::creation_flags(command, CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn hide_console(_command: &mut std::process::Command) {}

#[cfg(windows)]
fn hide_console_tokio(command: &mut tokio::process::Command) {
    // tokio exposes the same Windows flag through an inherent method.
    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn hide_console_tokio(_command: &mut tokio::process::Command) {}

#[cfg(test)]
mod tests {
    use super::gui_interpreter;

    #[test]
    fn a_quiet_command_still_runs_and_reports_output() {
        #[cfg(windows)]
        let mut command = super::quiet("cmd");
        #[cfg(windows)]
        command.args(["/C", "echo hi"]);

        #[cfg(not(windows))]
        let mut command = super::quiet("true");

        let output = command.output().expect("child should start");
        assert!(output.status.success());
    }

    #[test]
    fn non_python_programs_are_left_untouched() {
        let program = std::ffi::OsString::from("ffmpeg.exe");
        assert_eq!(gui_interpreter(&program), program);
    }

    #[cfg(windows)]
    #[test]
    fn python_is_swapped_for_the_windowless_interpreter_when_present() {
        let directory = tempfile::tempdir().unwrap();
        let python = directory.path().join("python.exe");
        let pythonw = directory.path().join("pythonw.exe");
        std::fs::write(&python, b"stub").unwrap();
        std::fs::write(&pythonw, b"stub").unwrap();

        let resolved = gui_interpreter(python.as_os_str());
        assert_eq!(std::path::Path::new(&resolved), pythonw);
    }
}
