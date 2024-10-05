pub mod insta {
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
}
