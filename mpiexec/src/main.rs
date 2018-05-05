#[macro_use]
extern crate clap;
extern crate empire;

use clap::{App, AppSettings, Arg};
use empire::{comm, Comm};

fn checks_usize(v: String) -> Result<(), String> {
    match v.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("This value must be a positive integer")),
    }
}

fn app_from_crate<'a, 'b>() -> App<'a, 'b>
where
    'a: 'b,
{
    app_from_crate!()
}

fn app<'a, 'b>() -> App<'a, 'b>
where
    'a: 'b,
{
    app_from_crate()
        .setting(AppSettings::NoBinaryName)
        .arg(
            Arg::with_name("maxprocs")
                .short("n")
                .takes_value(true)
                .validator(checks_usize)
                .help("The maximum number of processes that mpiexec will spawn"),
        )
        .arg(
            Arg::with_name("command")
                .help("The command to be executed")
                .required(true),
        )
        .arg(
            Arg::with_name("command_args")
                .help("The arguments to be passed to 'command'")
                .last(true),
        )
}

fn main() {
    let universe = empire::Universe::new();

    let args: Vec<_> = std::env::args_os().skip(1).collect();

    let colon = std::ffi::OsStr::new(":");
    let commands = args.split(|arg| arg == colon);

    let commands = commands
        .map(|args| app().get_matches_from(args.into_iter()))
        .map(|matches| {
            let mut spawn_info = comm::SpawnCommandInfo::new(
                matches.value_of_os("command").unwrap().to_owned(),
                matches
                    .values_of_os("command_args")
                    .into_iter()
                    .flat_map(|args| args.into_iter()),
            );

            if let Some(maxprocs) = matches.value_of("maxprocs") {
                spawn_info.max_procs(maxprocs.parse::<usize>().unwrap());
            }

            spawn_info
        });

    let locked = universe.write().unwrap();

    let intercomm = locked.comm_self().spawn_multiple_root(commands, 0);

    // let command =
    //     match env::args().take(1).next() {
    //         Some(command) => command,
    //         None => {

    //         }
    //     };
}
