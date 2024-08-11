use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::collections::binary_heap;
use std::collections::HashSet;

pub type CLis = Vec<usize>;
pub fn nexts(lis: &CLis, rev: bool) -> Vec<CLis> {
    if rev {
        nexts_backward(lis)
    } else {
        nexts_forward(lis)
    }
}
pub fn nexts_backward(lis: &CLis) -> Vec<CLis> {
    let mut res = vec![];
    // α: (a,..) -> (a-1,..,1):
    if lis.len() >= 2 {
        if let Some(a) = lis.first().copied() {
            let a = a + 1;
            if lis.last() == Some(&1) {
                res.push(
                    [a].iter()
                        .cloned()
                        .chain(lis[1..lis.len() - 1].to_vec())
                        .collect(),
                );
            }
        }
    }
    // β1: (1,{3;n},2,..) -> (..,n+4):
    if let Some(last) = lis.last().copied() {
        if last >= 4 {
            let n = last - 4;
            res.push(
                [1].iter()
                    .cloned()
                    .chain(std::iter::repeat(3).take(n))
                    .chain([2].iter().cloned())
                    .chain(lis[..lis.len() - 1].to_vec())
                    .collect(),
            );
        }
    }
    // β2: (1,{3;n},..) -> (..,n+1,3):
    if lis.len() >= 2 && lis.last() == Some(&3) {
        let n = *lis.get(lis.len() - 2).unwrap();
        if n >= 1 {
            let n = n - 1;
            res.push(
                [1].iter()
                    .cloned()
                    .chain(std::iter::repeat(3).take(n))
                    .chain(lis[..lis.len() - 2].to_vec())
                    .collect(),
            );
        }
    }
    // β3: (1,{3;n},a,..) -> (a-3,..,n+2):
    if lis.len() >= 2 {
        if let Some(last) = lis.last().copied() {
            if last >= 2 {
                let n = last - 2;
                if let Some(a) = lis.first().copied() {
                    let a = a + 3;
                    res.push(
                        [1].iter()
                            .cloned()
                            .chain(std::iter::repeat(3).take(n))
                            .chain([a].iter().cloned())
                            .chain(lis[1..lis.len() - 1].to_vec())
                            .collect(),
                    );
                }
            }
        }
    }
    res
}
pub fn nexts_forward(lis: &CLis) -> Vec<CLis> {
    let mut res = vec![];
    // α: (a,..) -> (a-1,..,1):
    if let [a, ..] = lis[..1] {
        if a >= 2 {
            res.push(
                [a - 1]
                    .iter()
                    .cloned()
                    .chain(lis[1..].to_vec())
                    .chain([1].iter().cloned())
                    .collect(),
            );
        }
    }
    // β1: (1,{3;n},2,..) -> (..,n+4):
    if let [1, ..] = lis[..1] {
        let mut n = 0;
        while lis.get(n + 1) == Some(&3) {
            n += 1;
        }
        if lis.get(n + 1) == Some(&2) {
            res.push(
                lis[n + 2..]
                    .iter()
                    .copied()
                    .chain([n + 4].iter().cloned())
                    .collect(),
            );
        }
    }
    // β2: (1,{3;n},..) -> (..,n+1,3):
    if let [1, ..] = lis[..1] {
        let mut n = 0;
        loop {
            res.push(
                lis[n + 1..]
                    .iter()
                    .copied()
                    .chain([n + 1, 3].iter().cloned())
                    .collect(),
            );
            if lis.get(n + 1) == Some(&3) {
                n += 1;
            } else {
                break;
            }
        }
    }
    // β3: (1,{3;n},a,..) -> (a-3,..,n+2):
    if let [1, ..] = lis[..1] {
        let mut n = 0;
        while lis.get(n + 1) == Some(&3) {
            n += 1;
        }
        if let [a, ..] = lis[n + 1..] {
            if a >= 4 {
                res.push(
                    [a - 3]
                        .iter()
                        .cloned()
                        .chain(lis[n + 2..].to_vec())
                        .chain([n + 2].iter().cloned())
                        .collect(),
                );
            }
        }
    }
    res
}

// check if the subsequence pat is included in the list
pub fn includes(lis: &CLis, pat: &CLis) -> bool {
    if lis.len() < pat.len() {
        return false;
    }
    for i in 0..lis.len() - pat.len() + 1 {
        if &lis[i..i + pat.len()] == pat {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    lis: CLis,
    bias: usize,
}
impl State {
    fn new(lis: CLis, bias: usize) -> Self {
        Self { lis, bias }
    }
    fn cost(&self) -> usize {
        self.lis.len() * 100 + self.bias
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost().cmp(&self.cost())
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

static MAX_ITER: usize = 20000000;
static MAX_LEN: usize = 3000;
static PRINT: bool = false;
pub fn list(seed: CLis, rng: &mut impl Rng, rev: bool) -> HashSet<CLis> {
    let mut visited = HashSet::new();
    //let mut queue = vec![seed];
    let mut queue: binary_heap::BinaryHeap<State> = binary_heap::BinaryHeap::new();
    queue.push(State::new(seed, 0));
    let mut iter = 0;
    while let Some(lis) = queue.pop() {
        let lis = lis.lis;
        //if includes(&lis, &vec![5, 3, 4]) {
        //    continue;
        //}
        if iter > MAX_ITER {
            break;
        }
        if visited.contains(&lis) {
            continue;
        }
        visited.insert(lis.clone());
        if PRINT {
            println!("{:?}", lis);
        }
        let nexts = nexts(&lis, rev);
        for next in nexts {
            if visited.contains(&next) {
                continue;
            }
            if next.len() > MAX_LEN {
                continue;
            }
            //queue.push(next);
            queue.push(State::new(next, iter + rng.gen_range(0..300)));
        }
        iter += 1;
        if (iter % 1000) == 0 {
            //queue.shuffle(rng);
            dbg!(iter);
        }
    }
    visited
}

fn main() {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(rng.gen());
    let from_start = list(vec![2, 1], &mut rng, false);
    let from_end = list(vec![1, 3], &mut rng, true);
    // if they have common elements, print them
    println!("====================");
    println!("Common elements");
    println!("====================");
    for lis in from_start.intersection(&from_end) {
        println!("{:?}", lis);
    }
}
