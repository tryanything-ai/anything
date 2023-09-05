use anything_events::{Observable, Response};

#[test]
fn test_multiple_mutations() {
    use anything_events::Observable;

    let mut name = Observable::new(String::from("Ari"));
    name.mutate(|n| n.push_str(" "));
    name.mutate(|n| n.push_str("Rock"));
    name.mutate(|n| n.push_str("Star"));

    assert_eq!(name.as_str(), "Ari RockStar");
}

#[test]
fn test_mutate_mutates_variable() {
    let mut var = Observable::new(10);
    var.subscribe(|_new_var| Response::Continue);
    var.mutate(|v| {
        *v += 10;
    });
    assert_eq!(*var, 20);
}

#[test]
fn test_broadcasts_new_val() {
    let mut seen = 0;
    {
        let mut o = Observable::new(0);
        o.subscribe(|new_val| {
            seen = *new_val;
            Response::Continue
        });
        o.mutate(|val| *val = 32);
    }
    assert_eq!(seen, 32);
}

#[test]
fn test_unsubscribe_no_notification() {
    let mut called = 0;
    {
        let mut o = Observable::new(0);
        let s = o.subscribe(|_| {
            called += 1;
            Response::Continue
        });
        o.mutate(|val| *val += 1);
        o.unsubscribe(s);
        o.mutate(|val| *val += 1);
    }
    assert_eq!(called, 1);
}
