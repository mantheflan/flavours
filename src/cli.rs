use clap::{App, Arg, AppSettings, crate_version, crate_authors};

pub fn build_cli() -> App<'static> {
    App::new("flavours")
        .about("A simple way to manage and use base16 standard schemes and templates")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::InferSubcommands)
        .arg(
            Arg::with_name("verbose")
            .about("Be more verbose")
            .long("verbose")
            .short('v')
        )
        .subcommand(
            App::new("apply")
                .about("Applies scheme, according to configuration")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .arg(
                    Arg::with_name("pattern")
                        .about("Scheme to apply, supports glob. If not specified, defaults to last applied scheme. If matches more than one scheme (when using wildcards, for example), one among them is chosen randomly."),
                )
        )
        .subcommand(
            App::new("query")
                .about("Queries installed scheme(s)")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::TrailingVarArg)
                .arg(
                    Arg::with_name("pattern")
                        .about("Scheme to query, supports glob. If not specified, defaults to last applied scheme. Supports displaying more than one scheme (when using wildcards, for example).")
                        .multiple(true)
                        .multiple_values(true)
                )
        )
        .subcommand(
            App::new("update")
                .about("Downloads schemes, templates, or updates their lists")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .arg(
                    Arg::with_name("operation")
                        .value_name("operation")
                        .about("Update sources lists from repositories or (re)download schemes/templates specified in the lists. Default repositories for lists, and the lists themselves, can be manually changed.")
                        .required(true)
                        .possible_values(&["lists", "schemes", "templates", "all"])
                )
        )
        .subcommand(
            App::new("completions")
                .about("Generates completion for given shell, outputs to stdout")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .arg(
                    Arg::with_name("shell")
                        .value_name("shell")
                        .about("Specify which shell to generate for")
                        .required(true)
                        .possible_values(&["bash", "elvish", "fish", "powershell", "zsh"])
                ) 
        )
}

