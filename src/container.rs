use crate::{browser::BrowserContext, error::Result};
use std::process::Command;

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
            let cmd = format!(
                "sudo apt-get update && sudo apt-get install -y {}",
                self.config.container_packages.join(" ")
            );
            self.run_in_container(cmd)?;
        }

        if let Some(bootstrap) = self.config.container_bootstrap.clone() {
            println!("Running bootstrap: {}", bootstrap);
            self.run_in_container(format!(
                "cd {} && {}",
                self.main_worktree().display(),
                bootstrap
            ))?;
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

    fn run_in_container(&self, cmd: String) -> Result<()> {
        self.container_enter(vec!["sh".into(), "-c".into(), cmd])
    }
}
