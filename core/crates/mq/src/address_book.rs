use std::collections::{HashMap, HashSet};

use crate::{
    error::{MqError, MqResult},
    ClientId,
};

use super::topic::{DOUBLE_WILDCARD, WILDCARD};

#[derive(Debug, Default, Clone)]
pub struct AddressBook {
    children: HashMap<String, AddressBook>,
    subscribers: HashSet<ClientId>,
}

impl AddressBook {
    pub(crate) fn new() -> AddressBook {
        AddressBook {
            children: HashMap::new(),
            subscribers: HashSet::new(),
        }
    }

    pub(crate) fn subscribe(&mut self, client: ClientId, topic: &[&str]) -> MqResult<()> {
        if topic.len() == 0 {
            self.subscribers.insert(client);
            return Ok(());
        }

        self._child(topic[0]);

        if self.children.get_mut(topic[0]).is_none() {
            return Err(MqError::InternalError(
                "internal error getting address book children".to_string(),
            ));
        }

        // Safe to unwrap because of the above check
        self.children
            .get_mut(topic[0])
            .unwrap()
            .subscribe(client, &topic[1..])?;

        Ok(())
    }

    pub(crate) fn unsubscribe(&mut self, client: ClientId, topic: &[&str]) -> MqResult<()> {
        if topic.len() == 0 {
            self.subscribers.remove(&client);
            return Ok(());
        }

        if let Some(child) = self.children.get_mut(topic[0]) {
            child.unsubscribe(client, &topic[1..])?;
        }

        Ok(())
    }

    pub(crate) fn get_subscribers(&self, topic: &[&str]) -> Vec<ClientId> {
        let mut subscribers = HashSet::new();
        self._get_subscribers(topic, &mut subscribers, false);
        subscribers.drain().collect()
    }

    pub(crate) fn drop_client(&mut self, client: ClientId) -> Vec<String> {
        let mut topic = vec![];
        let mut results = vec![];
        self._drop_client(client, &mut topic, &mut results);
        results
    }

    // Helper to get subscribers all the way down
    fn _get_subscribers(
        &self,
        topic: &[&str],
        subscribers: &mut HashSet<ClientId>,
        double_wildcard: bool,
    ) {
        // If we're at the last level (or no topic level)
        if topic.len() == 0 {
            subscribers.extend(self.subscribers.iter());
            return;
        }

        // If the next topic level is a double wildcard (**)
        if topic[0] == DOUBLE_WILDCARD {
            self._get_subscribers(&topic[1..], subscribers, double_wildcard);
            for child in self.children.iter() {
                child
                    .1
                    ._get_subscribers(topic, subscribers, double_wildcard);
            }
        }

        if double_wildcard {
            self._get_subscribers(&topic[1..], subscribers, true);

            if let Some(child) = self.children.get(topic[0]) {
                child._get_subscribers(&topic[1..], subscribers, false);
            }

            return;
        }

        // If the next topic level is a single wildcard (*)
        if topic[0] == WILDCARD {
            for child in self.children.iter() {
                child
                    .1
                    ._get_subscribers(&topic[1..], subscribers, double_wildcard);
            }
            return;
        }

        // Otherwise we have a specfic topic
        if let Some(c) = self.children.get(topic[0]) {
            c._get_subscribers(&topic[1..], subscribers, double_wildcard);
        }
        if let Some(c) = self.children.get(WILDCARD) {
            c._get_subscribers(&topic[1..], subscribers, double_wildcard);
        }
        if let Some(c) = self.children.get(DOUBLE_WILDCARD) {
            c._get_subscribers(&topic[1..], subscribers, true);
        }
    }

    fn _drop_client(
        &mut self,
        client: ClientId,
        topic: &mut Vec<String>,
        results: &mut Vec<String>,
    ) {
        if self.subscribers.remove(&client) {
            if self.subscribers.len() == 0 {
                results.push(topic.join("/"));
            }
        }

        for (id, abook) in self.children.iter_mut() {
            topic.push(id.to_string());
            abook._drop_client(client, topic, results);
            topic.pop();
        }
    }

