use tracing::subscriber::with_default;
use tracing_attributes::instrument;
use tracing_mock::*;

#[instrument]
fn default_name() {}

#[instrument(name = "my_name")]
fn custom_name() {}

// XXX: it's weird that we support both of these forms, but apparently we
// managed to release a version that accepts both syntax, so now we have to
// support it! yay!
#[instrument("my_other_name")]
fn custom_name_no_equals() {}

const ANOTHER_NAME: &str = "one_more_name";

#[instrument(name = ANOTHER_NAME)]
fn custom_name_expr() {}

#[test]
fn default_name_test() {
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("default_name"))
        .enter(span::mock().named("default_name"))
        .exit(span::mock().named("default_name"))
        .done()
        .run_with_handle();

    with_default(subscriber, || {
        default_name();
    });

    handle.assert_finished();
}

#[test]
fn custom_name_test() {
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("my_name"))
        .enter(span::mock().named("my_name"))
        .exit(span::mock().named("my_name"))
        .done()
        .run_with_handle();

    with_default(subscriber, || {
        custom_name();
    });

    handle.assert_finished();
}

#[test]
fn custom_name_no_equals_test() {
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("my_other_name"))
        .enter(span::mock().named("my_other_name"))
        .exit(span::mock().named("my_other_name"))
        .done()
        .run_with_handle();

    with_default(subscriber, || {
        custom_name_no_equals();
    });

    handle.assert_finished();
}

#[test]
fn custom_name_expr_test() {
    let (subscriber, handle) = subscriber::mock()
        .new_span(span::mock().named("one_more_name"))
        .enter(span::mock().named("one_more_name"))
        .exit(span::mock().named("one_more_name"))
        .done()
        .run_with_handle();

    with_default(subscriber, || {
        custom_name_expr();
    });

    handle.assert_finished();
}
