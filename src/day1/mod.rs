use super::*;

pub fn task1() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut previous: u16 = 0;
    let mut counter: u16 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let depth_now: u16 = line.parse().unwrap();
        if depth_now > previous {
            counter +=1;
        }
        previous = depth_now;
    }
    println!("{}", counter - 1);
}
pub fn task2() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut depths: [u16; 3] = [0; 3];
    let mut depth_now: u16;

    let mut count_to_3: usize = 0;
    let mut counter: u16 = 0;
    let mut sum: u16;

    for line in reader.lines() {
        let line = line.unwrap();
        depth_now = line.parse().unwrap();

        if count_to_3 < 3 {
            count_to_3 += 1;
        } else {
            sum= depths.iter().sum();
            if sum - depths[0] + depth_now > sum {
                counter += 1;
            }        
        }
        depths = [depths[1], depths[2], depth_now]; 
    }

    println!("{}", counter);


}