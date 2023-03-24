#[derive(Clone)]
pub enum ResultSetType {
    Ok,
    ResultSetError,
    None,
}

fn main() {
    use std::time::Instant;
    
    const STMT_COUNT:usize = 100000000;

    let now = Instant::now();
    {
        let mut vector = vec![];      
        for _ in 0..STMT_COUNT {
            vector.push(ResultSetType::None);
        } 
        assert_eq!(vector.len(), STMT_COUNT);
    }
    let elapsed = now.elapsed();
    println!("vec![]:\t\t\t\t\t {:.2?}", elapsed);



    
    let now = Instant::now();
    {
        let mut vector = Vec::with_capacity(STMT_COUNT);      
        for _ in 0..STMT_COUNT {
            vector.push(ResultSetType::None);
        }
        assert_eq!(vector.len(), STMT_COUNT);
    }
    let elapsed = now.elapsed();
    println!("Vec::with_capacity:\t\t\t {:.2?}", elapsed);




    let now = Instant::now();
    // fastest
    {
        let vector = vec![ResultSetType::None; STMT_COUNT];     
        assert_eq!(vector.len(), STMT_COUNT);
    }    let elapsed = now.elapsed();
    println!("vec![ResultSetType::None; STMT_COUNT]:\t {:.2?}", elapsed);


}

