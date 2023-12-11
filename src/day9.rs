#[allow(dead_code)]
enum Direction {
    Backwards,
    Forwards
}

#[allow(dead_code)]
fn is_ap(seq: &Vec<i64>) -> Option<i64> {
    let mut last_diff = None;
    for pair in seq.windows(2) {
        let d = pair[1] - pair[0];
        if let Some(p_d) = last_diff {
            if p_d != d {
                return None;
            }
        } else {
            last_diff = Some(d);
        }
    }
    return last_diff;
}

#[allow(dead_code)]
fn predict_value(seq: &Vec<i64>) -> (i64, i64) {
    if let Some(diff) = is_ap(seq) {
        return (seq[0] - diff, diff + seq[seq.len() - 1]);
    } else {
        let mut d_seq = vec![];
        for pair in seq.windows(2) {
            d_seq.push(pair[1] - pair[0]);
        }
        let (b, n) = predict_value(&d_seq);
        let before = seq[0] - b;
        let after = seq[seq.len() - 1] + n;
        return (before, after);
    }
}

#[allow(dead_code)]
pub fn soln(lines: Vec<String>) -> (i64, i64) {
    let mut sum_forwards = 0;
    let mut sum_backwards = 0;
    let mut sequences: Vec<Vec<i64>> = vec![];
    for line in lines {
        let mut sequence = vec![];
        for split in line.split(" ") {
            if let Ok(num) = split.parse::<i64>() {
                sequence.push(num);
            }
        }
        if !sequence.is_empty() {
            sequences.push(sequence);
        }
    }
    for sequence in sequences {
        let (b, n) = predict_value(&sequence);
        sum_forwards += n;
        sum_backwards += b;
    }
    (sum_forwards, sum_backwards)
}
