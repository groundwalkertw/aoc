use super::*;

pub fn task1() {
    let f = File::open("input_3.txt").unwrap();
    let reader = BufReader::new(f);

    let mut counter: [i16; 12] = [0; 12];

    for line in reader.lines() {
        let line = line.unwrap();
        for (bit, count) in line.chars().zip(counter.iter_mut()){
            if bit == '1' {
                *count += 1;
            } else {
                *count -= 1;
            }

        }
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for i in 0..12 {
        if counter[i] > 0 {
            gamma += 2_u32.pow(11 - i as u32);
        } else {
            epsilon += 2_u32.pow(11 - i as u32);
        }
    }
    println!("{}", gamma * epsilon);
}
enum WhichMore {
    One,
    Zero,
    Equal
}

fn which_is_more(i: usize, set: &Vec<&Vec<u8>>) -> WhichMore {
    let mut count = 0;
    for v in set {
        if v[i] == 1 {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if count > 0 {
        WhichMore::One
    } else if count == 0 {
        WhichMore::Equal
    } else {
        WhichMore::Zero
    }
}

pub fn task2() {
    const N: usize = 12;
    let f = File::open("input_3.txt").unwrap();
    let reader = BufReader::new(f);

    let str_iter = reader.lines().map(|x| x.unwrap());

    let data: Vec<Vec<u8>> = str_iter.map(|str|-> Vec<u8> {
        str.chars().map(|bit|-> u8 {
            if bit == '1' {
                1
            } else {
                0}
        }).collect()
    }).collect();

    let mut oxy_set: Vec<_> = data.iter().collect();

    let mut co2_set = oxy_set.clone();

    for i in 0..N {
        oxy_set = match which_is_more(i, &oxy_set) {
            WhichMore::Zero => oxy_set.into_iter().filter(|v| v[i] == 0).collect(),
            _ => oxy_set.into_iter().filter(|v| v[i] == 1).collect(),
        };

        if co2_set.len() > 1 {
            co2_set = match which_is_more(i, &co2_set) {
                WhichMore::Zero => co2_set.into_iter().filter(|v| v[i] == 1).collect(),
                _ => co2_set.into_iter().filter(|v| v[i] == 0).collect(),
            };
        }       
        //println!("oxy: {:?}, co2: {:?}", oxy_set, co2_set);
    }

    let oxy_vec = oxy_set[0];
    let co2_vec = co2_set[0];

    let mut oxy_num: u32 = 0;
    let mut co2_num: u32 = 0;

    for i in 0..N {
        oxy_num += (oxy_vec[i] as u32) * 2_u32.pow((N - 1 - i) as u32);
        co2_num += (co2_vec[i] as u32) * 2_u32.pow((N - 1 - i) as u32);
    }

    println!("{}", oxy_num * co2_num);
}