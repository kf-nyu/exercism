pub fn nth(n: u32) -> u32 {

    if n==0 {return 2 };

    let mut k:u32 = 2;
    let mut i:u32 = 0;
    
    loop {
        k += 1;
        if is_prime(k) {i+=1};

    if i == n { break; }    
    }
    
    k
}
    fn is_prime(k: u32) -> bool {
        if k == 2 {return true};

        for i in 2..k {
            if k.is_multiple_of(i) {
                return false
            };
        }
        true
    }
