use cucumber_rust::{Steps};
use exchange_bdd_scenarios::{
    default_string, interesting_appendage,
};

use crate::MyWorld;

pub fn steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.given("a string with some particular value", |_world, _ctx| {
        MyWorld::SomeString(default_string())
    });

    steps.when(
        "I append a known suffix to the value",
        |world, _ctx| match world {
            MyWorld::SomeString(x) => MyWorld::SuffixedString(interesting_appendage(&x)),
            _ => panic!("Invalid world state"),
        },
    );

    steps.then_regex(r#"^that string is now equal to "(.*)"$"#, |world, ctx| {
        match world {
            MyWorld::SuffixedString(x) => assert_eq!(x, ctx.matches[1]),
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps
}
