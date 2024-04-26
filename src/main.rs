use clap::{App, Arg};

fn main() {
    let matches = App::new("Ratings Optimizer")
        .version("1.0")
        .author("Your Name")
        .about("Optimizes ratings distribution to meet a target average.")
        .arg(Arg::with_name("TARGET_AVERAGE")
            .help("The target average rating")
            .required(true)
            .index(1))
        .arg(Arg::with_name("TOTAL_REVIEWS")
            .help("The total number of reviews")
            .required(true)
            .index(2))
        .get_matches();

    let target_average: f64 = matches.value_of("TARGET_AVERAGE").unwrap().parse().expect("Target average must be a float");
    let total_reviews: f64 = matches.value_of("TOTAL_REVIEWS").unwrap().parse().expect("Total reviews must be a float");

    let distribution = optimize_distribution(target_average, total_reviews);
    println!("Optimized distribution:");
    println!("1 Star: {}", distribution[0]);
    println!("2 Stars: {}", distribution[1]);
    println!("3 Stars: {}", distribution[2]);
    println!("4 Stars: {}", distribution[3]);
    println!("5 Stars: {}", distribution[4]);

    let actual_average = calculate_average(&distribution);
    println!("Actual average: {}", actual_average);

    println!("In JavaScript form:");

    println!("{{");
    println!("  1: {},", distribution[0]);
    println!("  2: {},", distribution[1]);
    println!("  3: {},", distribution[2]);
    println!("  4: {},", distribution[3]);
    println!("  5: {}", distribution[4]);
    println!("}};");
}

fn optimize_distribution(target_average: f64, total_reviews: f64) -> Vec<f64> {
    let mut distribution = vec![0.0; 5];
    let mut step_size = 0.01;

    for i in 0..5 {
        distribution[i] = total_reviews / 5.0;
    }

    loop {
        let current_average = calculate_average(&distribution);
        let error = target_average - current_average;

        if error.abs() < 0.01 {
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
    }

    distribution
}

fn calculate_average(distribution: &[f64]) -> f64 {
    let total = distribution.iter().enumerate().fold(0.0, |acc, (i, &value)| acc + value * (i as f64 + 1.0));
    let total_reviews: f64 = distribution.iter().sum();
    total / total_reviews
}
