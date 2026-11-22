//! Spawning console programs from a GUI application.
//!
//! Windows allocates a console for every console subsystem child started from a
//! process that has none, and Windows 11 hands that console to Windows Terminal,
//! which shows up as a black window. Giving every child a *hidden* console
//! (`CREATE_NO_WINDOW`) keeps the window away and — importantly — lets the
//! grandchildren (sox, ffmpeg) inherit it instead of allocating their own.
//!
//! `DETACHED_PROCESS` must not be used here: a detached parent has no console,
//! so each console grandchild would open a fresh visible one.

/// `CREATE_NO_WINDOW`: a console exists but is never shown.
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn quiet(program: impl AsRef<std::ffi::OsStr>) -> std::process::Command {
    let mut command = std::process::Command::new(program);
    hide_console(&mut command);
    command
}

pub fn quiet_tokio(program: impl AsRef<std::ffi::OsStr>) -> tokio::process::Command {
    let mut command = tokio::process::Command::new(program);
    hide_console_tokio(&mut command);
    command
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
    fn redirected_output_survives_the_hidden_console() {
        #[cfg(windows)]
        let mut command = super::quiet("cmd");
        #[cfg(windows)]
        command.args(["/C", "echo hidden"]);

        #[cfg(not(windows))]
        let mut command = super::quiet("echo");

        #[cfg(not(windows))]
        command.arg("hidden");

        let output = command.output().expect("child should start");
        let text = String::from_utf8_lossy(&output.stdout);
        assert!(text.contains("hidden"), "stdout was {text:?}");
    }
}
