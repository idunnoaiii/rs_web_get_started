cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration generate {{migration name}}
diesel migration run
diesel migration list