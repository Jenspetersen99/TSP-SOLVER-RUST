import itertools
import time

def held_karp_simple_with_path(dist_matrix):
    n = len(dist_matrix)
    C = {}

    for k in range(1, n):
        C[(1 << k, k)] = (dist_matrix[0][k], 0)

    for subset_size in range(2, n):
        for subset in itertools.combinations(range(1, n), subset_size):
            bits = sum(1 << bit for bit in subset)
            for k in subset:
                prev_bits = bits & ~(1 << k)
                min_cost, prev_city = min(
                    (C[(prev_bits, m)][0] + dist_matrix[m][k], m)
                    for m in subset if m != k
                )
                C[(bits, k)] = (min_cost, prev_city)

    bits = (1 << n) - 2
    opt, parent = min((C[(bits, k)][0] + dist_matrix[k][0], k) for k in range(1, n))

    path = [0]
    last = parent
    mask = bits
    for _ in range(n - 1):
        path.append(last)
        _, prev = C[(mask, last)]
        mask &= ~(1 << last)
        last = prev
    path.append(0)
    path.reverse()

    return opt, path


def held_karp_backtrack(dist_matrix):
    n = len(dist_matrix)
    dp = {}

    for city in range(1, n):
        mask = 1 << city
        dp[(mask, city)] = (dist_matrix[0][city], 0)

    for subset_size in range(2, n):
        for combo in itertools.combinations(range(1, n), subset_size):
            mask = sum(1 << c for c in combo)
            for last_city in combo:
                prev_mask = mask & ~(1 << last_city)
                min_cost = float('inf')
                best_prev_city = 0
                for prev_city in combo:
                    if prev_city == last_city:
                        continue
                    if (prev_mask, prev_city) in dp:
                        cost_to_prev, _ = dp[(prev_mask, prev_city)]
                        total_cost = cost_to_prev + dist_matrix[prev_city][last_city]
                        if total_cost < min_cost:
                            min_cost = total_cost
                            best_prev_city = prev_city
                dp[(mask, last_city)] = (min_cost, best_prev_city)

    full_mask = (1 << n) - 2
    min_total_cost = float('inf')
    final_last_city = 0
    for last_city in range(1, n):
        if (full_mask, last_city) in dp:
            cost, _ = dp[(full_mask, last_city)]
            total_cost = cost + dist_matrix[last_city][0]
            if total_cost < min_total_cost:
                min_total_cost = total_cost
                final_last_city = last_city

    path = [0]
    mask = full_mask
    last_city = final_last_city
    path.append(last_city)
    while mask:
        _, prev_city = dp[(mask, last_city)]
        mask &= ~(1 << last_city)
        if prev_city == 0:
            break
        path.append(prev_city)
        last_city = prev_city
    path.append(0)
    path.reverse()

    return min_total_cost, path


# Re-load distance matrix
distance_matrix = [
    [0, 29, 20, 21, 16, 31, 100, 12, 4, 31, 18, 25, 14, 8, 19, 28, 11, 33, 45, 9, 75],
    [29, 0, 15, 29, 28, 40, 72, 21, 29, 41, 12, 17, 35, 23, 26, 37, 22, 44, 55, 20, 47],
    [20, 15, 0, 15, 14, 25, 81, 9, 23, 27, 13, 16, 10, 7, 20, 22, 17, 31, 40, 11, 75],
    [21, 29, 15, 0, 4, 12, 92, 12, 25, 13, 25, 22, 18, 9, 24, 19, 20, 34, 39, 17, 67],
    [16, 28, 14, 4, 0, 16, 94, 9, 20, 16, 22, 21, 12, 5, 23, 24, 19, 30, 42, 15, 55],
    [31, 40, 25, 12, 16, 0, 95, 24, 36, 3, 37, 38, 28, 19, 30, 27, 21, 33, 48, 18, 87],
    [100, 72, 81, 92, 94, 95, 0, 90, 101, 99, 84, 83, 91, 79, 87, 93, 85, 73, 60, 88, 45],
    [12, 21, 9, 12, 9, 24, 90, 0, 15, 25, 13, 19, 8, 6, 18, 21, 10, 26, 35, 14, 43],
    [4, 29, 23, 25, 20, 36, 101, 15, 0, 35, 18, 20, 13, 10, 22, 30, 9, 27, 37, 12, 59],
    [31, 41, 27, 13, 16, 3, 99, 25, 35, 0, 30, 35, 26, 21, 33, 20, 24, 39, 50, 19, 23],
    [18, 12, 13, 25, 22, 37, 84, 13, 18, 30, 0, 10, 17, 14, 19, 28, 15, 32, 43, 16, 14],
    [25, 17, 16, 22, 21, 38, 83, 19, 20, 35, 10, 0, 16, 11, 23, 26, 18, 29, 41, 13, 90],
    [14, 35, 10, 18, 12, 28, 91, 8, 13, 26, 17, 16, 0, 5, 14, 20, 13, 24, 34, 11, 29],
    [8, 23, 7, 9, 5, 19, 79, 6, 10, 21, 14, 11, 5, 0, 12, 18, 9, 22, 30, 7, 29],
    [19, 26, 20, 24, 23, 30, 87, 18, 22, 33, 19, 23, 14, 12, 0, 25, 17, 29, 38, 15, 92],
    [28, 37, 22, 19, 24, 27, 93, 21, 30, 20, 28, 26, 20, 18, 25, 0, 22, 35, 47, 16, 70],
    [11, 22, 17, 20, 19, 21, 85, 10, 9, 24, 15, 18, 13, 9, 17, 22, 0, 27, 36, 10, 32],
    [33, 44, 31, 34, 30, 33, 73, 26, 27, 39, 32, 29, 24, 22, 29, 35, 27, 0, 28, 19, 86],
    [45, 55, 40, 39, 42, 48, 60, 35, 37, 50, 43, 41, 34, 30, 38, 47, 36, 28, 0, 31, 95],
    [9, 20, 11, 17, 15, 18, 88, 14, 12, 19, 16, 13, 11, 7, 15, 16, 10, 19, 31, 0, 27],
    [75, 47, 75, 67, 55, 87, 45, 43, 59, 23, 14, 90, 29, 29, 92, 70, 32, 86, 95, 27, 0]
]

if __name__ == "__main__":
    import time

    print("Running Held-Karp (simple)...")
    start_time_1 = time.perf_counter()
    cost_simple = held_karp_simple_with_path(distance_matrix)
    end_time_1 = time.perf_counter()

    print(f"Simple Held-Karp cost: {cost_simple}")
    print(f"Simple Held-Karp time: {end_time_1 - start_time_1:.4f} seconds\n")

    print("Running Held-Karp (with backtrack)...")
    start_time_2 = time.perf_counter()
    cost_backtrack = held_karp_backtrack(distance_matrix)
    end_time_2 = time.perf_counter()

    print(f"Backtrack Held-Karp cost: {cost_backtrack}")
    print(f"Backtrack Held-Karp time: {end_time_2 - start_time_2:.4f} seconds")