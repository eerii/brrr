use crate::{browser::BrowserContext, error::Result};
use std::{path::Path, process::Command};

impl BrowserContext {
    fn container_name(&self) -> String {
        format!("{}-dev", self.browser.name())
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

        if let Some(bootstrap) = self.config.container_bootstrap.clone() {
            println!("Running bootstrap: {}", bootstrap);
            self.run_in_path(&bootstrap, &self.main_worktree())?;
        }

        Ok(())
    }

    pub fn container_enter(&self, args: Vec<String>) -> Result<()> {
        let mut cmd = if self.config.use_wkdev {
            Command::new("wkdev-enter")
        } else {
            Command::new("distrobox")
        };

        cmd.arg("enter").arg(&self.container_name());

        if !args.is_empty() {
            cmd.arg("--").args(args);
        }

        cmd.status()?;
        Ok(())
    }

    pub fn run(&self, cmd: &str) -> Result<()> {
        self.container_enter(shell_words::split(cmd)?)
    }

    pub fn run_in_path(&self, cmd: &str, cwd: &Path) -> Result<()> {
        self.container_enter(vec![
            "sh".into(),
            "-c".into(),
            format!("cd {} && {}", cwd.display(), cmd),
        ])
    }
}
