#[allow(dead_code)]
use std::time::{Instant};
// use std::time::{Duration};
// use std::thread::sleep;
mod matrix_csr;


fn main() {
    let now = Instant::now();
    // let file = "pwtk.mtx";
    // let file = "lp_nug05.mtx";
    let file = "test1.mtx";
    // let file = "apache2.mtx";
    // let file = "lns__131.mtx";
    let matrix = matrix_csr::read_matrix_market(file);
    println!("matrix:{:?}", matrix);
    println!("|row_i|: {:?}; |col_i|:{:?}", matrix.row_index.len(), matrix.col_index.len());
    println!("BW: {}", matrix.bandwidth());
    // matrix.cmr();
    // println!("BW: {}", matrix.bandwidth());
    println!("time = {}ms", now.elapsed().as_millis());
    // sleep(Duration::new(5, 0));
}