class Solution:

    def cherryPickup(self, grid):
        m = len(grid)
        n = len(grid[0])

        def dp(row, col1, col2):
            if col1 < 0 or col1 >= n or col2 < 0 or col2 >= n:
                return -inf
            # current cell
            result = 0
            result += grid[row][col1]
            if col1 != col2:
                result += grid[row][col2]
            # transition
            if row != m - 1:
                result += max(
                    dp(row + 1, new_col1, new_col2) for new_col1 in [col1, col1 + 1, col1 - 1]
                    for new_col2 in [col2, col2 + 1, col2 - 1])
            return result

        return dp(0, 0, n - 1)


Solution.cherryPickup("what", [[1, 2, 4], [5, 6, 7]])