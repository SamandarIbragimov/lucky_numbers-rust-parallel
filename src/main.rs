extern crate rayon;
use rayon::prelude::*;

fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let r_min:Vec<i32> = matrix.iter().map(|r| *r.iter().min().unwrap()).collect();
    let c_max:Vec<i32> = (0..matrix[0].len()).into_iter().map(|j| matrix.iter().map(|r| r[j]).max().unwrap()).collect();
    
    matrix.iter()
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
        .collect()
}

 fn par_lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let r_min: Vec<i32> = matrix.par_iter()
        .map(|r| *r.iter().min().unwrap())
        .collect();
    
    let c_max: Vec<i32> = (0..matrix[0].len()).into_par_iter()
        .map(|j| matrix.iter().map(|r| r[j]).max().unwrap())
        .collect();

        matrix.par_iter()
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
        .collect()

}

fn generate_matrix(rows: i32, columns: i32) -> Vec<Vec<i32>> {
    (0..rows).map(|_| (0..columns).map(|_| rand::random::<i32>() % 100000).collect()).collect()
}
 
fn main() {
    let matrix = generate_matrix(100,100);
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
