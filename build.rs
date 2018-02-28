extern crate cc;
extern crate pkg_config;

use std::env;
use std::io;
use std::path::PathBuf;
use std::process::Command;

static LIBUSB_DIR: &'static str = "libusb-1.0";
static HIDAPI_DIR: &'static str = "hidapi";

fn main() {
	clone_libusb().expect("failed to checkout libusb sources, internet connection and git are needed");
	build_libusb().expect("failed to build libusb sources");

	clone_hidapi().expect("failed to checkout hidapi sources, internet connection and git are needed");
	build_hidapi().expect("failed to build hidapi sources");

	println!("cargo:rustc-link-search=native={}", output().to_string_lossy());
}

fn output() -> PathBuf {
	PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn source(lib: &str) -> PathBuf {
	output().join(lib)
}

fn clone_libusb() -> io::Result<()> {
	if !std::path::Path::new(&source(LIBUSB_DIR)).exists() {
		Command::new("git")
			.current_dir(&output())
			.arg("clone")
			.arg("https://github.com/AccuPS/libusb")
			.arg(LIBUSB_DIR)
			.arg("--branch")
			.arg("android")
			.status()?;
	} else {
		Command::new("git")
			.current_dir(&output())
			.arg("pull")
			.arg("https://github.com/AccuPS/libusb")
			.arg("android")
			.status()?;
	}

	Ok(())
}

fn clone_hidapi() -> io::Result<()> {
	if !std::path::Path::new(&source(HIDAPI_DIR)).exists() {
		Command::new("git")
			.current_dir(&output())
			.arg("clone")
			.arg("https://github.com/AccuPS/hidapi")
			.arg(HIDAPI_DIR)
			.arg("--branch")
			.arg("android_15")
			.status()?;
	} else {
		Command::new("git")
			.current_dir(&output())
			.arg("pull")
			.arg("https://github.com/AccuPS/hidapi")
			.arg("android_15")
			.status()?;
	}

	Ok(())
}

#[cfg(target_os = "linux")]
fn build_libusb() -> io::Result<()> {
	let mut build = cc::Build::new();

	let sources = vec![
		source(LIBUSB_DIR).join("libusb/core.c"),
		source(LIBUSB_DIR).join("libusb/descriptor.c"),
		source(LIBUSB_DIR).join("libusb/hotplug.c"),
		source(LIBUSB_DIR).join("libusb/io.c"),
		source(LIBUSB_DIR).join("libusb/sync.c"),
		source(LIBUSB_DIR).join("libusb/strerror.c"),
		source(LIBUSB_DIR).join("libusb/os/linux_usbfs.c"),
		source(LIBUSB_DIR).join("libusb/os/poll_posix.c"),
		source(LIBUSB_DIR).join("libusb/os/threads_posix.c"),
		source(LIBUSB_DIR).join("libusb/os/linux_netlink.c")
	];

	build.files(sources);
	build.include(source(LIBUSB_DIR).join("android"));
	build.include(source(LIBUSB_DIR).join("libusb"));
	build.include(source(LIBUSB_DIR).join("libusb/os"));
	if cfg!(target_os = "linux") {
		build.define("_GNU_SOURCE", None);
	}
	build.shared_flag(true);

	build.compile("usb-1.0");

	Ok(())
}

#[cfg(target_os = "linux")]
fn build_hidapi() -> io::Result<()> {
	let mut build = cc::Build::new();

	build.file(source("hidapi").join("libusb/hid.c"));
	build.include(source("hidapi").join("hidapi"));
	build.include(source(LIBUSB_DIR).join("libusb"));
	build.shared_flag(true);

	build.compile("hidapi-libusb");

	Ok(())
}

//#[cfg(target_os = "macos")]
//fn build() -> io::Result<()> {
//	let mut build = cc::Build::new();
//
//	build.file(source("hidapi").join("libusb/hid.c"));
//	build.include(source("hidapi").join("hidapi"));
//	build.shared_flag(true);
//
//	for path in pkg_config::find_library("libusb-1.0").unwrap().include_paths {
//		build.include(path.to_str().unwrap());
//	}
//
//	build.compile("libhidapi.a");
//
//	Ok(())
//}
//
//#[cfg(target_os = "windows")]
//fn build() -> io::Result<()> {
//	let mut build = cc::Build::new();
//
//	build.file(source("hidapi").join("windows/hid.c"));
//	build.include(source("hidapi").join("hidapi"));
//	build.shared_flag(true);
//
//	build.compile("libhidapi.a");
//
//	Ok(())
//}
