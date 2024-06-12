use rand::Rng;
extern crate savefile;
use savefile::prelude::*;
use serde::{Serialize, Deserialize};



fn main() {
    let num_rice: usize = 2000000;
    let columns: Vec<usize> = vec![8, 16, 32, 64, 128, 256, 512, 1024];
    let mut full_avalanches: Vec<Vec<usize>> = Vec::new();
    let mut total_height_zero: Vec<Vec<usize>> = Vec::new();


    for num_columns in columns {

        let fin_idx: usize = num_columns-1;
        let mut check_set: Vec<usize> = Vec::new();
        let mut avalanches: Vec<usize> = Vec::new();
        let mut height_zero: Vec<usize> = Vec::new();

        simulate_ricepile(num_rice, &mut check_set, fin_idx, &mut avalanches, num_columns, &mut height_zero);

        full_avalanches.push(avalanches);
        total_height_zero.push(height_zero);

    }

    let mut i = 0;
    for (height, avalanche) in total_height_zero.iter().zip(full_avalanches){

        let mut f = std::fs::File::create(format!("heights_zeroth_{i}.bin")).unwrap();
        serde_pickle::to_writer(&mut f, &height, Default::default()).unwrap();

        let mut f = std::fs::File::create(format!("avalanches_{i}.bin")).unwrap();
        serde_pickle::to_writer(&mut f, &avalanche, Default::default()).unwrap();


        i += 1;
    }

}



fn simulate_ricepile(num_rice: usize, check_set: &mut Vec<usize>, fin_idx : usize, avalanches: &mut Vec<usize>, num_columns: usize, height_zero: &mut Vec<usize>){
    let mut avg_1 : usize = 0;
    let mut landscape: Vec<SandPile> = Vec::new();

    for _ in 0..num_columns{
        landscape.push(SandPile {height : 0, gradient : generate_gradient()})
    }

    for i in 0..num_rice{
        let mut ava: usize = 0;
        landscape[0].add();
        check_set.push(0);
        height_zero.push(landscape[0].height);


        while !check_set.is_empty(){

            match check_set.pop(){

                Some(col_idx) => {
                    match col_idx{
                        x if x == fin_idx => {
                            if landscape[col_idx].height > landscape[col_idx].gradient{
                                landscape[col_idx].remove();
                                check_set.push(col_idx-1);
                                check_set.push(col_idx);
                                ava += 1;
                            }
                        },

                        0 => {
                            if landscape[col_idx].over_threshold(&landscape[col_idx+1]){
                                fall(&mut landscape, col_idx);
                                check_set.push(col_idx+1);
                                check_set.push(col_idx);
                                ava += 1;
                            }
                        },
                        
                        _ => {
                            if landscape[col_idx].over_threshold(&landscape[col_idx+1]){
                                fall(&mut landscape, col_idx);
                                check_set.push(col_idx-1);
                                check_set.push(col_idx+1);
                                ava += 1;
                            }
                        }
                    }
                },

                _ => {
                    println!("All settled for rice no. {}", i);
                },
            }

        }
        avalanches.push(ava);
        avg_1 += landscape[0].height;


    }

    println!("average height of first column: {}", avg_1 as f64/ num_rice as f64);

}


fn fall(landscape: &mut Vec<SandPile>, this: usize) {
    landscape[this].remove();
    landscape[this+1].add();
}


struct SandPile 
{
    height: usize, 
    gradient: usize,

}

fn generate_gradient() -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..3)
}


impl SandPile {

    fn over_threshold(&self, next: &SandPile) -> bool {
        let dh = self.height - next.height;
        return dh > self.gradient
    }

    fn add(&mut self){
        self.height += 1
    }

    fn remove(&mut self){
        self.height -= 1;
        self.gradient = generate_gradient()
    }


}