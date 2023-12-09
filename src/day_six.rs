use std::fs;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn total_possible_ways(&self) -> u64 {
        let mut sum = 0;
        for t in 0..=self.time {
            let remaining_time = self.time - t;
            if remaining_time * t > self.distance {
                // println!("hold time {t} {} => {:?}", remaining_time * t, self);
                sum += 1;
            }
        }
        return sum;
    }
}

pub fn solve_part_one() {
    let path: &str = "./data/six.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    let mut races: Vec<Race> = Vec::new();
    let lns = contents.lines().collect::<Vec<&str>>();

    let distances = lns[1].split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .into_iter()
        .filter_map(|val| {
            if let Ok(num) = val.trim().parse::<u64>() {
                Some(num)
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();

    let times = lns[0].split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .into_iter()
        .filter_map(|val| {
            if let Ok(num) = val.trim().parse::<u64>() {
                Some(num)
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();

    if times.len() != distances.len() {
        panic!("times and distances donot have equal entries");
    }

    for (idx, _) in times.iter().enumerate() {
        races.push(Race {
            time: times[idx],
            distance: distances[idx],
        });
    }

    let completed: u64 = races.iter().map(|r| r.total_possible_ways()).product();

    // println!("Six: {:?}", races);
    println!("Six: {:?}", completed);
}

pub fn solve_part_two() {
    let path: &str = "./data/six.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    let mut races: Vec<Race> = Vec::new();
    let lns = contents.lines().collect::<Vec<&str>>();

    let distances = lns[1].split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .expect("Error");
    let times = lns[0].split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .expect("Error");

    let r = Race {
        time: times,
        distance: distances,
    };

    // println!("Six: {:?}", races);
    println!("Six: {:?}", r.total_possible_ways());
}
