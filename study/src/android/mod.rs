mod shell;

use crate::android::shell::Shell;
use cargo_metadata::semver::Version;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::{env, fs};

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/26
///
/// Return a path to a discovered NDK and string describing how it was found
fn derive_ndk_path(shell: &mut Shell) -> Option<(PathBuf, String)> {
    let ndk_vars = [
        "ANDROID_NDK_HOME",
        "ANDROID_NDK_ROOT",
        "ANDROID_NDK_PATH",
        "NDK_HOME",
    ];
    if let Some((var_name, path)) = find_first_consistent_var_set(&ndk_vars, shell) {
        let path = PathBuf::from(path);
        return highest_version_ndk_in_path(&path)
            .or(Some(path))
            .map(|path| (path, var_name.to_string()));
    }

    let sdk_vars = ["ANDROID_HOME", "ANDROID_SDK_ROOT", "ANDROID_SDK_HOME"];
    if let Some((var_name, sdk_path)) = find_first_consistent_var_set(&sdk_vars, shell) {
        let ndk_path = PathBuf::from(&sdk_path).join("ndk");
        if let Some(v) = highest_version_ndk_in_path(&ndk_path) {
            return Some((v, var_name.to_string()));
        }
    }

    let ndk_dir = default_ndk_dir();
    highest_version_ndk_in_path(&ndk_dir).map(|path| (path, "standard location".to_string()))
}

/// Return the name and value of the first environment variable that is set
///
/// Additionally checks that if any other variables are set then they should
/// be consistent with the first variable, otherwise a warning is printed.
fn find_first_consistent_var_set<'a>(
    vars: &'a [&str],
    shell: &mut Shell,
) -> Option<(&'a str, OsString)> {
    let mut first_var_set = None;
    for var in vars {
        if let Some(path) = env::var_os(var) {
            if let Some((first_var, first_path)) = first_var_set.as_ref() {
                if *first_path != path {
                    shell
                        .warn(format!(
                            "Environment variable `{} = {:#?}` doesn't match `{} = {:#?}`",
                            first_var, first_path, var, path
                        ))
                        .unwrap();
                }
                continue;
            }
            first_var_set = Some((*var, path));
        }
    }

    first_var_set
}

fn highest_version_ndk_in_path(ndk_dir: &Path) -> Option<PathBuf> {
    if ndk_dir.exists() {
        fs::read_dir(ndk_dir)
            .ok()?
            .filter_map(Result::ok)
            .filter_map(|x| {
                let path = x.path();
                path.components()
                    .last()
                    .and_then(|comp| comp.as_os_str().to_str())
                    .and_then(|name| Version::parse(name).ok())
                    .map(|version| (version, path))
            })
            .max_by(|(a, _), (b, _)| a.cmp(b))
            .map(|(_, path)| path)
    } else {
        None
    }
}

fn default_ndk_dir() -> PathBuf {
    #[cfg(windows)]
    let dir = pathos::user::local_dir()
        .unwrap()
        .to_path_buf()
        .join("Android")
        .join("sdk")
        .join("ndk");

    #[cfg(target_os = "linux")]
    let dir = pathos::xdg::home_dir()
        .unwrap()
        .join("Android")
        .join("Sdk")
        .join("ndk");

    #[cfg(target_os = "macos")]
    let dir = pathos::user::home_dir()
        .unwrap()
        .join("Library")
        .join("Android")
        .join("sdk")
        .join("ndk");

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let dir = PathBuf::new();

    dir
}

#[cfg(test)]
mod tests {
    use crate::ptl;
    use std::env;

    /// /Users/angcyo/Library/Android/sdk/ndk
    /// [src/android/mod.rs:138:9]->ndk_home->"/Users/angcyo/Library/Android/sdk/ndk/29.0.13113456"
    /// [src/android/mod.rs:139:9]->_ndk_detection_method->"standard location"
    #[test]
    fn test_derive_ndk_path() {
        ptl!("env::var_os->{:?}", env::var_os("PATH"));

        let mut shell = crate::android::Shell::new();
        //shell.set_verbosity(verbosity);
        //shell.set_color_choice(color)?;
        let (ndk_home, _ndk_detection_method) = match crate::android::derive_ndk_path(&mut shell) {
            Some((path, method)) => (path, method),
            None => {
                shell.error("Could not find any NDK.").unwrap();
                shell.note(
                    "Set the environment ANDROID_NDK_HOME to your NDK installation's root directory,\nor install the NDK using Android Studio."
                ).unwrap();
                std::process::exit(1);
            }
        };
        ptl!("ndk_home->{:?}", ndk_home);
        ptl!("_ndk_detection_method->{:?}", _ndk_detection_method);
    }
}
