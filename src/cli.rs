use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_app() -> App<'static, 'static> {
    App::new("mdbook-blog")
        .about("mdbook blog extension")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}
