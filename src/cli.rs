use std::{
    io::{stdin, stdout, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use uk_manager::{core, mods::LookupMod, settings::Platform};
use uk_mod::{unpack::ModReader, Manifest};

xflags::xflags! {
    src "./src/cli.rs"

    /// Command line interface for U-King Mod Manager
    cmd ukmm {
        /// Verbose logging for debugging
        optional -d, --debug
        /// Run using settings in same folder as executable
        optional -p, --portable
        /// Automatically deploy after running command (redunant with `deploy` command)
        optional -D, --deploy
        /// Install a mod
        cmd install {
            /// Path to the mod to install
            required path: PathBuf
        }
        /// Uninstall a mod
        cmd uninstall {}
        /// Deploy mods
        cmd deploy {}
        /// Change current mode (Switch or Wii U)
        cmd mode {
            /// Mode to activate (Switch or Wii U)
            required platform: Platform
        }
    }
}
// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Ukmm {
    pub debug: bool,
    pub portable: bool,
    pub deploy: bool,
    pub subcommand: UkmmCmd,
}

#[derive(Debug)]
pub enum UkmmCmd {
    Install(Install),
    Uninstall(Uninstall),
    Deploy(Deploy),
    Mode(Mode),
}

#[derive(Debug)]
pub struct Install {
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Uninstall;

#[derive(Debug)]
pub struct Deploy;

#[derive(Debug)]
pub struct Mode {
    pub platform: Platform,
}

impl Ukmm {
    #[allow(dead_code)]
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end

macro_rules! input {
    () => {{
        stdout().flush()?;
        let mut _input = String::new();
        stdin().read_line(&mut _input).unwrap();
        _input
    }};
}

#[derive(Debug)]
pub struct Runner {
    core: core::Manager,
    cli:  Ukmm,
}

impl Runner {
    pub fn new(cli: Ukmm) -> Self {
        Self {
            core: core::Manager::init().unwrap(),
            cli,
        }
    }

    fn check_mod(&self, path: &Path) -> Result<Option<PathBuf>> {
        println!("Opening mod at {}...", path.display());
        let (mod_, path) = match ModReader::open(path, vec![]) {
            Ok(mod_) => (mod_, path.to_path_buf()),
            Err(e) => {
                match uk_manager::mods::convert_gfx(&self.core, path, None) {
                    Ok(path) => {
                        log::info!("Opening mod at {}", path.display());
                        (
                            ModReader::open(&path, vec![])
                                .context("Failed to open converted mod")?,
                            path,
                        )
                    }
                    Err(e2) => {
                        anyhow::bail!(
                            "Could not open mod. Error when attempting to open as UKMM mod: {}. \
                             Error when attempting to open as legacy mod: {}.",
                            e,
                            e2
                        )
                    }
                }
            }
        };
        if !mod_.meta.options.is_empty() {
            // anyhow::bail!(
            //     "This mod contains configuration options and should be installed via the GUI."
            // );
        }
        println!(
            "Identified mod: {} (v{}) by {}",
            &mod_.meta.name, &mod_.meta.version, &mod_.meta.author
        );
        print!("Do you want to continue installing? [Y/n] ");
        let cont = !input!().to_lowercase().starts_with('n');
        if cont {
            println!("Installing {}...", mod_.meta.name);
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }

    fn deploy(&self) -> Result<()> {
        let deployer = self.core.deploy_manager();
        if deployer.pending() {
            println!("Deploying changes...");
            deployer.deploy()?;
            println!("Deployment complete");
        } else {
            println!("No changes pending deployment");
        };
        Ok(())
    }

    pub fn run(self) -> Result<()> {
        match &self.cli.subcommand {
            UkmmCmd::Mode(Mode { platform }) => {
                self.core
                    .settings_mut()
                    .apply(|s| s.current_mode = *platform)?;
                self.core.reload()?;
                println!("Mode changed to {:?}", platform);
                if self.cli.deploy {
                    self.deploy()?;
                }
                println!("Done!");
            }
            UkmmCmd::Install(Install { path }) => {
                if let Some(path) = self.check_mod(path)? {
                    let mods = self.core.mod_manager();
                    let mod_ = mods.add(&path)?;
                    mods.set_enabled(mod_.as_hash_id(), true)?;
                    mods.save()?;
                    println!("Applying mod to load order...");
                    let deployer = self.core.deploy_manager();
                    deployer.apply(Some(mod_.manifest()?.as_ref().clone()))?;
                    if self.cli.deploy {
                        self.deploy()?;
                    }
                    println!("Done!");
                }
            }
            UkmmCmd::Uninstall(_) => {
                println!("Installed mods:");
                let mod_manager = self.core.mod_manager();
                let mods = mod_manager.mods().collect::<Vec<_>>();
                for (i, mod_) in mods.iter().enumerate() {
                    println!(
                        "{}. {} (v{}) by {}",
                        i + 1,
                        &mod_.meta.name,
                        &mod_.meta.version,
                        &mod_.meta.author
                    );
                }
                print!("Enter mod(s) to uninstall, separated by commas: ");
                let mut manifests = Manifest::default();
                for id in input!().replace(' ', "").split(',') {
                    let mod_ = mods
                        .get(id.trim().parse::<usize>().context("Invalid mod number")? - 1)
                        .with_context(|| format!("Mod {} does not exist", id))?;
                    println!("Removing mod {}...", &mod_.meta.name);
                    mod_manager.del(mod_)?;
                    mod_manager.save()?;
                    manifests.extend(mod_.manifest()?.as_ref());
                }
                println!("Applying changes to merge...");
                self.core.deploy_manager().apply(Some(manifests))?;
                if self.cli.deploy {
                    self.deploy()?;
                }
                println!("Done!");
            }
            UkmmCmd::Deploy(_) => self.deploy()?,
        };
        Ok(())
    }
}
