fn main() {
    let mut s = String::new();
    let data = std::fs::read_to_string("C:\\Users\\Sergio\\numproc\\data.txt").unwrap();
    let data: Vec<(f64, f64)> = data
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap() as f64;
            let y = parts.next().unwrap().parse::<f64>().unwrap();
            (x, y)
        })
        .collect();

    let mut x = data.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let mut y = data.iter().map(|(_, y)| *y).collect::<Vec<_>>();

    x = x
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, x)| *x)
        .collect::<Vec<_>>();
    y = y
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, y)| *y)
        .collect::<Vec<_>>();

    let n = 10;
    println!("Enter the value of x for which y is to be found");
    std::io::stdin().read_line(&mut s).unwrap();
    let xi: f64 = s.trim().parse().unwrap();

    let mut li = Vec::<f64>::new();
    for i in 0..n {
        let mut l = 1.0;
        for j in 0..n {
            if i != j {
                l *= (xi - x[j]) / (x[i] - x[j]);
            }
        }
        li.push(l);
    }

    let mut y1 = 0.0;
    for i in 0..n {
        y1 += y[i] * li[i];
    }
    println!("y = {}", y1);

}
