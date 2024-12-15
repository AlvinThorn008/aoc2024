directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]

def cost_flood_fill(grid, start, visited):
    rows, cols = len(grid), len(grid[0])

    letter = grid[start[0]][start[1]]
    stack = [start]
    area = 0
    perimeter = 0

    horizontal_boundaries = set()
    vertical_boundaries = set()

    while stack:
        r, c = stack.pop()
        if visited[r][c]:
            continue
        
        visited[r][c] = True
        area += 1

        # # Number of exclusive sides of the current plot
        # plot_perimeter = 4
        for dr, dc in directions:
            nr, nc = r + dr, c + dc # Adjacent plot
            if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == letter:
                if not visited[nr][nc]:
                    stack.append((nr, nc))
            else:
                if dr != 0:  # Vertical boundary
                    vertical_boundaries.add((min(r, nr), c))
                if dc != 0:  # Horizontal boundary
                    horizontal_boundaries.add((r, min(c, nc)))

    perimeter = len(horizontal_boundaries) + len(vertical_boundaries)

    return area * perimeter


map_data = None
with open("inputs/input12.txt", "r") as file_handle:
    map_data = [line.strip() for line in file_handle]

grid = [list(row) for row in map_data]
rows, cols = len(grid), len(grid[0])
visited = [[False] * cols for _ in range(rows)]

# Collect results
total_cost = 0
for r in range(rows):
    for c in range(cols):
        if not visited[r][c]:
            letter = grid[r][c]
            total_cost += cost_flood_fill(grid, (r, c), visited)

print(f"Total cost: {total_cost}")
            
            # if letter not in results:
            #     results[letter] = []
            # results[letter].append({'area': area, 'perimeter': perimeter})

# Display results
# for letter, regions in results.items():
#     print(f"Letter: {letter}")
#     for i, region in enumerate(regions):
#         print(f"  Region {i + 1}: Area = {region['area']}, Perimeter = {region['perimeter']}")
