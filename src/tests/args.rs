use args;

#[test]
fn error_if_no_subcommand() {
    let out = args::get_matches_for(vec!("mdp".into()));
    assert!(out.is_err());
}