    // Internal helper to ensure that a child exists
    fn _child(&mut self, name: &str) {
        if !self.children.contains_key(name) {
            self.children.insert(name.to_string(), AddressBook::new());
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::topic::parse_topic;

    use super::*;

    #[test]
    fn test_topic_matches() {
        assert_key_match("a", "a", true);
        assert_key_match("a", "b", false);
        assert_key_match("a/b", "a/b", true);
        assert_key_match("a/*", "a/b", true);
        assert_key_match("*/b", "a/b", true);
        assert_key_match("*/c", "a/b", false);
        assert_key_match("a/**", "a/b/c", true);
        assert_key_match("a/**/d", "a/b/c/d", true);
        assert_key_match("a/**/d", "a/b/c/d/e", false);
        assert_key_match("a/**/c/d", "a/**/c/d", true);
        assert_key_match("a/b/**/d", "a/**/c/d", true);
    }

    #[test]
    fn test_subscribers_returns_valid_count() {
        let mut sub = AddressBook::new();
        let complex_topic = parse_topic("string/hello/world").unwrap();
        let wildcard_topic = parse_topic("*/hello/world").unwrap();
        let bye_topic = parse_topic("string/buhbye/world").unwrap();
        let non_topic = parse_topic("otherprefix/buhbye/world").unwrap();
        let _ = sub.subscribe(0, &complex_topic);
        let _ = sub.subscribe(1, &complex_topic);
        let _ = sub.subscribe(2, &wildcard_topic);
        let _ = sub.subscribe(4, &bye_topic);
        let _ = sub.subscribe(5500, &non_topic);
        let subs = sub.get_subscribers(&complex_topic);
        assert_eq!(3, subs.len());
        let subs = sub.get_subscribers(&bye_topic);
        assert_eq!(1, subs.len());
        sub.drop_client(0);
        let subs = sub.get_subscribers(&complex_topic);
        assert_eq!(2, subs.len());
    }

    #[test]
    fn test_drop_clients() {
        _test_client_drop(vec![vec![]]);
        _test_client_drop(vec![vec!["a"]]);
        _test_client_drop(vec![vec!["a/b"]]);
        _test_client_drop(vec![vec!["a/b/c"]]);
        _test_client_drop(vec![vec!["a", "b"]]);
        _test_client_drop(vec![vec!["a"], vec![]]);
        _test_client_drop(vec![vec!["a"], vec!["b"]]);
        _test_client_drop(vec![vec!["a/b"], vec!["b"]]);
        _test_client_drop(vec![vec!["a/b/c"], vec!["b"]]);
        _test_client_drop(vec![vec!["*"], vec![]]);
        _test_client_drop(vec![vec!["*", "a"], vec![]]);
        _test_client_drop(vec![vec!["*", "a"], vec!["a"]]);
        _test_client_drop(vec![vec!["*", "a"], vec!["*"]]);
        _test_client_drop(vec![vec!["**"], vec![]]);
        _test_client_drop(vec![vec!["**"], vec!["a"]]);
        _test_client_drop(vec![vec!["**"], vec!["*"]]);
        _test_client_drop(vec![vec!["**", "a"], vec![]]);
        _test_client_drop(vec![vec!["**", "a"], vec!["a"]]);
        _test_client_drop(vec![vec!["**", "a"], vec!["*"]]);
    }

    fn assert_key_match(topic_sub_str: &str, topic_pub_str: &str, expectation: bool) {
        let mut sub = AddressBook::new();
        let topic_sub = parse_topic(topic_sub_str).unwrap();
        let topic_pub = parse_topic(topic_pub_str).unwrap();

        let _ = sub.subscribe(0, &topic_sub);
        let subs = sub.get_subscribers(&topic_pub);

        if expectation {
            assert_eq!(subs.len(), 1);
        } else {
            assert_eq!(0, subs.len());
        }
    }

    fn _test_client_drop(subs: Vec<Vec<&str>>) {
        let mut sub = AddressBook::new();
        for i in 0..subs.len() {
            for topic in &subs[i] {
                let _ = sub.subscribe(i as ClientId, &parse_topic(&topic).unwrap());
            }
        }

        let empty_topics = sub.drop_client(0);
        for topic in &empty_topics {
            assert!(subs[0].contains(&topic.as_str()));
        }

        for topic in &subs[0] {
            let subscribers = sub.get_subscribers(&parse_topic(&topic).unwrap());
            assert!(!subscribers.contains(&0));
        }
    }
}
