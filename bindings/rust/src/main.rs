use c_kzg::verify_kzg_proof_wrapper;

fn main() {
    let iterations = 128;
    println!("Number of iterations: {}", iterations);

    for i in 0..iterations {
        let _result = verify_kzg_proof_wrapper();
        println!("Iteration {} validation result: {}", i, _result);
    }
}