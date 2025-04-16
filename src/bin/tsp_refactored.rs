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

fn main(){
    let distance_matrix = vec![
        vec![0, 2, 9, 10, 5, 7],
        vec![1, 0, 6, 4, 1, 3],
        vec![15, 7, 0, 8, 9, 2],
        vec![6, 3, 12, 0, 8, 8],
        vec![10, 7, 10, 7, 0, 6],
        vec![2, 3, 6, 9, 7, 9, 0],
    ];

    let mut subproblemsolutions = HashMap::new();
    compute_base_case(&mut subproblemsolutions, &distance_matrix);

    compute_min_cost(&mut subproblemsolutions, &distance_matrix);


    let (final_cost, path) = return_to_start_and_backtrack(&subproblemsolutions, &distance_matrix);
    println!("PRINTING FROM MAIN");
    println!("Final PATH: {:?}", path);
    println!("FINAL COST: {}", final_cost);

}

fn compute_base_case(subproblemsolutions : &mut HashMap<(u64, usize), (i32, usize)>,dist: &Vec<Vec<i32>>,){
    let n = dist.len();
    for city in 1..n {
        let mask = 1 << city;
        let cost = dist[0][city];
        subproblemsolutions.insert((mask, city), (cost, 0));
        println!("Base case = {:0width$b}, last_city = {}, cost = {}", mask, city, cost, width = n );
        
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