pub fn nexts_backward(lis: &CLis) -> Vec<CLis> {
    let mut res = vec![];
    // β2: (1,..) -> (..,1,3)
    if lis.len() >= 2 {
        if let [.., 1, 3] = lis[lis.len() - 2..] {
            res.push(
                [1].iter()
                    .cloned()
                    .chain(lis[..lis.len() - 2].to_vec())
                    .collect(),
            );
        }
    }
    // β1: (1,2,..) -> (..,4)
    if let [.., 4] = lis[lis.len() - 1..] {
        res.push(
            [1, 2]
                .iter()
                .cloned()
                .chain(lis[..lis.len() - 1].to_vec())
                .collect(),
        );
    }
    //β2: (1,3,..) -> (..,2,3)
    if lis.len() >= 2 {
        if let [.., 2, 3] = lis[lis.len() - 2..] {
            res.push(
                [1, 3]
                    .iter()
                    .cloned()
                    .chain(lis[..lis.len() - 2].to_vec())
                    .collect(),
            );
        }
    }
    //β1: (1,3,2,..) -> (..,5)
    if let [.., 5] = lis[lis.len() - 1..] {
        res.push(
            [1, 3, 2]
                .iter()
                .cloned()
                .chain(lis[..lis.len() - 1].to_vec())
                .collect(),
        );
    }
    //β3: (1,3,4,..) -> (1,..,3)
    if lis.len() >= 2 {
        if let [1, ..] = lis[..1] {
            if let [.., 3] = lis[lis.len() - 1..] {
                res.push(
                    [1, 3, 4]
                        .iter()
                        .cloned()
                        .chain(lis[1..lis.len() - 1].to_vec())
                        .collect(),
                );
            }
        }
    }
    //β3: (1,3,5,..) -> (2,..,3)
    if lis.len() >= 2 {
        if let [2, ..] = lis[..1] {
            if let [.., 3] = lis[lis.len() - 1..] {
                res.push(
                    [1, 3, 5]
                        .iter()
                        .cloned()
                        .chain(lis[1..lis.len() - 1].to_vec())
                        .collect(),
                );
            }
        }
    }
    //β3: (1,4,..) -> (1,..,2)
    if lis.len() >= 2 {
        if let [1, ..] = lis[..1] {
            if let [.., 2] = lis[lis.len() - 1..] {
                res.push(
                    [1, 4]
                        .iter()
                        .cloned()
                        .chain(lis[1..lis.len() - 1].to_vec())
                        .collect(),
                );
            }
        }
    }
    //β3: (1,5,..) -> (2,..,2)
    if lis.len() >= 2 {
        if let [2, ..] = lis[..1] {
            if let [.., 2] = lis[lis.len() - 1..] {
                res.push(
                    [1, 5]
                        .iter()
                        .cloned()
                        .chain(lis[1..lis.len() - 1].to_vec())
                        .collect(),
                );
            }
        }
    }
    //α: (2,..) -> (1,..,1)
    if lis.len() >= 2 {
        if let [1, ..] = lis[..1] {
            if let [.., 1] = lis[lis.len() - 1..] {
                res.push(
                    [2].iter()
                        .cloned()
                        .chain(lis[1..lis.len() - 1].to_vec())
                        .collect(),
                );
            }
        }
    }
    //α: (3,..) -> (2,..,1)
    if lis.len() >= 2 {
        if let [2, ..] = lis[..1] {
            if let [.., 1] = lis[lis.len() - 1..] {
                res.push(
                    [3].iter()
                        .cloned()
                        .chain(lis[1..lis.len() - 1].to_vec())
                        .collect(),
                );
            }
        }
    }
    //α: (4,..) -> (3,..,1)
    if lis.len() >= 2 {
        if let [3, ..] = lis[..1] {
            if let [.., 1] = lis[lis.len() - 1..] {
                res.push(
                    [4].iter()
                        .cloned()
                        .chain(lis[1..lis.len() - 1].to_vec())
                        .collect(),
                );
            }
        }
    }
    //α: (5,..) -> (4,..,1)
    if lis.len() >= 2 {
        if let [4, ..] = lis[..1] {
            if let [.., 1] = lis[lis.len() - 1..] {
                res.push(
                    [5].iter()
                        .cloned()
                        .chain(lis[1..lis.len() - 1].to_vec())
                        .collect(),
                );
            }
        }
    }
    res
}
pub fn nexts_forward(lis: &CLis) -> Vec<CLis> {
    let mut res = vec![];
    // β2: (1,..) -> (..,1,3)
    if let [1, ..] = lis[..1] {
        res.push(
            lis[1..]
                .iter()
                .copied()
                .chain([1, 3].iter().cloned())
                .collect(),
        );
    }
    // β1: (1,2,..) -> (..,4)
    if let [1, 2, ..] = lis[..2] {
        res.push(
            lis[2..]
                .iter()
                .copied()
                .chain([4].iter().cloned())
                .collect(),
        );
    }
    // β2: (1,3,..) -> (..,2,3)
    if let [1, 3, ..] = lis[..2] {
        res.push(
            lis[2..]
                .iter()
                .copied()
                .chain([2, 3].iter().cloned())
                .collect(),
        );
    }
    // β1: (1,3,2,..) -> (..,5)
    if lis.len() >= 3 {
        if let [1, 3, 2, ..] = lis[..3] {
            res.push(
                lis[3..]
                    .iter()
                    .copied()
                    .chain([5].iter().cloned())
                    .collect(),
            );
        }
    }
    // β3: (1,3,4,..) -> (1,..,3)
    if lis.len() >= 3 {
        if let [1, 3, 4, ..] = lis[..3] {
            res.push(
                [1].iter()
                    .cloned()
                    .chain(lis[3..].to_vec())
                    .chain([3].iter().cloned())
                    .collect(),
            );
        }
    }
    // β3: (1,3,5,..) -> (2,..,3)
    if lis.len() >= 3 {
        if let [1, 3, 5, ..] = lis[..3] {
            res.push(
                [2].iter()
                    .cloned()
                    .chain(lis[3..].to_vec())
                    .chain([3].iter().cloned())
                    .collect(),
            );
        }
    }
    // β3: (1,4,..) -> (1,..,2)
    if let [1, 4, ..] = lis[..2] {
        res.push(
            [1].iter()
                .cloned()
                .chain(lis[2..].to_vec())
                .chain([2].iter().cloned())
                .collect(),
        );
    }
    // β3: (1,5,..) -> (2,..,2)
    if let [1, 5, ..] = lis[..2] {
        res.push(
            [2].iter()
                .cloned()
                .chain(lis[2..].to_vec())
                .chain([2].iter().cloned())
                .collect(),
        );
    }
    // α: (2,..) -> (1,..,1)
    if let [2, ..] = lis[..1] {
        res.push(
            [1].iter()
                .cloned()
                .chain(lis[1..].to_vec())
                .chain([1].iter().cloned())
                .collect(),
        );
    }
    // α: (3,..) -> (2,..,1)
    if let [3, ..] = lis[..1] {
        res.push(
            [2].iter()
                .cloned()
                .chain(lis[1..].to_vec())
                .chain([1].iter().cloned())
                .collect(),
        );
    }
    // α: (4,..) -> (3,..,1)
    if let [4, ..] = lis[..1] {
        res.push(
            [3].iter()
                .cloned()
                .chain(lis[1..].to_vec())
                .chain([1].iter().cloned())
                .collect(),
        );
    }
    // α: (5,..) -> (4,..,1)
    if let [5, ..] = lis[..1] {
        res.push(
            [4].iter()
                .cloned()
                .chain(lis[1..].to_vec())
                .chain([1].iter().cloned())
                .collect(),
        );
    }
    res
}
