dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)]

def flood_fill_with_perimeter(grid, start, visited):
    """
    Flood-fill to calculate the area and perimeter of a region.
    Perimeter counts sides on the same line as one.

    Args:
    - grid (list of list of str): The 2D map of letters.
    - start (tuple of int): Starting cell (row, col).
    - visited (list of list of bool): Boolean grid to mark visited cells.

    Returns:
    - area (int): The number of cells in the region.
    - perimeter (int): The total perimeter of the region.
    """
    rows, cols = len(grid), len(grid[0])
    letter = grid[start[0]][start[1]]
    stack = [start]
    area = 0

    # For horizontal and vertical boundaries
    horizontal_boundaries = set()
    vertical_boundaries = set()

    while stack:
        r, c = stack.pop()
        if visited[r][c]:
            continue
        visited[r][c] = True
        area += 1

        # Check boundaries for current cell
        for dr, dc in dirs:
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == letter:
                if not visited[nr][nc]:
                    stack.append((nr, nc))
            else:
                # Track boundary transitions (horizontal and vertical)
                if dr != 0:  # Vertical boundary
                    vertical_boundaries.add((min(r, nr), c))
                if dc != 0:  # Horizontal boundary
                    horizontal_boundaries.add((r, min(c, nc)))

    # Perimeter is the count of distinct horizontal and vertical boundaries
    perimeter = len(horizontal_boundaries) + len(vertical_boundaries)
    return area * perimeter

map_data = None
with open("inputs/input12.txt", "r") as file_handle:
    map_data = [line.strip() for line in file_handle]

# Convert map to a grid
grid = [list(row) for row in map_data]
rows, cols = len(grid), len(grid[0])
visited = [[False] * cols for _ in range(rows)]

# Collect results
results = {}
total = 0
for r in range(rows):
    for c in range(cols):
        if not visited[r][c]:
            letter = grid[r][c]
            total += flood_fill_with_perimeter(grid, (r, c), visited)
            # if letter not in results:
            #     results[letter] = []
            # results[letter].append({'area': area, 'perimeter': perimeter})

print(total)
