use std::collections::HashSet;


const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");


#[derive(Debug)]

struct Sensor{
    sensor: Point,
    beacon: Point,
    radius: usize
}
#[derive(Debug)]
struct Range{
    start:i32,
    end:i32
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

fn get_intersection_with_y(sensors: &Vec<Sensor>, y:i32)-> Vec<Range>{

    let mut result:Vec<Range> = Vec::new();
    for sensor in sensors{
        let base_x = sensor.sensor.0;
        let  distance =  Sensor::manhattan_distance(&sensor.sensor, &(base_x,y));

        if distance > sensor.radius {
            continue;
        }
        let range_radius = sensor.radius - distance;
        let start = base_x - range_radius as i32;
        let end = base_x + range_radius as i32;

        if result.iter().all(|r| (start < r.start || r.end < end)){
            result.push(Range{start,end});
        }
    }

    //println!("Intersection for row {} is {:?}",y,&result);
    result
}
fn find_gap(points: &Vec<Range>, y:i32)-> Option<Point>{
    let start = points.iter().map(|r| r.start).min().unwrap();
    let end = points.iter().map(|r| r.end).max().unwrap();


    let mut i = start;
    while start <= i && i < end{
        let mut found = false;
        for range in points{
            if range.start <= i && i < range.end{
                found = true;
                i = range.end+1;
            }
        }
        if !found {
            return Some((i,y))
        }
    }
    
    None
    
    
}
fn main() {
    let sensors = Sensor::from_text(INPUT_DATA);
    println!("{} sensors found", sensors.len());

    let factor = 4000000;
    let max = factor;
    for index in 0..max{
        let i = max - index;
        let set = get_intersection_with_y(&sensors, i);
        if let Some((x,y)) = find_gap(&set, i){
            println!("Part 2: {} {} {:?}", x,y, x  as i64 * factor as i64 + y as i64);
            return;
        }else{
            println!("{} done", i);
        }

    }
    

}
