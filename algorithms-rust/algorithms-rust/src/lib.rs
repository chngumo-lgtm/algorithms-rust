//! Classic algorithms implemented in safe Rust.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// In-place insertion sort. O(n^2) worst case, O(n) on sorted input.
pub fn insertion_sort<T: PartialOrd + Copy>(v: &mut [T]) {
    for i in 1..v.len() {
        let key = v[i];
        let mut j = i;
        while j > 0 && v[j - 1] > key {
            v[j] = v[j - 1];
            j -= 1;
        }
        v[j] = key;
    }
}

/// Merge sort returning a new vector. O(n log n).
pub fn merge_sort<T: PartialOrd + Copy>(v: &[T]) -> Vec<T> {
    if v.len() <= 1 {
        return v.to_vec();
    }
    let mid = v.len() / 2;
    let (l, r) = (merge_sort(&v[..mid]), merge_sort(&v[mid..]));
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::with_capacity(v.len());
    while i < l.len() && j < r.len() {
        if l[i] <= r[j] {
            out.push(l[i]);
            i += 1;
        } else {
            out.push(r[j]);
            j += 1;
        }
    }
    out.extend_from_slice(&l[i..]);
    out.extend_from_slice(&r[j..]);
    out
}

/// Binary search on a sorted slice. Returns the index if found.
pub fn binary_search<T: Ord>(v: &[T], target: &T) -> Option<usize> {
    let (mut lo, mut hi) = (0usize, v.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match v[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}

/// Dijkstra on an adjacency list `graph[u] = [(v, weight)]`.
/// Returns `None` for unreachable nodes.
pub fn dijkstra(graph: &[Vec<(usize, u64)>], source: usize) -> Vec<Option<u64>> {
    let mut dist: Vec<Option<u64>> = vec![None; graph.len()];
    let mut heap = BinaryHeap::new();
    dist[source] = Some(0);
    heap.push(Reverse((0u64, source)));
    while let Some(Reverse((d, u))) = heap.pop() {
        if dist[u].map_or(false, |best| d > best) {
            continue;
        }
        for &(v, w) in &graph[u] {
            let nd = d + w;
            if dist[v].map_or(true, |best| nd < best) {
                dist[v] = Some(nd);
                heap.push(Reverse((nd, v)));
            }
        }
    }
    dist
}

/// Longest common subsequence length. O(n*m).
pub fn lcs_length(a: &str, b: &str) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let mut dp = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    dp[a.len()][b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_match_std() {
        let data = vec![5, 2, 9, -1, 0, 5, 3];
        let mut expected = data.clone();
        expected.sort();
        let mut ins = data.clone();
        insertion_sort(&mut ins);
        assert_eq!(ins, expected);
        assert_eq!(merge_sort(&data), expected);
    }

    #[test]
    fn sort_edge_cases() {
        assert_eq!(merge_sort::<i32>(&[]), vec![]);
        assert_eq!(merge_sort(&[1]), vec![1]);
    }

    #[test]
    fn binary_search_works() {
        let v = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&v, &7), Some(3));
        assert_eq!(binary_search(&v, &4), None);
        assert_eq!(binary_search::<i32>(&[], &1), None);
    }

    #[test]
    fn dijkstra_works() {
        let g = vec![vec![(1, 1), (2, 4)], vec![(2, 2), (3, 6)], vec![(3, 3)], vec![], vec![]];
        assert_eq!(dijkstra(&g, 0), vec![Some(0), Some(1), Some(3), Some(6), None]);
    }

    #[test]
    fn lcs_works() {
        assert_eq!(lcs_length("ABCBDAB", "BDCABA"), 4);
        assert_eq!(lcs_length("", "abc"), 0);
    }
}
