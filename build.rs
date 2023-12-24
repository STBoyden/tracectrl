use std::{env, process::Command};

#[cfg(not(windows))]
const KNOWN_PACKAGE_MANAGERS: [&str; 4] = ["bun", "pnpm", "yarn", "npm"];

#[cfg(windows)]
const KNOWN_PACKAGE_MANAGERS: [&str; 5] =
	["bun.exe", "pnpm.exe", "yarn.exe", "npm.exe", "npm.cmd"];

fn find_pkg_manager(command_name: &str) -> bool {
	env::var_os("PATH")
		.and_then(|bin_dirs| {
			env::split_paths(&bin_dirs).find_map(|dir| {
				let full_path = dir.join(command_name);

				if full_path.is_file() {
					Some(full_path)
				} else {
					None
				}
			})
		})
		.is_some()
}

fn main() {
	println!("cargo:rerun-if-changed=tc-frontend/package.json");
	println!("cargo:rerun-if-changed=tc-frontend/tsconfig.json");
	println!("cargo:rerun-if-changed=tc-frontend/svelte.config.js");
	println!("cargo:rerun-if-changed=tc-frontend/vite.config.js");
	println!("cargo:rerun-if-changed=migrations");

	let Some(package_manager) = KNOWN_PACKAGE_MANAGERS
		.iter()
		.find(|pkg| find_pkg_manager(pkg))
	else {
		panic!(
			"Could not find a suitable node package manager. Tried {KNOWN_PACKAGE_MANAGERS:#?}"
		)
	};

	println!("cargo:warning=Using the '{package_manager}' package manager for node");

	println!("cargo:warning=Running '{package_manager} install'...");
	match Command::new(package_manager)
		.arg("install")
		.current_dir("./tc-frontend")
		.spawn()
		.unwrap_or_else(|_| {
			panic!("Could not run the selected package manager: {package_manager}")
		})
		.wait()
	{
		Ok(status) if status.success() => (),
		Ok(status) => panic!(
			"Could not succeed in installing node packages, got: {}",
			status.code().expect("Could not get exit status of command")
		),
		Err(error) => panic!("Could not succeed in installing node packages: {error}"),
	}

	println!("cargo:rustc-env=TC_PACKAGE_MANAGER={package_manager}");
	println!("cargo:rustc-env=TC_FRONTEND_DIR=./tc-frontend");

	if !cfg!(debug_assertions) {
		match Command::new(package_manager)
			.args(["run", "build"])
			.current_dir("./tc-frontend")
			.spawn()
			.unwrap_or_else(|_| panic!("Could not run '{package_manager} run build'"))
			.wait()
		{
			Ok(status) if status.success() => (),
			Ok(status) => panic!(
				"Could not succeed in building production application, got: {}",
				status.code().expect("Could not get exit status of command")
			),
			Err(error) => {
				panic!("Could not succeed in building production application, got: {error}")
			}
		}
	}
}
