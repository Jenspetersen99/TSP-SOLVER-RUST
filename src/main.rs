use std::collections::HashMap;
use itertools::Itertools;
use std::time::Instant;

const DEBUG: bool = false;

macro_rules! debug {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
        }
    };
}


fn main() {
    let start_time = Instant::now(); //  Start timer


    let distance_matrix = vec![
    vec![0, 65, 22, 28, 69, 90, 44, 84, 24, 24, 79, 59, 77, 62, 54, 12, 85, 17, 14, 82, 31, 65, 40, 42, 53],
    vec![65, 0, 32, 83, 83, 5, 18, 37, 35, 97, 54, 10, 6, 75, 59, 31, 20, 9, 88, 65, 51, 68, 42, 19, 25],
    vec![22, 32, 0, 12, 27, 8, 92, 41, 80, 99, 37, 86, 80, 92, 32, 33, 39, 77, 18, 13, 72, 74, 91, 9, 65],
    vec![28, 83, 12, 0, 32, 67, 31, 91, 84, 21, 71, 98, 22, 69, 7, 16, 41, 89, 90, 77, 38, 97, 62, 23, 37],
    vec![69, 83, 27, 32, 0, 25, 54, 59, 12, 69, 13, 14, 19, 42, 43, 93, 55, 13, 67, 96, 16, 27, 34, 76, 37],
    vec![90, 5, 8, 67, 25, 0, 19, 35, 26, 81, 43, 55, 29, 83, 37, 11, 35, 10, 70, 67, 65, 96, 20, 70, 67],
    vec![44, 18, 92, 31, 54, 19, 0, 45, 58, 12, 50, 31, 17, 32, 71, 86, 55, 9, 32, 86, 34, 36, 76, 51, 65],
    vec![84, 37, 41, 91, 59, 35, 45, 0, 76, 29, 19, 11, 41, 37, 31, 30, 51, 65, 6, 19, 75, 77, 56, 48, 26],
    vec![24, 35, 80, 84, 12, 26, 58, 76, 0, 24, 17, 67, 32, 9, 18, 54, 70, 49, 59, 16, 50, 13, 24, 78, 28],
    vec![24, 97, 99, 21, 69, 81, 12, 29, 24, 0, 76, 76, 97, 91, 17, 41, 35, 75, 26, 43, 45, 100, 6, 62, 7],
    vec![79, 54, 37, 71, 13, 43, 50, 19, 17, 76, 0, 49, 42, 57, 49, 51, 20, 48, 45, 25, 85, 16, 93, 79, 73],
    vec![59, 10, 86, 98, 14, 55, 31, 11, 67, 76, 49, 0, 80, 25, 64, 7, 12, 15, 7, 15, 48, 14, 31, 66, 80],
    vec![77, 6, 80, 22, 19, 29, 17, 41, 32, 97, 42, 80, 0, 99, 78, 35, 11, 80, 85, 78, 99, 38, 76, 59, 85],
    vec![62, 75, 92, 69, 42, 83, 32, 37, 9, 91, 57, 25, 99, 0, 13, 49, 32, 96, 92, 41, 73, 94, 73, 23, 92],
    vec![54, 59, 32, 7, 43, 37, 71, 31, 18, 17, 49, 64, 78, 13, 0, 60, 30, 27, 29, 100, 69, 9, 30, 73, 11],
    vec![12, 31, 33, 16, 93, 11, 86, 30, 54, 41, 51, 7, 35, 49, 60, 0, 80, 48, 23, 13, 59, 24, 38, 59, 5],
    vec![85, 20, 39, 41, 55, 35, 55, 51, 70, 35, 20, 12, 11, 32, 30, 80, 0, 88, 58, 25, 87, 81, 70, 100, 15],
    vec![17, 9, 77, 89, 13, 10, 9, 65, 49, 75, 48, 15, 80, 96, 27, 48, 88, 0, 6, 84, 61, 47, 9, 5, 74],
    vec![14, 88, 18, 90, 67, 70, 32, 6, 59, 26, 45, 7, 85, 92, 29, 23, 58, 6, 0, 64, 86, 54, 27, 43, 94],
    vec![82, 65, 13, 77, 96, 67, 86, 19, 16, 43, 25, 15, 78, 41, 100, 13, 25, 84, 64, 0, 47, 52, 23, 29, 14],
    vec![31, 51, 72, 38, 16, 65, 34, 75, 50, 45, 85, 48, 99, 73, 69, 59, 87, 61, 86, 47, 0, 85, 85, 45, 43],
    vec![65, 68, 74, 97, 27, 96, 36, 77, 13, 100, 16, 14, 38, 94, 9, 24, 81, 47, 54, 52, 85, 0, 39, 23, 28],
    vec![40, 42, 91, 62, 34, 20, 76, 56, 24, 6, 93, 31, 76, 73, 30, 38, 70, 9, 27, 23, 85, 39, 0, 60, 15],
    vec![42, 19, 9, 23, 76, 70, 51, 48, 78, 62, 79, 66, 59, 23, 73, 59, 100, 5, 43, 29, 45, 23, 60, 0, 49],
    vec![53, 25, 65, 37, 37, 67, 65, 26, 28, 7, 73, 80, 85, 92, 11, 5, 15, 74, 94, 14, 43, 28, 15, 49, 0],
];




    // Print distance_matrix
    for row in &distance_matrix {
        debug!("{:?}", row);
    }

    // Our DP table now stores (cost, prev_city)
    let mut subproblemsolutions: HashMap<(u64, usize), (i32, usize)> = HashMap::new();

    let n = distance_matrix.len();
    debug!("Number of cities: {}", n);

    // Base case
    for city in 1..n {
        let mask = 1 << city;
        let cost = distance_matrix[0][city];
        subproblemsolutions.insert((mask, city), (cost, 0));
        debug!("Base case = {:04b}, last_city = {}, cost = {}", mask, city, cost );
    }

    // DP table construction
    for subset_size in 2..n {
        for combo in (1..n).combinations(subset_size) {
            debug!("\n--- New Subset ---");
            debug!("Subset: {:?}", combo);

            let mask = combo.iter().fold(0u64, |acc, &c| acc | (1 << c));
            debug!("Subset as bitmask = {:04b}", mask);

            for &last_city in &combo {
                let prev_mask = mask & !(1 << last_city);
                debug!("\n  Considering last_city = {}", last_city);
                debug!("  prev_mask (subset without last_city) = {:04b}", prev_mask);

                let mut min_cost = i32::MAX;
                let mut best_prev_city = 0;

                for &prev_city in &combo {
                    if prev_city == last_city {
                        continue;
                    }

                    debug!("    Trying prev_city = {}", prev_city);

                    if let Some(&(cost_to_prev, _)) = subproblemsolutions.get(&(prev_mask, prev_city)) {
                        debug!(
                            "    Found dp[({:04b}, {})] = {}",
                            prev_mask, prev_city, cost_to_prev
                        );
                        debug!(
                            "    Cost from {} to {} = {}",
                            prev_city, last_city, distance_matrix[prev_city][last_city]
                        );

                        let total_cost = cost_to_prev + distance_matrix[prev_city][last_city];
                        debug!("    Total cost to reach {} = {}", last_city, total_cost);

                        if total_cost < min_cost {
                            min_cost = total_cost;
                            best_prev_city = prev_city;
                        }
                    } else {
                        debug!(
                            "    dp[({:04b}, {})] not found. Skipping.",
                            prev_mask, prev_city
                        );
                    }
                }

                debug!(
                    " Insert: dp[({:04b}, {})] = {}",
                    mask, last_city, min_cost
                );
                subproblemsolutions.insert((mask, last_city), (min_cost, best_prev_city));
            }
        }
    }

    // FINAL STEP: Close the tour
    debug!("\n=== Final Step: Return to Start ===");

    let full_mask = (1 << n) - 2; // all cities except 0
    let mut min_total_cost = i32::MAX;
    let mut final_last_city = 0;

    for last_city in 1..n {
        if let Some(&(cost, _)) = subproblemsolutions.get(&(full_mask, last_city)) {
            let total_cost = cost + distance_matrix[last_city][0];
            debug!(
                "Cost of tour ending at city {} and returning to 0: {} + {} = {}",
                last_city, cost, distance_matrix[last_city][0], total_cost
            );

            if total_cost < min_total_cost {
                min_total_cost = total_cost;
                final_last_city = last_city;
            }
        }
    }

    debug!("\n Shortest complete tour cost = {}", min_total_cost);

    // PATH RECONSTRUCTION
    debug!("\n=== Reconstructing Path (Step-by-Step) ===");

let mut path = vec![0]; // Start city
let mut mask = full_mask;
let mut last_city = final_last_city;

debug!("Starting backtrace:");
debug!("  Initial mask = {:04b} (visited all cities)", mask);
debug!("  Starting from last_city = {}", last_city);

path.push(last_city);

while mask != 0 {
    let &(_, prev_city) = subproblemsolutions.get(&(mask, last_city)).unwrap();

    debug!(
        "\n--- Step ---\nCurrent mask: {:04b}\nLast city: {}\nPrevious city: {}",
        mask, last_city, prev_city
    );

    // Show mask shrink visually
    let new_mask = mask & !(1 << last_city);
    debug!("Removing last_city {} gives new mask: {:04b}", last_city, new_mask);

    if prev_city == 0 {
        debug!("Found start city (0), ending backtrace.");
        break;
    }

    path.push(prev_city);
    mask = new_mask;
    last_city = prev_city;
}

path.push(0); // return to start
path.reverse();
let elapsed = start_time.elapsed(); //  Stop timer
println!("Execution Time: {:.3?}", elapsed);


println!("\n Reconstructed Path: {:?}", path);

}
