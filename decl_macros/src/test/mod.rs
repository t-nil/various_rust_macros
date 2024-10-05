pub mod insta {
    use std::path::PathBuf;

    /// https://insta.rs/docs/cmd/#filtering
    ///
    /// Filters common, changing expressions from test snapshots, like tempfiles.
    #[macro_export]
    macro_rules! apply_common_filters {
        {} => {
            let mut settings = insta::Settings::clone_current();
            // Macos Temp Folder
            settings.add_filter(r"/var/folders/\S+?/T/\S+(/.*)", "[TEMP_FILE]$1");
            settings.add_filter(r"/var/folders/\S+?/T/\S+", "[TEMP_FILE]");
            // Linux Temp Folder
            settings.add_filter(r"/tmp/\.tmp\S+(/.*)", "[TEMP_FILE]$1");
            settings.add_filter(r"/tmp/\.tmp\S+", "[TEMP_FILE]");
            // Windows Temp folder
            settings.add_filter(r"\b[A-Z]:\\.*\\Local\\Temp\\\S+", "[TEMP_FILE]");
            // Convert windows paths to Unix Paths.
            settings.add_filter(r"\\\\?([\w\d.])", "/$1");
            let _bound = settings.bind_to_scope();
        }
    }

    // Creating non-utf8 path is an OS-specific pain, so let's run this only on
    // linux, where arbitrary bytes work.
    //
    // (stolen from https://github.com/rust-lang/cargo/pull/9226/files#diff-9977238c61100eb9f319febd88e2434b304ac401f0da3b50d00d7c91de319e2fR2957-R2966)
    pub fn generate_non_utf8_path<T>() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            use std::ffi::OsString;
            use std::os::unix::ffi::OsStringExt;
            use std::path::PathBuf;

            let wrong = PathBuf::from(OsString::from_vec(vec![255]));

            wrong
        }
    }
}
