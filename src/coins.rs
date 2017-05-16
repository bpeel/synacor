const N_PIECES: usize = 5;
const PIECES: [i32; N_PIECES] = [ 3, 2, 9, 5, 7 ];

fn try_permutation(permutation: &[i32]) {
    let sum =
        permutation[0] +
        permutation[1] *
        permutation[2] *
        permutation[2] +
        permutation[3] *
        permutation[3] *
        permutation[3] -
        permutation[4];

    if sum != 399 {
        return;
    }

    for coin in permutation {
        print!("{} ", coin);
    }
    println!("= {}", sum);
}

fn swap(permutation: &mut [i32],
        a: usize,
        b: usize) {
    let tmp = permutation[a];
    permutation[a] = permutation[b];
    permutation[b] = tmp;
}

fn permute() {
    let mut permutation = PIECES;
    let mut stack = [-1 as i32; N_PIECES + 1];
    let mut depth: usize = 0;

    loop {
        if depth >= N_PIECES {
            try_permutation(&permutation);
        }

        stack[depth] += 1;

        if stack[depth] >= N_PIECES as i32 {
            if depth <= 0 {
                break;
            }
            depth -= 1;
            swap(&mut permutation,
                 stack[depth] as usize,
                 depth);
        } else {
            swap(&mut permutation,
                 stack[depth] as usize,
                 depth);
            depth += 1;
            stack[depth] = (depth - 1) as i32;
        }
    }
}

fn main() {
    permute();
}
