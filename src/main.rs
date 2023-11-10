extern crate rayon;
use rayon::prelude::*;

fn lucky_numbers(matrix: Vec<Vec<u32>>) -> Vec<u32> {
    let r_min:Vec<u32> = matrix.iter().map(|r| *r.iter().min().unwrap()).collect();
    let c_max:Vec<u32> = (0..matrix[0].len()).into_iter().map(|j| matrix.iter().map(|r| r[j]).max().unwrap()).collect();
    
    let mut lucky = Vec::new();
    
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == r_min[i] && matrix[i][j] == c_max[j] {
                lucky.push(matrix[i][j]);
            }
        }
    }
    
    lucky
}

 fn par_lucky_numbers(matrix: Vec<Vec<u32>>) -> Vec<u32> {
    let r_min: Vec<u32> = matrix.par_iter()
        .map(|r| *r.iter().min().unwrap())
        .collect();
    
    let c_max: Vec<u32> = (0..matrix[0].len()).into_par_iter()
        .map(|j| matrix.iter().map(|r| r[j]).max().unwrap())
        .collect();

        let lucky: Vec<u32> = matrix.par_iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.iter()
                .enumerate()
                .filter_map(|(j, &e)| {
                    if e == r_min[i] && e == c_max[j] {
                        Some(e)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>() 
        })
        .collect();

    
    lucky
}

fn generate_matrix(rows: usize, columns: usize) -> Vec<Vec<u32>> {
    (0..rows).map(|_| (0..columns).map(|_| rand::random::<u32>() % 10 + 1).collect()).collect()
}
 
fn main() {
    let matrix = generate_matrix(5,5);
    for row in &matrix {
        println!("{:?}", row);
    }
    let start = std::time::Instant::now();
    println!("Lucky numbers: {:?}", lucky_numbers(matrix.clone()));
    println!("seq took {:?}", start.elapsed());
    let start = std::time::Instant::now();
    println!("Lucky numbers: {:?}", par_lucky_numbers(matrix));
    println!("par took {:?}", start.elapsed());
}