use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let distance_matrix = vec![
    vec![0, 43, 22, 71, 36, 65, 54, 27],
    vec![43, 0, 31, 62, 49, 78, 41, 55],
    vec![22, 31, 0, 40, 34, 66, 59, 29],
    vec![71, 62, 40, 0, 58, 25, 37, 50],
    vec![36, 49, 34, 58, 0, 5, 44, 50],
    vec![65, 78, 66, 25, 61, 0, 48, 53],
    vec![54, 41, 59, 37, 44, 48, 0, 39],
    vec![27, 55, 29, 50, 31, 53, 39, 0],
];



    let (cost_dp, path_dp) = held_karp(distance_matrix.clone());
    let (cost_bf, path_bf) = brute_force_tsp(&distance_matrix);

    println!("📦 Held-Karp:   cost = {}, path = {:?}", cost_dp, path_dp);
    println!("🦥 Brute Force: cost = {}, path = {:?}", cost_bf, path_bf);

    if cost_dp == cost_bf {
        println!("✅ Held-Karp gives the optimal solution!");
    } else {
        println!("❌ Held-Karp did NOT return optimal solution!");
    }
}

fn held_karp(distance_matrix: Vec<Vec<i32>>) -> (i32, Vec<usize>) {
    let n = distance_matrix.len();
    let mut subproblemsolutions = HashMap::new();

    // Base case
    for city in 1..n {
        let mask = 1 << city;
        let cost = distance_matrix[0][city];
        subproblemsolutions.insert((mask, city), (cost, 0));
    }

    // DP computation
    for subset_size in 2..n {
        for combo in (1..n).combinations(subset_size) {
            let mask = combo.iter().fold(0u64, |acc, &c| acc | (1 << c));
            for &last_city in &combo {
                let prev_mask = mask & !(1 << last_city);
                let mut min_cost = i32::MAX;
                let mut best_prev = 0;

                for &prev_city in &combo {
                    if prev_city == last_city {
                        continue;
                    }
                    if let Some(&(prev_cost, _)) = subproblemsolutions.get(&(prev_mask, prev_city)) {
                        let total = prev_cost + distance_matrix[prev_city][last_city];
                        if total < min_cost {
                            min_cost = total;
                            best_prev = prev_city;
                        }
                    }
                }

                subproblemsolutions.insert((mask, last_city), (min_cost, best_prev));
            }
        }
    }

    // Return to start
    let full_mask = (1 << n) - 2;
    let mut min_total_cost = i32::MAX;
    let mut final_city = 0;

    for city in 1..n {
        if let Some(&(cost, _)) = subproblemsolutions.get(&(full_mask, city)) {
            let total = cost + distance_matrix[city][0];
            if total < min_total_cost {
                min_total_cost = total;
                final_city = city;
            }
        }
    }

    // Reconstruct path
    let mut path = vec![0];
    let mut mask = full_mask;
    let mut last = final_city;
    path.push(last);

    while mask != 0 {
        let &(_, prev) = subproblemsolutions.get(&(mask, last)).unwrap();
        if prev == 0 {
            break;
        }
        path.push(prev);
        mask &= !(1 << last);
        last = prev;
    }

    path.push(0);
    path.reverse();

    (min_total_cost, path)
}

fn brute_force_tsp(dist: &Vec<Vec<i32>>) -> (i32, Vec<usize>) {
    let n = dist.len();
    let mut min_cost = i32::MAX;
    let mut best_path = Vec::new();

    for perm in (1..n).permutations(n - 1) {
        let mut path = vec![0];
        path.extend(&perm);
        path.push(0);

        let mut cost = 0;
        for i in 0..path.len() - 1 {
            cost += dist[path[i]][path[i + 1]];
        }

        if cost < min_cost {
            min_cost = cost;
            best_path = path;
        }
    }

    (min_cost, best_path)
}
