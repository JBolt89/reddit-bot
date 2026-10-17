use crate::is_pickle_message;

#[test]
fn pickle_is_pickle() {
    assert!(is_pickle_message("pickle"));
}

#[test]
fn message_containing_pickle_is_pickle() {
    assert!(is_pickle_message("asdasdpickleasdasd"));
}

#[test]
fn p1ckle_is_pickle() {
    assert!(is_pickle_message("p1ckle"));
}

#[test]
fn spaced_pickle_is_pickle() {
    assert!(is_pickle_message("p i c k l e"));
}

#[test]
fn dotted_pickle_is_pickle() {
    assert!(is_pickle_message("p.i.c.k.l.e"));
}

#[test]
fn pickle_emoji_is_pickle() {
    assert!(is_pickle_message("asdfasd 🥒 pickldsfe"));
}

#[test]
fn piiccckkkkllllleeeeee_is_pickle() {
    assert!(is_pickle_message("piiccckkkkllllleeeeee"));
}

#[test]
fn elkcip_isnt_pickle() {
    assert!(!is_pickle_message("elkcip"));
}

#[test]
fn lettered_pickle_isnt_pickle() {
    assert!(!is_pickle_message("paiacakalae"));
}
