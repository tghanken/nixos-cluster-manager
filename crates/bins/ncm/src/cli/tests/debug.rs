use crate::cli::{ObjectAction, SecretAction};

#[test]
fn test_object_action_debug() {
    let create = ObjectAction::Create { name: "test-node".to_string() };
    assert_eq!(format!("{:?}", create), "Create(test-node)");

    let list = ObjectAction::List;
    assert_eq!(format!("{:?}", list), "List");

    let remove = ObjectAction::Remove { name: "test-node".to_string() };
    assert_eq!(format!("{:?}", remove), "Remove(test-node)");
}

#[test]
fn test_secret_action_debug() {
    let create = SecretAction::Create { name: "test-secret".to_string() };
    assert_eq!(format!("{:?}", create), "Create(test-secret)");

    let list = SecretAction::List;
    assert_eq!(format!("{:?}", list), "List");

    let remove = SecretAction::Remove { name: "test-secret".to_string() };
    assert_eq!(format!("{:?}", remove), "Remove(test-secret)");

    let rekey = SecretAction::Rekey;
    assert_eq!(format!("{:?}", rekey), "Rekey");
}
