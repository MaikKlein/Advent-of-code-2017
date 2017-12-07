use std::fs::File;
use std::collections::HashMap;
use std::io::Read;
type Input<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn parse(s: &str) -> Input {
    s.lines()
        .map(|line| {
            let split: Vec<_> = line.split("->").collect();
            let name = split[0].split_whitespace().collect::<Vec<_>>()[0];
            let mut sublist = Vec::new();
            if let Some(sublist_str) = split.get(1) {
                sublist.extend(sublist_str.split(",").map(str::trim));
            }
            (name, sublist)
        })
        .collect()
}

pub fn find_root<'a>(input: &Input<'a>) -> Option<&'a str> {
    let mut map: HashMap<_, _> = input.keys().map(|key| (key, 1)).collect();
    input.values().flat_map(|v| v.iter()).for_each(|name| {
        *map.get_mut(name).expect("No name") += 1;
    });
    map.iter()
        .find(|&(key, &value)| value == 1)
        .map(|(&&name, _)| name)
}

fn main() {
    let mut file = File::open("src/bin/day7.input").expect("file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("read");
    let map = parse(&input);
    println!("Part 1 {:?}", find_root(&map));
}
