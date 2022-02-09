mod note;
mod scanner;
mod wave;

use scanner::Scanner;
use wave::Wave;

fn main() {
    let mut sc = Scanner::default();
    let n: Vec<char> = sc.next::<String>().chars().collect();
    let ampl = sc.next::<f64>();
    let duration = sc.next::<u32>();
    let frame_rate = sc.next::<u32>();
    let fname = sc.next::<String>();

    let w = Wave::new(frame_rate);
    w.write(note::ntof(&n), ampl, duration, &fname).unwrap();
}
