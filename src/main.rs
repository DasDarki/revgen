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

fn optimize_distribution(target_average: f64, total_reviews: f64) -> (Vec<f64>, bool) {
    let mut distribution = vec![0.0; 5];
    let mut step_size = 0.01;
    let mut is_approximate = false;
    let mut iterations = 0;
    let max_iterations_before_approximation = 1000000; // Nach 1000 Iterationen erlauben wir eine größere Fehlermarge

    for i in 0..5 {
        distribution[i] = total_reviews / 5.0;
    }

    loop {
        let current_average = calculate_average(&distribution);
        let error = target_average - current_average;

        if error.abs() < 0.01 {
            break;
        } else if iterations > max_iterations_before_approximation && error.abs() < 0.05 {
            is_approximate = true;
            break;
        }

        if error > 0.0 {
            for i in (1..5).rev() {
                distribution[i] += step_size * total_reviews;
                distribution[i - 1] -= step_size * total_reviews;
                if distribution[i - 1] < 0.0 {
                    distribution[i - 1] = 0.0;
                }
            }
        } else {
            for i in 0..4 {
                distribution[i] += step_size * total_reviews;
                if i < 4 {
                    distribution[i + 1] -= step_size * total_reviews;
                    if distribution[i + 1] < 0.0 {
                        distribution[i + 1] = 0.0;
                    }
                }
            }
        }

        normalize_distribution(&mut distribution, total_reviews);

        iterations += 1;
    }

    (distribution, is_approximate)
}

fn normalize_distribution(distribution: &mut [f64], total_reviews: f64) {
    let current_total: f64 = distribution.iter().sum();
    for value in distribution.iter_mut() {
        *value *= total_reviews / current_total;
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