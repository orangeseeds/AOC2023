use std::{fs, u64};

#[derive(Debug)]
struct MapRecord(u64, u64, u64);
#[derive(Debug)]
enum Error {
    MapRecordParseError,
}

impl TryFrom<&str> for MapRecord {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut record = MapRecord(0, 0, 0);
        let split_recs: Vec<&str> = value.trim().split_whitespace().collect();
        if split_recs.len() < 3 {
            return Err(Error::MapRecordParseError);
        }
        record.0 = split_recs[0].parse::<u64>().expect("parse error!");
        record.1 = split_recs[1].parse::<u64>().expect("parse error!");
        record.2 = split_recs[2].parse::<u64>().expect("parse error!");
        Ok(record)
    }
}

pub fn solve_part_one() {
    let path: &str = "./data/five.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    let mut inputs = Vec::new();

    let paragraphs = contents.split("\n\n").collect::<Vec<&str>>();
    let mut records_set = Vec::new();

    let input_nums = &paragraphs[0].split_whitespace().collect::<Vec<&str>>();
    input_nums[1..].iter().for_each(|&val| {
        let parsed: u64 = val
            .parse::<u64>()
            .expect("error occured while parsing input!");
        inputs.push(parsed)
    });

    for (idx, el) in paragraphs.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        let mut data = Vec::new();
        el.split(":").collect::<Vec<&str>>()[1]
            .trim()
            .lines()
            .for_each(|val| {
                let records: MapRecord = val.try_into().expect("parse error!");
                // println!("{:?}", records);
                data.push(records);
            });
        records_set.push(data);
    }
    let mut locs = Vec::new();

    for i in inputs {
        let mut val = i;
        'outer: for v in records_set.iter() {
            for recs in v {
                if val >= recs.1 && val <= recs.1 + recs.2 {
                    val = recs.0 + (val - recs.1);
                    // println!("{}<-{} | {i}->{val}", recs.0, recs.1);
                    continue 'outer;
                }
            }
        }
        // println!("Final:: {i}->{val}");
        locs.push(val);
    }

    locs.sort();
    println!("day five smallest loc: {}", locs[0]);
}

pub fn solve_part_two_brute() {
    let path: &str = "./data/five.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    let mut inputs = Vec::new();

    let paragraphs = contents.split("\n\n").collect::<Vec<&str>>();
    let mut records_set = Vec::new();

    let input_nums = &paragraphs[0].split_whitespace().collect::<Vec<&str>>();
    input_nums[1..].iter().for_each(|&val| {
        let parsed: u64 = val
            .parse::<u64>()
            .expect("error occured while parsing input!");
        inputs.push(parsed)
    });

    for (idx, el) in paragraphs.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        let mut data = Vec::new();
        el.split(":").collect::<Vec<&str>>()[1]
            .trim()
            .lines()
            .for_each(|val| {
                let records: MapRecord = val.try_into().expect("parse error!");
                // println!("{:?}", records);
                data.push(records);
            });
        records_set.push(data);
    }
    let mut locs = Vec::new();
    let mut min = u64::MAX;

    for (idx, &i) in inputs.iter().enumerate() {
        if idx % 2 == 1 {
            continue;
        }
        println!("{idx}:: {i}->{min}");
        for j in i..=(i + inputs[idx + 1]) {
            let mut val = j;
            'outer: for v in records_set.iter() {
                for recs in v {
                    if val >= recs.1 && val <= recs.1 + recs.2 {
                        val = recs.0 + (val - recs.1);
                        // println!("{}<-{} | {i}->{val}", recs.0, recs.1);
                        min = min.min(val);
                        continue 'outer;
                    }
                }
            }
            locs.push(val);
            // break;
            println!("{idx}:: {j}->{min}");
        }
        // println!("{idx}:: {i}->{min}");
    }

    locs.sort();
    println!("day five smallest loc: {}, {min}", locs[0]);
}

pub fn solve_part_two() {
    let path: &str = "./data/five.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    let mut inputs = Vec::new();

    let paragraphs = contents.split("\n\n").collect::<Vec<&str>>();
    let mut records_set = Vec::new();

    let input_nums = &paragraphs[0].split_whitespace().collect::<Vec<&str>>();
    input_nums[1..].iter().for_each(|&val| {
        let parsed: u64 = val
            .parse::<u64>()
            .expect("error occured while parsing input!");
        inputs.push(parsed)
    });

    for (idx, el) in paragraphs.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        let mut data = Vec::new();
        el.split(":").collect::<Vec<&str>>()[1]
            .trim()
            .lines()
            .for_each(|val| {
                let records: MapRecord = val.try_into().expect("parse error!");
                // println!("{:?}", records);
                data.push(records);
            });
        records_set.push(data);
    }
    let mut locs = Vec::new();
    let mut min = u64::MAX;

    records_set.reverse();

    for i in (0..).step_by(10) {
        let mut val = i;
        'outer: for v in records_set.iter() {
            for recs in v {
                if val >= recs.0 && val <= recs.0 + recs.2 {
                    val = recs.1 + (val - recs.0);
                    // println!("{}<-{} | {i}->{val}", recs.0, recs.1);
                    min = min.min(val);
                    continue 'outer;
                }
            }
        }
        println!("{i}->{val}");

        for (idx, &j) in inputs.iter().enumerate() {
            if idx % 2 == 1 {
                continue;
            }

            if (j..j + inputs[idx + 1]).contains(&val) {
                locs.push(i);
            }
        }
        if locs.len() == 1 {
            break;
        }
    }

    locs.sort();
    println!("day five smallest loc: {:?}", locs);
}
