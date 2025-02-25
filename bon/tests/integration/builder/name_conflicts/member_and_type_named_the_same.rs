//! @Veetaha encountered a bug when working on big changes to `bon` where
//! the generic params generated by the macro were named the same as the
//! type of the member, and thus leading to a name conflict. This test is
//! here to catch any such simple regression.
use crate::prelude::*;
struct User;

#[test]
fn member_and_type_named_the_same_fn() {
    #[builder]
    fn sut(user: User) {
        let _ = user;
    }

    sut().user(User).call();
}

#[test]
fn member_and_type_named_the_same_struct() {
    #[derive(Builder)]
    struct Sut {
        #[allow(dead_code)]
        user: User,
    }

    #[bon]
    impl Sut {
        #[builder]
        fn sut(user: User) {
            let _ = user;
        }
    }

    let _ = Sut::builder().user(User).build();
    Sut::sut().user(User).call();
}
