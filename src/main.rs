use clap::{App, Arg};

fn main() {
    let matches = App::new("Ratings Optimizer")
        .version("1.0")
        .author("DasDarki")
        .about("Optimizes ratings distribution to meet a target average.")
        .arg(Arg::with_name("TARGET_AVERAGE")
            .help("The target average rating (between 1.0 and 5.0)")
            .required(true)
            .index(1))
        .arg(Arg::with_name("TOTAL_REVIEWS")
            .help("The total number of reviews")
            .required(true)
            .index(2))
        .arg(Arg::with_name("TRY_REALISM")
            .help("Attempts to create a more realistic distribution")
            .long("try-realism")
            .takes_value(false))
        .get_matches();

    let target_average: f64 = matches.value_of("TARGET_AVERAGE").unwrap().parse().expect("Target average must be a float");
    if target_average < 1.0 || target_average > 5.0 {
        eprintln!("Error: Target average must be between 1.0 and 5.0");
        return;
    }
    let total_reviews: f64 = matches.value_of("TOTAL_REVIEWS").unwrap().parse().expect("Total reviews must be a float");
    let try_realism = matches.is_present("TRY_REALISM");

    let (distribution, is_approximate) = optimize_distribution(target_average, total_reviews, try_realism);
    println!("Optimized distribution:");
    println!("1 Star: {:.0}", distribution[0]);
    println!("2 Stars: {:.0}", distribution[1]);
    println!("3 Stars: {:.0}", distribution[2]);
    println!("4 Stars: {:.0}", distribution[3]);
    println!("5 Stars: {:.0}", distribution[4]);

    let actual_average = calculate_average(&distribution);
    println!("Total reviews: {:.0}", calculate_totals(&distribution));
    println!("Actual average: {:.2}", actual_average);
    if is_approximate {
        println!("Note: The result is an approximation to the target average.");
    }

    println!("In JavaScript form:");
    println!("{{");
    println!("  1: {:.0},", distribution[0]);
    println!("  2: {:.0},", distribution[1]);
    println!("  3: {:.0},", distribution[2]);
    println!("  4: {:.0},", distribution[3]);
    println!("  5: {:.0}", distribution[4]);
    println!("}},");
}

fn optimize_distribution(target_average: f64, total_reviews: f64, try_realism: bool) -> (Vec<f64>, bool) {
    let mut distribution = vec![0.0; 5];
    let mut iterations = 0;
    let max_iterations_before_approximation = 1000;
    let mut is_approximate = false;

    if try_realism {
        let weights = match target_average {
            avg if avg > 4.5 => vec![0.05, 0.05, 0.10, 0.20, 0.60],
            avg if avg > 4.0 => vec![0.10, 0.10, 0.15, 0.25, 0.40],
            avg if avg > 3.5 => vec![0.15, 0.20, 0.25, 0.25, 0.15],
            avg if avg > 3.0 => vec![0.20, 0.25, 0.25, 0.20, 0.10],
            _ => vec![0.30, 0.25, 0.20, 0.15, 0.10],
        };

        for (i, weight) in weights.iter().enumerate() {
            distribution[i] = total_reviews * weight;
        }
    } else {
        for value in distribution.iter_mut() {
            *value = total_reviews / 5.0;
        }
    }

    loop {
        let current_average = calculate_average(&distribution);
        let error = target_average - current_average;

        if error.abs() < 0.01 || iterations > max_iterations_before_approximation {
            break;
        }

        adjust_distribution(&mut distribution, error, total_reviews);

        iterations += 1;
        if iterations > max_iterations_before_approximation {
            is_approximate = true;
            break;
        }
    }

    (distribution, is_approximate)
}

fn adjust_distribution(distribution: &mut Vec<f64>, error: f64, total_reviews: f64) {
    let step_size = 0.001 * total_reviews;

    if error > 0.0 {
        for i in 0..4 {
            if distribution[i] > step_size {
                distribution[i] -= step_size;
                distribution[i + 1] += step_size;
            }
        }
    } else {
        for i in (1..5).rev() {
            if distribution[i] > step_size {
                distribution[i] -= step_size;
                distribution[i - 1] += step_size;
            }
        }
    }
}

fn calculate_average(distribution: &[f64]) -> f64 {
    let total = distribution.iter().enumerate().fold(0.0, |acc, (i, &value)| acc + value * (i as f64 + 1.0));
    let total_reviews: f64 = distribution.iter().sum();
    total / total_reviews
}

fn calculate_totals(distribution: &[f64]) -> f64 {
    distribution.iter().sum()
}