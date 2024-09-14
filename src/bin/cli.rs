extern crate c8s;
use c8s::commands::users::{create_user, delete_user, list_users};
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("c8s")
        .about("C8s commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new user")
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(
                            Arg::new("roles")
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
                )
                .subcommand(Command::new("list").about("List all users"))
                .subcommand(
                    Command::new("delete")
                        .about("Delete user by Id")
                        .arg_required_else_help(true)
                        .arg(Arg::new("id").required(true)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", create_matches)) => {
                let roles_iter = create_matches.get_many::<String>("roles");

                let roles: Vec<String> = match roles_iter {
                    Some(values) => values.map(|s| s.to_string()).collect(),
                    None => Vec::new(),
                };

                create_user(
                    create_matches
                        .get_one::<String>("username")
                        .unwrap()
                        .to_owned(),
                    create_matches
                        .get_one::<String>("password")
                        .unwrap()
                        .to_owned(),
                    roles,
                );
            }

            Some(("list", _)) => {
                list_users();
            }

            Some(("delete", delete_matches)) => {
                delete_user(delete_matches.get_one::<&i32>("id").unwrap());
            }

            _ => {}
        },

        _ => {}
    }
}
