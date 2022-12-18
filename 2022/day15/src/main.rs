use std::collections::HashSet;


const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");


#[derive(Debug)]

struct Sensor{
    sensor: Point,
    beacon: Point,
    radius: usize
}

type Point = (i32,i32);


impl Sensor{

    fn from_text(input: &str)->Vec<Self>{
        input
        .split('\n')
        .map(Sensor::item_from_text)
        .collect()
       
    }
    fn item_from_text(input: &str)->Self{
        let relevant = &input[10..];
        let (sensor,beacon) = relevant.split_once(": closest beacon is at ").unwrap();
        let (x,y) = Sensor::get_point(sensor);
        let (x2,y2) = Sensor::get_point(beacon);

        let radius = ((x-x2).abs() + (y-y2).abs() ) as usize ;
        Self { sensor: (x,y) , beacon : (x2,y2), radius }
    }

    fn get_point(i: &str)-> Point{
        let (x,y) = i.split_once(", ").unwrap();
        let x = x[2..].parse().unwrap();
        let y = y[2..].parse().unwrap();
        (x,y)
    }
    fn manhattan_distance(p1: &(i32,i32), p2: &(i32,i32))-> usize{
        ((p1.0-p2.0).abs() + (p1.1 - p2.1).abs() ) as usize
    }

    fn is_in_radius(&self, point:&(i32,i32))-> bool{
        Sensor::manhattan_distance(&self.sensor, point) <= self.radius
    }
    
}

fn get_intersection_with_y(sensors: &Vec<Sensor>, y:i32)-> HashSet<Point>{

    let mut result = HashSet::new();
    for sensor in sensors{
        let base_x = sensor.sensor.0;
        for i in 0..=sensor.radius{
            if  sensor.is_in_radius(&(base_x + i as i32, y)){
                result.insert((base_x + i as i32, y));
                result.insert((base_x - i as i32, y));
            }else{
                break;
            }
        }
    }
    for sensor in sensors{
        result.remove(&sensor.beacon);

    } 
    result
}
fn get_intersection_or_beacon_with_y(sensors: &Vec<Sensor>, y:i32)-> HashSet<Point>{

    let mut result = HashSet::new();
    for sensor in sensors{
        let base_x = sensor.sensor.0;
        for i in 0..=sensor.radius{
            if  sensor.is_in_radius(&(base_x + i as i32, y)){
                result.insert((base_x + i as i32, y));
                result.insert((base_x - i as i32, y));
            }else{
                break;
            }
        }
    }
    result
}
fn find_gap(points: &HashSet<Point>, y:i32)-> Option<Point>{
    let mut xs = points.iter().map(|(x,y)| *x).collect::<Vec<_>>();
    xs.sort();
    let mut start = xs[0] - 1;
    for i in xs{
        start += 1;
        if start != i{
            return Some((start,y));
        }
    }
    None
}
fn main() {
    let sensors = Sensor::from_text(INPUT_DATA);
    println!("{} sensors found", sensors.len());
    /*
    let set = get_intersection_with_y(&sensors, 2000000);
    println!("Part 1: {}", set.len() );
 */
    for i in 0 ..4000000{
        let set = get_intersection_or_beacon_with_y(&sensors, i);
        if let Some(gap) = find_gap(&set, i){
            println!("Part 2: {:?}", gap);
            return;
        }else{
            println!("{} done", i);
        }

    }
    

}
