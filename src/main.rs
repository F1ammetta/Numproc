use plotters::prelude::*;

fn main() {
    let data = std::fs::read_to_string("C:\\Users\\Sergio\\numproc\\data.txt").unwrap();
    // data lines schema: x, y\n
    let data: Vec<(usize, f64)> = data
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap();
            let y = parts.next().unwrap().parse::<f64>().unwrap();
            (x, y)
        })
        .collect();

    // ignore data var
    let x = data.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let y = data.iter().map(|(_, y)| *y).collect::<Vec<_>>();

    let mut y_max = 0.0;

    for i in 0..y.len() {
        if y[i] > y_max {
            y_max = y[i];
        }
    }

    let w: Vec<f64> = (0..*x.last().unwrap())
        .into_iter()
        .map(|x| x as f64)
        .collect::<Vec<_>>();
    let z: Vec<f64> = w
        .iter()
        .map(|&x| {
            0.3024847379857066 - 0.0034042211761172937 * x + 0.000022216248306630203 * (x.powi(2))
        })
        .collect::<Vec<_>>();
    let v: Vec<f64> = w
        .iter()
        .map(|&x| -0.1715159067058829 + 0.0061762822934934 * x)
        .collect::<Vec<_>>();

    let latex_table = format!(
        "\\hline\rNúmero de clientes simultaneos & Tiempo de respuesta (s)\\\\\r\\hline\r{}\\\\\r\\hline\r",
        data.iter()
            .map(|(x, y)| format!("{} & {}", x, y))
            .collect::<Vec<_>>()
            .join("\\\\\r")
    );

    std::fs::write("C:\\Users\\Sergio\\numproc\\table.tex", latex_table).unwrap();

    let root = BitMapBackend::new("C:\\Users\\Sergio\\numproc\\plotss.png", (1360, 1020))
        .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Time vs Clients",
            ("times new roman", 50).into_font().color(&BLACK),
        )
        .margin(5)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(0..(*x.last().unwrap()) as i32, 0.0..y_max * 1.1)
        .unwrap();

    chart
        .configure_mesh()
        .axis_style(&BLACK)
        .label_style(("times new roman", 20).into_font().color(&BLACK))
        .light_line_style(&TRANSPARENT)
        .bold_line_style(&BLACK.mix(0.6))
        .x_desc("Number of concurent clients")
        .y_desc("Time (s)")
        .axis_desc_style(("times new roman", 30).into_font().color(&BLACK))
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            x.iter().zip(y.iter()).map(|(&x, &y)| (x as i32, y)),
            Into::<ShapeStyle>::into(&BLUE).stroke_width(2),
        ))
        .unwrap()
        .label("Datos obtenidos")
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 20, y)],
                Into::<ShapeStyle>::into(&BLUE).stroke_width(3),
            )
        });

    chart
        .draw_series(LineSeries::new(
            w.iter().zip(v.iter()).map(|(&x, &y)| (x as i32, y)),
            Into::<ShapeStyle>::into(&full_palette::GREEN).stroke_width(2),
        ))
        .unwrap()
        .label("Regresión lineal ( y = 0.006176x - 0.17151 )")
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 20, y)],
                Into::<ShapeStyle>::into(&full_palette::GREEN).stroke_width(3),
            )
        });

    chart
        .draw_series(LineSeries::new(
            w.iter().zip(z.iter()).map(|(&x, &y)| (x as i32, y)),
            Into::<ShapeStyle>::into(&RED).stroke_width(2),
        ))
        .unwrap()
        .label("Regresión polinomial ( y = 0.000022x^2 - 0.0034x + 0.3025 )")
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 20, y)],
                Into::<ShapeStyle>::into(&RED).stroke_width(3),
            )
        });

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE)
        .label_font(("times new roman", 30).into_font())
        .position(SeriesLabelPosition::MiddleLeft)
        .draw()
        .unwrap();

    root.present().unwrap();
}
