use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::Solution;

#[derive(Copy, Clone)]
struct Idx2D {
    row: usize,
    column: usize,
}

pub struct Foo<const N: usize> {
    bytes: [u8; N],
}

fn foo<const N: usize>(take: Box<Foo<N>>) {
    struct Foo([i32; 1]);
}

impl Idx2D {
    fn desired(number: i32) -> Idx2D {
        let (row, column) = match number {
            1 => (0, 0),
            2 => (0, 1),
            3 => (0, 2),
            4 => (1, 0),
            5 => (1, 1),
            6 => (1, 2),
            _ => panic!("invalid number")
        };
        Idx2D { row, column }
    }

    fn manhatten(&self, other: Idx2D) -> usize {
        let d_column = (self.column as i64 - other.column as i64).abs();
        let d_row = (self.row as i64 - other.row as i64).abs();
        (d_column + d_row) as usize
    }
}

fn heuristic(board: &Vec<Vec<i32>>) -> usize {
    /*
    goal = [
        [1,2,3],
        [4,5,0]
    ]
     */

    let mut summed_manhatten_dist: usize = 0;
    for (row_idx, row) in board.iter().enumerate() {
        for (column_idx, &entry) in row.iter().enumerate() {
            let desired = Idx2D::desired(entry);
            let actual = Idx2D { row: row_idx, column: column_idx };
            let manhatten_dist = desired.manhatten(actual);
            summed_manhatten_dist += manhatten_dist;
        }
    }
    return summed_manhatten_dist;
}

struct Node {
    state: Vec<Vec<i32>>,
    g_score: usize,
    f_score: usize, // g + h
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}


impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        /*
        This is very similar to the 8-puzzle problem

        A simple way to solve this is BFS, but if we add a heuristic we can use
        A* and have a much faster run-time

        Let us relax the problem: we can move pieces wherever we want independently
        but we still must move adjacently. Therefore, the heuristic given a board
        is the sum of the Manhattan distances between the current and goal
         */

        let init_heuristic = heuristic(&board);

        let init_node = Node {
            state: board,
            g_score: 0,
            f_score: init_heuristic,
        };

        let mut heap = BinaryHeap::new();
        heap.push(init_node);

        while let Some(smallest) = heap.pop() {
            let h_score = smallest.f_score - smallest.g_score;
            if h_score == 0 {
                return smallest.g_score as i32
            }



        }

        unreachable!()
    }
}
