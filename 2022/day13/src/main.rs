use std::{str::Chars, fmt, cmp::Ordering};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Packet {
    Val(usize),
    List(Vec<Packet>) 
}



impl fmt::Debug for Packet {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
       
        match self{
            Packet::Val(x) => write!(f, "{x}"),
            Packet::List(l) =>   f.debug_list().entries(l).finish(),
        }
    }
} 

impl Packet {
    fn cmp(&self, right:&Self) -> Ordering {
        match (self,right){
            (Packet::Val(s),Packet::Val(r)) => s.cmp(r),
            (Packet::List(s),Packet::List(o)) => {
                let min = s.len().min(o.len());
                for i in 0..min{
                    let cmp = s[i].cmp(&o[i]);
                    if cmp != Ordering::Equal{
                        return  cmp;
                    }
                }
                s.len().cmp( &o.len())
            },
            (Packet::List(_),Packet::Val(o)) => self.cmp(&Packet::List(vec![Packet::Val(*o)])),
            (Packet::Val(s),Packet::List(_)) => Packet::List(vec![Packet::Val(*s)]).cmp(right)
        }
        
    }

    fn from_text(input: &str)->Packet{
        let mut chars = input.chars();
        chars.next();
        Packet::from_chars(&mut chars )
    }

    fn from_chars(chars: &mut Chars )->Packet{
        let mut res = Vec::new();

        let mut buffer = "".to_owned();
        while let Some(c) = chars.next(){
            if c >= '0' && c <= '9'{
                buffer.push(c);
            }else if c == '[' {
                let p = Packet::from_chars(chars);
                res.push(p)

            }else{
                if let Ok(val) = buffer.parse(){

                res.push(Packet::Val(val));
                buffer .clear();

                }
                if c == ']' {
                    break;
                }
            }
        }
        
        return Packet::List(res);

       
    }
}
type Pair = (Packet,Packet);



fn read_input(input: &str)-> Vec<Pair>{
    input.split("\n\n")
    .map(|p| {
        let (l,r) = p.split_once('\n').unwrap();
        (Packet::from_text(l), Packet::from_text(r))
    })
    .collect()
}
fn main() {

    const INPUT: &str = include_str!("input.txt");
    let decoded = read_input(INPUT);

    for (p1,p2) in &decoded{


     println!("{:#?}", p1.cmp(&p2));
     println!("{:?}\n{:?}\n", p1,p2);
    }
    let result: usize = decoded
    .iter()
    .enumerate()
    .map(|(i, (p1,p2))| if p1.cmp(p2) == Ordering::Less {Some(i+1)}else{None})
    .flatten()
    .sum();
    

    println!("Part 1 {}", result);

    let mut packs = INPUT
    .split_ascii_whitespace()
    .chain("[[2]] [[6]]".split_ascii_whitespace())
    .map(Packet::from_text)
    .collect::<Vec<_>>();
    packs.sort_by(|a, b| a.cmp(b));

    
    for p in &packs{
        println!("{:?}", p);
       }

}
