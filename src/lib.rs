#![warn(missing_docs)]

//! This crate implements functions to search in a graph.
//!
//! It supports the following Cargo features:
//!
//! - `edmonds_karp`: include the Edmonds-Karp algorithm variants
//!   (default: true)
//! - `kuhn_munkres`: include the Kuhn-Munkres algorithm (default: true)

#[cfg(feature = "kuhn_munkres")]
extern crate fixedbitset;
#[cfg(any(feature = "edmonds_karp", feature = "kuhn_munkres"))]
pub extern crate ndarray;
pub extern crate num_traits;

mod astar;
mod bfs;
mod dfs;
mod dijkstra;
#[cfg(feature = "edmonds_karp")]
mod edmonds_karp;
mod fringe;
mod idastar;
#[cfg(feature = "kuhn_munkres")]
mod kuhn_munkres;

pub use astar::*;
pub use bfs::*;
pub use dfs::*;
pub use dijkstra::*;
#[cfg(feature = "edmonds_karp")]
pub use edmonds_karp::*;
pub use fringe::*;
pub use idastar::*;
#[cfg(feature = "kuhn_munkres")]
pub use kuhn_munkres::*;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

struct InvCmpHolder<K, P> {
    key: K,
    payload: P,
}

impl<K: PartialEq, P> PartialEq for InvCmpHolder<K, P> {
    fn eq(&self, other: &InvCmpHolder<K, P>) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K: PartialEq, P> Eq for InvCmpHolder<K, P> {}

impl<K: PartialOrd, P> PartialOrd for InvCmpHolder<K, P> {
    fn partial_cmp(&self, other: &InvCmpHolder<K, P>) -> Option<Ordering> {
        other.key.partial_cmp(&self.key)
    }
}

impl<K: Ord, P> Ord for InvCmpHolder<K, P> {
    fn cmp(&self, other: &InvCmpHolder<K, P>) -> Ordering {
        other.key.cmp(&self.key)
    }
}

fn reverse_path<N: Eq + Hash>(mut parents: HashMap<N, N>, start: N) -> Vec<N> {
    let mut path = vec![start];
    while let Some(parent) = parents.remove(path.last().unwrap()) {
        path.push(parent);
    }
    path.into_iter().rev().collect()
}
