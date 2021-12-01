use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Rule<'a> {
    Str(&'a str),
    Opts(Vec<Vec<usize>>)
}

fn main() {
    let contents = include_str!("../input");
    let mut parts = contents.split("\n\n");

    let mut rules = parts.next()
        .expect("No rules section")
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(read_rule)
        .collect::<Option<HashMap<usize, Rule>>>()
        .expect("Error reading rules");
    
    let messages = parts.next()
        .expect("No messages section")
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
        
    println!("Part 1: {}", messages.iter().filter(|m| test_rule(&rules, 0, m)).count());

    rules.insert(8, Rule::Opts(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Opts(vec![vec![42, 31], vec![42, 11, 31]]));

    println!("Part 2: {}", messages.iter().filter(|m| test_rule(&rules, 0, m)).count());
}

fn read_rule(line: &str) -> Option<(usize, Rule)> {
    let index_rule = line.splitn(2, ":")
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    if index_rule.len() != 2 {
        return None;
    }
    let index = index_rule[0].parse::<usize>().ok()?;
    match index_rule[1].strip_prefix("\"").and_then(|s| s.strip_suffix("\"")) {
        Some(s) => return Some((index, Rule::Str(s))),
        None => (),
    }
    let opts = index_rule[1].split("|")
        .map(|s| s.split_whitespace()
            .map(|x| x.parse::<usize>().ok())
            .collect::<Option<Vec<usize>>>())
        .collect::<Option<Vec<Vec<usize>>>>()?;
    Some((index, Rule::Opts(opts)))
}

fn test_rule(rules: &HashMap<usize, Rule>, rule_no: usize, msg: &str) -> bool {
    match match_rule(rules, rule_no, msg) {
        Some(v) => v.iter().filter(|s| s.is_empty()).count() > 0,
        None => false,
    }
}

fn match_rule<'a>(rules: &HashMap<usize, Rule>, rule_no: usize, msg: &'a str) -> Option<Vec<&'a str>> {
    match &rules[&rule_no] {
        Rule::Str(s) => {
            if msg.starts_with(s) {
                Some(vec![&msg[s.len()..]])
            } else {
                None
            }
        },
        Rule::Opts(opts) => {
            let mut msgs = Vec::new();
            for opt in opts {
                let mut opt_msgs = vec![msg];
                for &i in opt {
                    let mut next_opt_msgs = Vec::new();
                    for opt_msg in opt_msgs.iter() {
                        match match_rule(rules, i, opt_msg) {
                            Some(xs) => next_opt_msgs.extend(xs),
                            None => (),
                        }
                    }
                    opt_msgs = next_opt_msgs;
                }
                msgs.extend(opt_msgs);
            }
            if msgs.len() > 0 {
                Some(msgs)
            } else {
                None
            }
        }
    }
}
