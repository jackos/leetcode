/// from https://leetcode.com/problems/cherry-pickup-ii/
/// Uses top down DP to calculate highest scoring paths
struct Solution;
impl Solution {
    fn dp(
        row: usize,
        col1: usize,
        col2: usize,
        grid: &Vec<Vec<i32>>,
        dp_cache: &mut [&mut [&mut [i32]]],
    ) -> i32 {
        if col1 >= grid[0].len() || col2 >= grid[0].len() {
            return 0;
        }
        // check cache
        if dp_cache[row][col1][col2] != -1 {
            return dp_cache[row][col1][col2];
        }

        // Current cell
        let mut result = 0;
        result += grid[row][col1];
        println!("Row: {}", row);
        println!("robot 1: {} points: {}", col1, grid[row][col1]);
        if col1 != col2 {
            result += grid[row][col2];
            println!("robot 2: {} points: {}", col2, grid[row][col2]);
        }

        // transition
        if row != grid.len() - 1 {
            let col1_start = col1.checked_sub(1).unwrap_or(0);
            let col2_start = col2.checked_sub(1).unwrap_or(0);
            let mut max = 0;
            for new_col_1 in col1_start..col1 + 2 {
                for new_col_2 in col2_start..col2 + 2 {
                    max = std::cmp::max(
                        max,
                        Solution::dp(row + 1, new_col_1, new_col_2, &grid, dp_cache),
                    )
                }
            }
            result += max;
        }

        dp_cache[row][col1][col2] = result;
        result
    }

    fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid_raw = vec![-1; m * n * n];
        let mut grid_2d: Vec<_> = grid_raw.as_mut_slice().chunks_mut(n).collect();
        let mut grid_3d: Vec<_> = grid_2d.as_mut_slice().chunks_mut(n).collect();
        let dp_cache = grid_3d.as_mut_slice();
        return Solution::dp(0, 0, n - 1, &grid, dp_cache);
    }
}

fn main() {
    let x = vec![vec![3, 1, 1], vec![2, 5, 1], vec![1, 5, 5], vec![2, 1, 1]];
    let result = Solution::cherry_pickup(x);
    println!("Final Result: {}", result);
}
