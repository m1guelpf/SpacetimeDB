use anyhow;
use clap::error::ContextKind;
use clap::error::ContextValue;
use clap::ArgMatches;
use clap::Command;
use std::process::exit;
use std::vec;

mod call;
mod energy;
mod identity;
mod init;
mod login;
mod logs;
mod metrics;
mod query;
mod revert;
mod signup;
mod update;
mod rm;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = match get_command().try_get_matches() {
        Ok(args) => args,
        Err(e) => {
            if e.kind() == clap::ErrorKind::UnrecognizedSubcommand {
                let cmd = e
                    .context()
                    .find_map(|c| match c {
                        (ContextKind::InvalidSubcommand, &ContextValue::String(ref cmd)) => Some(cmd),
                        _ => None,
                    })
                    .expect("UnrecognizedSubcommand implies the presence of InvalidSubcommand");

                println!("invalid command: {}", cmd);
                exit(0);
            } else {
                e.exit();
            }
        }
    };
    match args.subcommand() {
        Some((cmd, subcommand_args)) => exec_subcommand(cmd, subcommand_args).await?,
        None => {
            get_command().print_help().unwrap();
            exit(0);
        }
    }
    Ok(())
}

fn get_command() -> Command<'static> {
    Command::new("stdb")
        .allow_external_subcommands(true)
        .subcommands(get_subcommands())
        .override_usage("stdb [OPTIONS] [SUBCOMMAND]")
        .help_template(
            "\
Client program for SpacetimeDB

Usage: {usage}

Options:
{options}

Some common SpacetimeDB commands are
    init        Initializes a new Spacetime database
    update      Updates the Wasm module of an existing Spacetime database
    rm          Removes the Wasm module of an existing Spacetime database
    logs        Prints logs from a Spacetime database
    call        Invokes a Spacetime function
    identity    Requests a new Spacetime Identity and token
",
        )
    //signup      Creates a new SpacetimeDB identity using your email
    //login       Login using an existing identity
    //energy      Invokes commands related to energy
    //query       Run a SQL query on the database
    //revert      Reverts the database to a given point in time
    //metrics     Prints metrics
}

fn get_subcommands() -> Vec<Command<'static>> {
    vec![
        init::cli(),
        update::cli(),
        rm::cli(),
        logs::cli(),
        call::cli(),
        identity::cli(),

        // TODO
        energy::cli(),
        login::cli(),
        metrics::cli(),
        query::cli(),
        revert::cli(),
        signup::cli(),
    ]
}

async fn exec_subcommand(cmd: &str, args: &ArgMatches) -> Result<(), anyhow::Error> {
    match cmd {
        "identity" => identity::exec(args).await,
        "call" => call::exec(args).await,
        "energy" => energy::exec(args).await,
        "init" => init::exec(args).await,
        "rm" => rm::exec(args).await,
        "login" => login::exec(args).await,
        "logs" => logs::exec(args).await,
        "metrics" => metrics::exec(args).await,
        "query" => query::exec(args).await,
        "revert" => revert::exec(args).await,
        "signup" => signup::exec(args).await,
        "update" => update::exec(args).await,
        _ => Err(anyhow::anyhow!("invalid subcommand")),
    }
}
