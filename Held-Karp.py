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
    [0, 15, 27, 38, 50, 61, 66, 71, 33, 45, 53, 59, 26, 34, 48, 55, 69, 76, 40, 63, 78, 58, 42, 54, 37],
    [15, 0, 17, 30, 41, 50, 57, 65, 25, 36, 44, 50, 22, 29, 41, 48, 62, 69, 35, 59, 73, 54, 36, 48, 32],
    [27, 17, 0, 14, 28, 39, 45, 54, 23, 31, 39, 45, 20, 25, 37, 43, 58, 65, 30, 53, 67, 48, 31, 43, 28],
    [38, 30, 14, 0, 18, 31, 39, 49, 26, 29, 35, 41, 23, 26, 34, 39, 54, 61, 33, 49, 63, 45, 29, 39, 27],
    [50, 41, 28, 18, 0, 16, 26, 39, 30, 33, 39, 45, 29, 27, 30, 36, 49, 56, 36, 45, 59, 41, 33, 37, 30],
    [61, 50, 39, 31, 16, 0, 14, 28, 38, 41, 47, 51, 36, 34, 30, 35, 47, 54, 44, 43, 54, 38, 35, 35, 29],
    [66, 57, 45, 39, 26, 14, 0, 17, 44, 47, 51, 55, 41, 38, 33, 38, 46, 51, 48, 41, 49, 34, 37, 32, 28],
    [71, 65, 54, 49, 39, 28, 17, 0, 50, 53, 55, 58, 48, 45, 38, 42, 44, 48, 55, 39, 43, 30, 41, 31, 29],
    [33, 25, 23, 26, 30, 38, 44, 50, 0, 13, 21, 29, 17, 21, 30, 36, 49, 56, 20, 51, 67, 47, 32, 41, 25],
    [45, 36, 31, 29, 33, 41, 47, 53, 13, 0, 12, 21, 25, 27, 28, 33, 46, 55, 23, 48, 63, 44, 31, 36, 27],
    [53, 44, 39, 35, 39, 47, 51, 55, 21, 12, 0, 11, 32, 32, 30, 34, 43, 51, 30, 46, 59, 41, 33, 34, 30],
    [59, 50, 45, 41, 45, 51, 55, 58, 29, 21, 11, 0, 38, 37, 32, 36, 41, 49, 33, 45, 55, 39, 36, 36, 32],
    [26, 22, 20, 23, 29, 36, 41, 48, 17, 25, 32, 38, 0, 14, 26, 32, 48, 56, 17, 52, 66, 47, 30, 39, 26],
    [34, 29, 25, 26, 27, 34, 38, 45, 21, 27, 32, 37, 14, 0, 18, 26, 44, 52, 21, 48, 61, 42, 29, 35, 23],
    [48, 41, 37, 34, 30, 30, 33, 38, 30, 28, 30, 32, 26, 18, 0, 16, 34, 44, 29, 42, 53, 36, 27, 31, 26],
    [55, 48, 43, 39, 36, 35, 38, 42, 36, 33, 34, 36, 32, 26, 16, 0, 30, 40, 33, 39, 49, 34, 29, 28, 25],
    [69, 62, 58, 54, 49, 47, 46, 44, 49, 46, 43, 41, 48, 44, 34, 30, 0, 22, 41, 27, 35, 22, 29, 23, 28],
    [76, 69, 65, 61, 56, 54, 51, 48, 56, 55, 51, 49, 56, 52, 44, 40, 22, 0, 48, 30, 30, 25, 32, 27, 30],
    [40, 35, 30, 33, 36, 44, 48, 55, 20, 23, 30, 33, 17, 21, 29, 33, 41, 48, 0, 45, 59, 40, 28, 34, 27],
    [63, 59, 53, 49, 45, 43, 41, 39, 51, 48, 46, 45, 52, 48, 42, 39, 27, 30, 45, 0, 22, 17, 29, 23, 26],
    [78, 73, 67, 63, 59, 54, 49, 43, 67, 63, 59, 55, 66, 61, 53, 49, 35, 30, 59, 22, 0, 14, 31, 24, 30],
    [58, 54, 48, 45, 41, 38, 34, 30, 47, 44, 41, 39, 47, 42, 36, 34, 22, 25, 40, 17, 14, 0, 27, 21, 24],
    [42, 36, 31, 29, 33, 35, 37, 41, 32, 31, 33, 36, 30, 29, 27, 29, 29, 32, 28, 29, 31, 27, 0, 18, 22],
    [54, 48, 43, 39, 37, 35, 32, 31, 41, 36, 34, 36, 39, 35, 31, 28, 23, 27, 34, 23, 24, 21, 18, 0, 19],
    [37, 32, 28, 27, 30, 29, 28, 29, 25, 27, 30, 32, 26, 23, 26, 25, 28, 30, 27, 26, 30, 24, 22, 19, 0],
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