use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read file");

    let groups = contents.split("\n\n")
                         .map(|ls| ls.split("\n")
                                     .map(|l| l.trim())
                                     .filter(|l| !l.is_empty()));
        
    let answers = read_all_groups_answers(groups);

    let total_anyone = sum_all_groups_answers(answers.iter().map(|&(any,_)| any));
    println!("Total (anyone): {}", total_anyone);

    let total_everyone = sum_all_groups_answers(answers.iter().map(|&(_,every)| every));
    println!("Total (everyone): {}", total_everyone);
}

fn read_all_groups_answers<'a, I, II>(groups: I) -> Vec<(u32, u32)>
where
    II: Iterator<Item = &'a str>,
    I: Iterator<Item = II>,
{
    let mut answers = Vec::new();
    for group in groups {
        let mut any: u32 = 0;
        let mut every: u32 = 0xFFFFFFFF;
        for person in group {
            let mut person_answers = 0;
            for c in person.chars() {
                if 'a' <= c && c <= 'z' {
                    let mask = 1 << ((c as u32) - ('a' as u32));
                    any |= mask;
                    person_answers |= mask;
                }
            }
            every &= person_answers;
        }
        answers.push((any, every));
    }
    answers
}

fn sum_all_groups_answers<I: Iterator<Item = u32>>(answers: I) -> u32 {
    answers.map(|x| sum_group_answers(x)).sum()
}

fn sum_group_answers(answers: u32) -> u32 {
    let mut sum = 0;
    for i in 0..26 {
        sum += (answers >> i) & 1;
    }
    sum
}
