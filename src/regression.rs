fn main() {
    let mut n = 1;
    loop {
        if n == 3 {
            break;
        }
        match n {
            1 => linear(),
            2 => polynomial(),
            _ => println!("Invalid option"),
        }
        n+=1;
    }
}

fn polynomial() {
    let data = std::fs::read_to_string("data.txt").unwrap();
    let data: Vec<(f64, f64)> = data
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap() as f64;
            let y = parts.next().unwrap().parse::<f64>().unwrap();
            (x, y)
        })
        .collect();

    let x = data.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let y = data.iter().map(|(_, y)| *y).collect::<Vec<_>>();
    let arr = (x, y);
    let (a0, a1, a2) = poly_reg(&arr);
    println!("Polynomial regression");
    print_poly_regression(&a0, &a1, &a2);
    println!("r\t: {}", coorr_coef(&arr));
    println!("σ\t: {}", std_deviation(&arr));
    println!("R^2\t: {}", determination_coef(&arr, &a2, &a1, &a0, "poly"));
    println!("Se\t: {}", std_error(&arr, &a2, &a1, &a0, "poly"));
}

fn print_poly_regression(a0: &f64, a1: &f64, a2: &f64) {
    println!("y = {} + {}x + {}x^2", a0, a1, a2);
}

fn poly_reg(arr: &(Vec<f64>, Vec<f64>)) -> (f64, f64, f64) {
    let n = arr.0.len();
    let x = &arr.0;
    let y = &arr.1;

    let sx: f64 = x.iter().sum::<f64>();
    let sy: f64 = y.iter().sum::<f64>();
    let sx2 = x.iter().map(|x| x * x).sum::<f64>();
    let sx3 = x.iter().map(|x| x * x * x).sum::<f64>();
    let sx4 = x.iter().map(|x| x * x * x * x).sum::<f64>();
    let sxy = x.iter().zip(y.iter()).map(|(x, y)| x * y).sum::<f64>();
    let sx2y = x.iter().zip(y.iter()).map(|(x, y)| x * x * y).sum::<f64>();

    let matrix = [[n as f64, sx, sx2], [sx, sx2, sx3], [sx2, sx3, sx4]];
    let b = [sy, sxy, sx2y];
    let res = gauss_jordan(&matrix, &b);
    (res[0], res[1], res[2])
}

fn gauss_jordan(matrix: &[[f64; 3]; 3], b: &[f64; 3]) -> [f64; 3] {
    let mut a = matrix.clone();
    let mut x = b.clone();
    let n = a.len();
    for i in 0..n {
        let mut max = a[i][i];
        let mut max_index = i;
        for j in i + 1..n {
            if a[j][i] > max {
                max = a[j][i];
                max_index = j;
            }
        }
        if max_index != i {
            a.swap(i, max_index);
            x.swap(i, max_index);
        }
        for j in i + 1..n {
            let c = a[j][i] / a[i][i];
            for k in i..n {
                a[j][k] -= c * a[i][k];
            }
            x[j] -= c * x[i];
        }
    }
    for i in (0..n).rev() {
        for j in i + 1..n {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }
    x
}

fn linear() {

    let data = std::fs::read_to_string("data.txt").unwrap();
    let data: Vec<(f64, f64)> = data
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap() as f64;
            let y = parts.next().unwrap().parse::<f64>().unwrap();
            (x, y)
        })
        .collect();

    let x = data.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let y = data.iter().map(|(_, y)| *y).collect::<Vec<_>>();
    let arr = (x, y);

    let (a0, a1) = match arr.0.len() {
        0 => (0., 0.),
        1 => (arr.1[0], 0.),
        _ => linear_reg(&arr),
    };
    println!("Linear regression:");
    print_linear_regression(&a0, &a1);
    println!("r\t: {}", coorr_coef(&arr));
    println!("σ\t: {}", std_deviation(&arr));
    println!("R^2\t: {}", determination_coef(&arr, &0.0, &a1, &a0, "linear"));
    println!("Se\t: {}\n", std_error(&arr, &0.0, &a1, &a0, "linear"));
}

fn std_error(arr: &(Vec<f64>, Vec<f64>), a2: &f64, a1: &f64, a0: &f64, method: &str) -> f64 {
    let n = arr.0.len();
    let rss = arr.0.iter()
        .zip(arr.1.iter())
        .map(|(x, y)| (y - 
            (match method{
            "linear" => a1 * *x as f64 + a0,
            "poly" => a2 * *x as f64 * *x as f64 + a1 * *x as f64 + a0,
            _ => 0.,})
            ).powi(2))
        .sum::<f64>();
    (rss / n as f64).sqrt()
}

fn determination_coef(arr: &(Vec<f64>, Vec<f64>), a2: &f64, a1: &f64, a0: &f64, method: &str) -> f64 {
    let rss = arr.0.iter()
        .zip(arr.1.iter())
        .map(|(x, y)| (y - 
            (match method{
            "linear" => a1 * *x as f64 + a0,
            "poly" => a2 * *x as f64 * *x as f64 + a1 * *x as f64 + a0,
            _ => 0.,})
            ).powi(2))
        .sum::<f64>();
    let tss = (0..arr.0.len() as i32)
        .map(|x| (arr.1[x as usize] - mean(&arr.1)).powi(2))
        .sum::<f64>();
    1. - (rss / tss)
}

fn mean(arr: &Vec<f64>) -> f64 {
    arr.iter().sum::<f64>() / arr.len() as f64
}

fn coorr_coef(arr: &(Vec<f64>, Vec<f64>)) -> f64 {
    let mean = mean(&arr.1);
    let num = arr.0.iter()
        .zip(arr.1.iter())
        .map(|(x, y)| (*x as f64 - mean) * (y - mean))
        .sum::<f64>();
    let den = arr.0.iter()
        .map(|x| (*x as f64 - mean).powi(2))
        .sum::<f64>()
        * arr.1.iter().map(|x| (*x - mean).powi(2)).sum::<f64>();
    num / (den.sqrt())
}

fn print_linear_regression(a0: &f64, a1: &f64) {
    println!("y = {} + {} * x", a0, a1);
}

fn std_deviation(arr: &(Vec<f64>, Vec<f64>)) -> f64 {
    let n = arr.1.len();
    let mean = arr.1.iter().sum::<f64>() / n as f64;
    let sum = arr.1.iter().map(|x| (*x - mean).powi(2)).sum::<f64>();
    (sum / n as f64).sqrt()
}

fn linear_reg(arr: &(Vec<f64>, Vec<f64>)) -> (f64, f64){
    let n = arr.0.len();
    let x = arr.0.iter().sum::<f64>();
    let y = arr.1.iter().sum::<f64>();
    let x2 = arr.0.iter().map(|x| x.powi(2)).sum::<f64>();
    let xy = arr.0.iter().zip(arr.1.iter()).map(|(x, y)| x * y).sum::<f64>();
    let a1 = (n as f64 * xy - x * y) / (n as f64 * x2 - x.powi(2));
    let a0 = (y - a1 * x) / n as f64;
    (a0, a1)
}

