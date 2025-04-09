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

    // let (final_cost, last_city) = compute_return_to_start_city(&subproblemsolutions, &distance_matrix);

    // let path = reconstruct_path(&dp, &dist, last_city);

    // println!("FINAL COST: {}", final_cost);
    // println!("Path: {:?}", path);

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

// fn compute_return_to_start_city(){
// }
// fn reconstruct_path(){
    
// }