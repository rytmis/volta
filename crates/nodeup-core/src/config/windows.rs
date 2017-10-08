use std::path::{Path, PathBuf};

use winfolder;

// C:\
//     ProgramData\
//         Nodeup\
//             cache\                                  cache_dir
//                 node\                               node_cache_dir
//                     node-v4.8.4-win64.tar.gz
//                     node-v6.11.3-win64.tar.gz
//                     node-v8.6.0-win64.tar.gz
//                     ...
//             versions\                               versions_dir
//                 node\                               node_versions_dir
//                     4.8.4\                          node_version_dir("4.8.4")
//                     6.11.3\
//                     8.6.0\
//                     ...
//             binstub.exe                             binstub_file

fn program_data_root() -> Option<PathBuf> {
    Some(Path::new(&winfolder::known_path(&winfolder::id::PROGRAM_DATA))
        .join("Nodeup"))
}

pub fn cache_dir() -> Option<PathBuf> {
    program_data_root().map(|root| {
        root.join("cache")
    })
}

pub fn node_cache_dir() -> Option<PathBuf> {
    cache_dir().map(|cache| {
        cache.join("node")
    })
}

pub fn versions_dir() -> Option<PathBuf> {
    program_data_root().map(|root| {
        root.join("versions")
    })
}

pub fn node_versions_dir() -> Option<PathBuf> {
    versions_dir().map(|versions| {
        versions.join("node")
    })
}

pub fn node_version_dir(version: &str) -> Option<PathBuf> {
    node_versions_dir().map(|node| {
        node.join(version)
    })
}

pub fn binstub_file() -> Option<PathBuf> {
    program_data_root().map(|root| {
        root.join("binstub.exe")
    })
}

// C:\
//     Program Files\
//         Nodeup\                                     bin_dir
//             nodeup.exe                              nodeup_file
//             toolchain\                              toolchain_dir
//                 node.exe                            toolchain_file("node")
//                 npm.exe
//                 npx.exe
//                 ...

fn program_files_root() -> Option<PathBuf> {
    Some(Path::new(&winfolder::known_path(&winfolder::id::PROGRAM_FILES_X64))
        .join("Nodeup"))
}

pub fn bin_dir() -> Option<PathBuf> {
    program_files_root()
}

pub fn nodeup_file() -> Option<PathBuf> {
    bin_dir().map(|bin| {
        bin.join("nodeup.exe")
    })
}

pub fn toolchain_dir() -> Option<PathBuf> {
    program_files_root().map(|root| {
        root.join("toolchain")
    })
}

pub fn toolchain_file(toolname: &str) -> Option<PathBuf> {
    toolchain_dir().map(|toolchain| {
        toolchain.join(&format!("{}.exe", toolname))
    })
}

// C:\
//     Users\
//         dherman\
//             AppData\
//                 Local\
//                     Nodeup\
//                         config.toml                 user_config_file

fn local_data_root() -> Option<PathBuf> {
    Some(Path::new(&winfolder::known_path(&winfolder::id::LOCAL_APP_DATA))
        .join("Nodeup"))
}

pub fn user_config_file() -> Option<PathBuf> {
    local_data_root().map(|root| {
        root.join("config.toml")
    })
}