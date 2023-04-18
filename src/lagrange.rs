fn main() {
    let data = std::fs::read_to_string("C:data.txt").unwrap();
    let data: Vec<(f64, f64)> = data
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap() as f64;
            let y = parts.next().unwrap().parse::<f64>().unwrap();
            (x, y)
        })
        .collect();

    let ogx = data.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let ogy = data.iter().map(|(_, y)| *y).collect::<Vec<_>>();

    let x = ogx
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, x)| *x)
        .collect::<Vec<_>>();
    let y = ogy
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, y)| *y)
        .collect::<Vec<_>>();
    let mut md_table = String::new();
    md_table.push_str(&format!("|Grado|{}|   |   |\r|---|---|---|---|\r", 1));
    for n in 1..=8 {
        let mut errs = Vec::<f64>::new();
        if n != 1 {
            md_table.push_str(&format!("|Grado|{}|   |   |\r", n));
        }
        md_table.push_str("|X|Y|Y lagrange|Err %|\r");
        for xii in (10..500).step_by(20) {
            let xi = xii as f64;

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
            let err = (ogy[ogx.iter().position(|&x| x == xi).unwrap()] - y1).abs();
            errs.push(err);
            md_table.push_str(&format!(
                "|{}|{:.4}|{:.4}|{:.4}|\r",
                xi,
                ogy[ogx.iter().position(|&x| x == xi).unwrap()],
                y1,
                err * 100.0
            ));
        }
        md_table.push_str(&format!(
            "|Err promedio|{:.4}|\r",
            (errs.iter().sum::<f64>() / errs.len() as f64) * 100.0
        ));
    }
    std::fs::write("lagrange.md", md_table).unwrap();
    println!("Tabla guardada en lagrange.md");
}
