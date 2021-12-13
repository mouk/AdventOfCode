use std::fs;
struct HeatMap{
    data: Vec<u8>,
    dimension: usize
}

impl HeatMap{
    fn new_from_file_content(content: Vec<u8>) -> Self{
        let data = content.iter()
        .filter(|&&x| x >= 48 && x <= 57)
        .cloned()
        .map(|c| c - 48)
        .collect();

        return HeatMap{
            data : data,
            dimension : 100
        };        
    }
    fn get_neighbors(&self, index:usize) -> Vec<u8>{
       let mut result:Vec<u8> = Vec::new();
       let dim = self.dimension;
        //north
       if index >= dim{
           result.push(self.data[index-dim])
       }
        //east
        if (index+1) % dim != 0{
            result.push(self.data[index+1])
        } 
        //south
        if (index + dim) < self.data.len(){
            result.push(self.data[index+dim])
        } 
        //west
        if index % dim != 0{
            result.push(self.data[index-1])
        } 
       return result;
    }
    
    fn get_neighbors_index(&self, index:usize) -> Vec<usize>{
        let mut result:Vec<usize> = Vec::new();
        let dim = self.dimension;
         //north
        if index >= dim{
            result.push(index-dim)
        }
         //east
         if (index+1) % dim != 0{
             result.push(index+1)
         } 
         //south
         if (index + dim) <  self.data.len(){
             result.push(index+dim)
         } 
         //west
         if index > 0{
             result.push(index-1)
         } 
        return result;
     }
     
    fn get_neighbors_index_expect_for(&self, start:&Vec<usize>, except:&Vec<usize>) -> Vec<usize>{
      let mut result = Vec::new();
      for start_point in start.iter(){
          for n in self.get_neighbors_index(*start_point){
              if  self.data[n] != 9 && !except.contains(&n)  && !result.contains(&n) && self.data[*start_point] < self.data[n] {
                result.push(n);
              }
          }
      }
      return result;
     }
    fn get_local_minimum_index(&self) -> Vec<usize>{
        return (0..(self.data.len()))
            .filter(|index|{
                let neighbors = self.get_neighbors(*index);
                let x = self.data[*index];
                for n in neighbors {
                    if x >= n {
                        return false;
                    }
                }
                return true;
            })
            .collect();
     }
     
    fn get_local_minima(&self) -> Vec<u8>{
        return self
            .get_local_minimum_index()
            .iter()
            .map(|&x| self.data[x])
            .collect();
     }

    fn get_basin(&self, index:usize) -> Vec<usize>{
        
        println!("gettings basin for {:?}", index);
        let mut result = Vec::new();
        result.push(index);
        let add = &mut Vec::new();
        add.push(index);
        loop {
            
            println!("expanding on {:?}", add);
            let n_add = self
            .get_neighbors_index_expect_for(&add, &result);

            for a in n_add.iter().cloned() {
                result.push(a);
            }
            if n_add.len() == 0{
                break;
            };
            *add = n_add;
        }
        return result;
    }
}

fn main() {
    let content:Vec<u8> = fs::read("./src/input.txt")
    .expect("war richtig");

    let heat_map = HeatMap::new_from_file_content(content);

    let result:u16 = heat_map.get_local_minima()
    .iter()
    .map(|x| u16::from(x + 1))
    .sum();
    println!("{:?}", result);

    
    let mut basins = heat_map
    .get_local_minimum_index()
    .iter()
    .map(|x| heat_map.get_basin(*x))
    .collect::<Vec<Vec<usize>>>();

    basins.sort_by(|a, b| b.len().cmp(&a.len()));

    
    println!("{:?}", basins);
    
    let nums:Vec<usize> = 
    basins
    .iter()
    .take(3)
    .map(|x| x.len())
    .collect();
    //basins.reverse();


    //


    //.collect();
    println!("{:?}", nums[0] * nums[1] *nums[2]);
}