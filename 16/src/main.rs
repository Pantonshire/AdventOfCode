use std::collections::HashMap;
use std::collections::HashSet;

type Rule = Vec<(u32,u32)>;

fn main() {
    let contents = include_str!("../input");

    let mut parts = contents.split("\n\n")
        .map(|part| part.split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty()));

    let rules = read_rules(parts.next().expect("No rules section"))
        .expect("Error reading rules");

    let my_ticket = read_ticket(parts.next()
            .expect("No my ticket secion")
            .skip(1)
            .next()
            .expect("No ticket to read for my ticket"))
        .expect("Error reading my ticket");

    let tickets = parts.next()
        .expect("No tickets section")
        .skip(1)
        .map(read_ticket)
        .collect::<Option<Vec<Vec<u32>>>>()
        .expect("Error reading ticket");

    let mut valid_tickets = Vec::new();

    let mut error_rate = 0;

    for ticket in tickets.iter() {
        let e = ticket.iter()
            .map(|&x| if rules.iter().any(|(_,rule)| test_rule(rule, x)) { 0 } else { x })
            .sum::<u32>();
        if e == 0 {
            valid_tickets.push(ticket);
        } else {
            error_rate += e;
        }
    }

    println!("Part 1: {}", error_rate);

    let n_fields = my_ticket.len();

    let mut field_positions: HashMap<String, Vec<usize>> = HashMap::new();
    for (name,_) in rules.iter() {
        field_positions.insert(name.clone(), (0..n_fields).collect());
    }

    for ticket in valid_tickets {
        let mut ticket_field_positions: HashMap<String, Vec<usize>> = HashMap::new();
        for (name,rule) in rules.iter() {
            ticket_field_positions.insert(name.clone(), ticket.iter()
                .enumerate()
                .filter(|&(_,&x)| test_rule(rule, x))
                .map(|(i,_)| i)
                .collect());
        }
        reduce_possible_positions(&mut ticket_field_positions);
        for (k,v) in field_positions.iter_mut() {
            v.retain(|x| ticket_field_positions[k].contains(x));
        }
    }

    reduce_possible_positions(&mut field_positions);

    let prod = field_positions.iter()
        .filter(|(k,_)| k.starts_with("departure"))
        .map(|(_,v)| my_ticket[v[0]] as u64)
        .product::<u64>();

    println!("Part 2: {}", prod);
}

fn read_rules<'a, I>(lines: I) -> Option<HashMap<String, Rule>>
where
    I: Iterator<Item=&'a str>
{
    let mut rules = HashMap::new();
    for line in lines {
        let (name, rule) = read_rule(line)?;
        rules.insert(name.to_owned(), rule);
    }
    Some(rules)
}

fn read_rule(line: &str) -> Option<(&str, Rule)> {
    let mut parts = line.splitn(2, ":")
        .map(|s| s.trim());
    let name = parts.next()?;
    let rule = parts.next()?
        .split("or")
        .map(|s| s.trim())
        .map(read_range)
        .collect::<Option<Rule>>()?;
    Some((name, rule))
}

fn read_range(range_str: &str) -> Option<(u32,u32)> {
    let mut parts = range_str.splitn(2, "-")
        .map(|s| s.parse());
    let min = parts.next()?.ok()?;
    let max = parts.next()?.ok()?;
    Some((min,max))
}

fn read_ticket(line: &str) -> Option<Vec<u32>> {
    line.split(",")
        .map(|s| s.parse().ok())
        .collect()
}

fn test_rule(rule: &Rule, x: u32) -> bool {
    return rule.iter().any(|&(min,max)| min <= x && x <= max)
}

fn reduce_possible_positions(positions: &mut HashMap<String, Vec<usize>>) {
    let mut to_remove = HashSet::new();
    loop {
        let mut changed = false;
        for v in positions.values().filter(|v| v.len() == 1) {
            to_remove.insert(v[0]);
        }
        for (_,v) in positions.iter_mut() {
            if v.len() > 1 {
                v.retain(|i| {
                    let remove = to_remove.contains(i);
                    if remove {
                        changed = true;
                    }
                    !remove
                });
            }
        }
        if !changed {
            break;
        }
    }
}
