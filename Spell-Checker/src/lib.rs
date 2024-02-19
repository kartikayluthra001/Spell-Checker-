mod RustProject1;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn power_of_two(x: i8) {
    if x == 1 {
        println!("true");
        return;
    }

    let mut y: i8 = x;

    while y > 1 {
        if y % 2 != 0 {
            return;
        }
        y /= 2;
    }

    println!("true");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
