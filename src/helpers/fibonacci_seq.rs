use std::usize;

pub fn get_fibonacci_number(num: usize) -> usize {
    if num <= 2 {
        return 1;
    }

    let mut prev_prev = 1;
    let mut prev = 1;
    let mut cur_num = 3;

    loop {
        match cur_num {
            n if n < num => {
                let tmp = prev;
                prev += prev_prev;
                prev_prev = tmp;

                cur_num += 1;
            }
            _ => return prev + prev_prev,
        }
    }
}
