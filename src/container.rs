use crate::{browser::BrowserContext, error::Result};
use std::{path::Path, process::Command};

impl BrowserContext {
    fn container_name(&self) -> String {
        format!("{}-dev", self.browser)
    }

    pub fn container_setup(&self) -> Result<()> {
        let name = self.container_name();
        let home = self.root.join("home");
        std::fs::create_dir_all(&home)?;

        println!("Creating container: {}", name);
        println!("Container home: {:?}", home);

        if self.config.use_wkdev {
            Command::new("wkdev-create")
                .args(["--name", &name, "--home", &home.to_string_lossy()])
                .status()?;
        } else {
            Command::new("distrobox")
                .args([
                    "create",
                    "--name",
                    &name,
                    "--image",
                    "debian:latest",
                    "--home",
                    &home.to_string_lossy(),
                    "--additional-flags",
                    &format!(
                        "--userns=keep-id -v {}:{}:rw",
                        self.root.display(),
                        self.root.display()
                    ),
                ])
                .status()?;
        }

        self.container_finish()
    }

    pub fn container_finish(&self) -> Result<()> {
        if !self.config.container_packages.is_empty() {
            println!(
                "Installing packages: {}",
                self.config.container_packages.join(", ")
            );
            self.run("sudo apt-get update")?;
            self.run(&format!(
                "sudo apt-get install {}",
                self.config.container_packages.join(" ")
            ))?;
        }

        if let Some(bootstrap) = self.config.commands.get("bootstrap") {
            println!("Running bootstrap: {}", bootstrap);
            self.run_in_path(bootstrap, &self.main_worktree())?;
        }

        Ok(())
    }

    pub fn container_remove(&self) -> Result<()> {
        Command::new("podman")
            .args(["container", "rm", "--force"])
            .arg(self.container_name())
            .status()?;
        Ok(())
    }

    pub fn run(&self, command: &str) -> Result<()> {
        let mut cmd = if self.config.use_wkdev {
            Command::new("wkdev-enter")
        } else {
            Command::new("distrobox")
        };

        cmd.arg("enter").arg(&self.container_name());

        if !self.config.env.is_empty() {
            let env_args = self
                .config
                .env
                .iter()
                .map(|var| format!("--env {}", var))
                .collect::<Vec<_>>()
                .join("  ");
            cmd.arg("--additional-flags").arg(env_args);
        }

        if !command.is_empty() {
            cmd.args(["--", "sh", "-c"]).arg(command);
        }

        cmd.status()?;
        Ok(())
    }

    pub fn run_in_path(&self, command: &str, cwd: &Path) -> Result<()> {
        self.run(&format!("cd {} && {}", cwd.display(), command))
    }
}
