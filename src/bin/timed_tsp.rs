use std::collections::HashMap;
use itertools::Itertools;

const DEBUG: bool = false;

macro_rules! debug {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
        }
    };
}

use std::time::Instant;

fn main() {
    let distance_matrix = vec![
    vec![0, 29, 20, 21, 16, 31, 100, 12, 4, 31, 18, 25, 14, 8, 19, 28, 11, 33, 45, 9],
    vec![29, 0, 15, 29, 28, 40, 72, 21, 29, 41, 12, 17, 35, 23, 26, 37, 22, 44, 55, 20],
    vec![20, 15, 0, 15, 14, 25, 81, 9, 23, 27, 13, 16, 10, 7, 20, 22, 17, 31, 40, 11],
    vec![21, 29, 15, 0, 4, 12, 92, 12, 25, 13, 25, 22, 18, 9, 24, 19, 20, 34, 39, 17],
    vec![16, 28, 14, 4, 0, 16, 94, 9, 20, 16, 22, 21, 12, 5, 23, 24, 19, 30, 42, 15],
    vec![31, 40, 25, 12, 16, 0, 95, 24, 36, 3, 37, 38, 28, 19, 30, 27, 21, 33, 48, 18],
    vec![100, 72, 81, 92, 94, 95, 0, 90, 101, 99, 84, 83, 91, 79, 87, 93, 85, 73, 60, 88],
    vec![12, 21, 9, 12, 9, 24, 90, 0, 15, 25, 13, 19, 8, 6, 18, 21, 10, 26, 35, 14],
    vec![4, 29, 23, 25, 20, 36, 101, 15, 0, 35, 18, 20, 13, 10, 22, 30, 9, 27, 37, 12],
    vec![31, 41, 27, 13, 16, 3, 99, 25, 35, 0, 30, 35, 26, 21, 33, 20, 24, 39, 50, 19],
    vec![18, 12, 13, 25, 22, 37, 84, 13, 18, 30, 0, 10, 17, 14, 19, 28, 15, 32, 43, 16],
    vec![25, 17, 16, 22, 21, 38, 83, 19, 20, 35, 10, 0, 16, 11, 23, 26, 18, 29, 41, 13],
    vec![14, 35, 10, 18, 12, 28, 91, 8, 13, 26, 17, 16, 0, 5, 14, 20, 13, 24, 34, 11],
    vec![8, 23, 7, 9, 5, 19, 79, 6, 10, 21, 14, 11, 5, 0, 12, 18, 9, 22, 30, 7],
    vec![19, 26, 20, 24, 23, 30, 87, 18, 22, 33, 19, 23, 14, 12, 0, 25, 17, 29, 38, 15],
    vec![28, 37, 22, 19, 24, 27, 93, 21, 30, 20, 28, 26, 20, 18, 25, 0, 22, 35, 47, 16],
    vec![11, 22, 17, 20, 19, 21, 85, 10, 9, 24, 15, 18, 13, 9, 17, 22, 0, 27, 36, 10],
    vec![33, 44, 31, 34, 30, 33, 73, 26, 27, 39, 32, 29, 24, 22, 29, 35, 27, 0, 28, 19],
    vec![45, 55, 40, 39, 42, 48, 60, 35, 37, 50, 43, 41, 34, 30, 38, 47, 36, 28, 0, 31],
    vec![9, 20, 11, 17, 15, 18, 88, 14, 12, 19, 16, 13, 11, 7, 15, 16, 10, 19, 31, 0],
];

    let start_time = Instant::now(); //  Start timer

    let mut subproblemsolutions = std::collections::HashMap::new();
    compute_base_case(&mut subproblemsolutions, &distance_matrix);
    compute_min_cost(&mut subproblemsolutions, &distance_matrix);
    let (final_cost, path) = return_to_start_and_backtrack(&subproblemsolutions, &distance_matrix);

    let elapsed = start_time.elapsed(); //  Stop timer

    println!("PRINTING FROM MAIN");
    println!("Final PATH: {:?}", path);
    println!("FINAL COST: {}", final_cost);
    println!("Execution Time: {:.3?}", elapsed);
}

fn compute_base_case(subproblemsolutions : &mut HashMap<(u64, usize), (i32, usize)>,dist: &Vec<Vec<i32>>,){
    let n = dist.len();
    for city in 1..n {
        let mask = 1 << city;
        let cost = dist[0][city];
        subproblemsolutions.insert((mask, city), (cost, 0));
        debug!("Base case = {:0width$b}, last_city = {}, cost = {}", mask, city, cost, width = n );
        
    }

}

fn compute_min_cost(subproblemsolutions : &mut HashMap<(u64, usize), (i32, usize)>, dist: &Vec<Vec<i32>>){
    let n = dist.len();

    for subset_size in 2..n {
        for combo in (1..n).combinations(subset_size) {
            debug!("\n--- New Subset ---");
            debug!("Subset: {:?}", combo);
            // fold is basically a for each loop
            let mask = combo.iter().fold(0u64, |acc, &c| acc | (1 << c));
            debug!("Subset as bitmask = {:0width$b}", mask, width = n);

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
                            prev_city, last_city, dist[prev_city][last_city]
                        );

                        let total_cost = cost_to_prev + dist[prev_city][last_city];
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
}

fn return_to_start_and_backtrack(subproblemsolutions : &HashMap<(u64, usize), (i32, usize)>, dist: &Vec<Vec<i32>>)
-> (i32, Vec<usize>){
    // FINAL STEP: Close the tour
    let n = dist.len();
    debug!("\n=== Final Step: Return to Start ===");

    let full_mask = (1 << n) - 2; // all cities except 0
    let mut min_total_cost = i32::MAX;
    let mut final_last_city = 0;

    for last_city in 1..n {
        if let Some(&(cost, _)) = subproblemsolutions.get(&(full_mask, last_city)) {
            let total_cost = cost + dist[last_city][0];
            debug!(
                "Cost of tour ending at city {} and returning to 0: {} + {} = {}",
                last_city, cost, dist[last_city][0], total_cost
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

    println!("\n Reconstructed Path from function: {:?}", path);
    (min_total_cost, path)


    }