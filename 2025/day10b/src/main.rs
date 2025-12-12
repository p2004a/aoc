fn gen_divisions<F>(mut n: u16, m: &[u16], mut f: F)
where
    F: FnMut(&[u16]) -> bool,
{
    let b = m.len();
    let mut state = vec![0; b];
    for i in 0..b {
        state[i] = n.min(m[i]);
        n -= state[i];
    }
    if n > 0 {
        return;
    }
    'outer: loop {
        if f(state.as_slice()) {
            return;
        }
        let mut dist = 0;
        for i in (1..b).rev() {
            dist += state[i];
            if state[i - 1] > 0 && state[i] < m[i] {
                state[i - 1] -= 1;
                dist += 1;
                for j in i..b {
                    state[j] = dist.min(m[j]);
                    dist -= state[j];
                }
                continue 'outer;
            }
        }
        break;
    }
}

fn build_button_sequence<'a>(
    buttons: &'a [Vec<u16>],
    joltage: &Vec<u16>,
) -> Vec<(usize, Vec<&'a [u16]>)> {
    let mut seq = Vec::new();
    let mut button_used = vec![false; buttons.len()];
    while button_used.iter().any(|&u| !u) {
        let mut buttons_for_joltage = vec![vec![]; joltage.len()];
        for (i, button) in buttons.iter().enumerate() {
            if !button_used[i] {
                for &level in button.iter() {
                    buttons_for_joltage[level as usize].push(i);
                }
            }
        }
        let level = buttons_for_joltage
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| if v.len() == 0 { usize::MAX } else { v.len() })
            .unwrap()
            .0;
        seq.push((
            level,
            buttons_for_joltage[level]
                .iter()
                .map(|&i| buttons[i].as_slice())
                .collect(),
        ));
        for &i in buttons_for_joltage[level].iter() {
            button_used[i] = true;
        }
    }
    seq
}

fn find_min_presses3(sequence: &[(usize, Vec<&[u16]>)], joltage: &Vec<u16>) -> Option<u32> {
    let level = sequence[0].0;
    let buttons = sequence[0].1.as_slice();
    let m = buttons
        .iter()
        .map(|&b| b.iter().map(|&i| joltage[i as usize]).min().unwrap())
        .collect::<Vec<_>>();

    let mut min_presses = None;
    let mut j = Vec::new();
    gen_divisions(joltage[level], m.as_slice(), |s| {
        j.clone_from(joltage);
        for (&button, &times) in buttons.iter().zip(s.iter()) {
            for &b in button.iter() {
                if j[b as usize] < times {
                    return false;
                }
                j[b as usize] -= times;
            }
        }
        if j.iter().all(|&v| v == 0) {
            min_presses = Some(joltage[level] as u32);
            return true;
        }
        if sequence.len() > 1
            && let Some(presses) = find_min_presses3(&sequence[1..], &j)
        {
            min_presses = Some(
                min_presses
                    .unwrap_or(u32::MAX)
                    .min(joltage[level] as u32 + presses),
            );
        }
        false
    });
    min_presses
}

fn main() {
    let mut res = 0;
    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let parts: Vec<_> = line.split(' ').collect();
        let sections: Vec<Vec<u16>> = parts[1..parts.len()]
            .iter()
            .map(|p| {
                p[1..p.len() - 1]
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();
        let buttons = &sections[..sections.len() - 1];
        let joltage = &sections[sections.len() - 1];

        let sequence = build_button_sequence(buttons, joltage);
        res += find_min_presses3(sequence.as_slice(), joltage).unwrap();
    }
    println!("{res}");
}
