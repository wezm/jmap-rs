use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use parse::Presence::*;
use record;
use record::{Record, PartialRecord};


make_prop_enum_type!(SendingRestriction, "SendingRestriction", Everybody,
    Everybody   => "everybody",
    Internal => "internal", // Only FastMail users
    Nobody => "nobody"
);

make_record_type!(Alias, PartialAlias, "Alias",
    email: String => "email",
    target_emails: Vec<String> => "targetEmails",
    target_group_id: Option<String> => "targetGroupId",
    restrict_sending_to: SendingRestriction => "restrictSendingTo",
    is_srs_enabled: bool => "isSRSEnabled"
);

impl Alias {
    pub fn new<S: Into<String>>(email: S) -> Alias {
        let mut alias = Alias::default();
        alias.email = email.into();
        alias
    }

    pub fn add_target<S: Into<String>>(&mut self, target: S) {
        self.target_emails.push(target.into());
    }
}

#[test]
fn test_creation() {
    let alias = Alias::new("test@example.com");
    assert_eq!("test@example.com", alias.email);
}

#[test]
fn test_adding_targets() {
    let mut alias = Alias::new("test@example.com");
    alias.add_target("other@example.com");
    alias.add_target("another@example.com");

    assert_eq!(vec!["other@example.com", "another@example.com"], alias.target_emails);
}
