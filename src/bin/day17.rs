use std::ops::BitXor;

use itertools::{Itertools, min};

static PROGRAM: &'static [u64] = &[
    2, 4, // bst
    1, 7, // bxl
    7, 5, // cdv
    4, 1, // bxc
    1, 4, // bxl
    5, 5, // out
    0, 3, // adv
    3, 0, // jnz
];

/*
24: b = a % 8
17: b = b xor (111)
75: c = a / 2^b     # c = a >> b
41: b = b xor c
14: b = b xor (100)
55: out b % 8
03: a = a / 2^3 (8) # a = a >> 3
30: if a != 0 {jump start}
*/

static REG_A: u64 = 53437164;
static REG_B: u64 = 0;
static REG_C: u64 = 0;

fn run(program: &[u64], rega: u64, regb: u64, regc: u64) -> (Vec<u64>, u64, u64, u64) {
    let mut pc = 0;
    let mut rega = rega;
    let mut regb = regb;
    let mut regc = regc;

    let mut output = vec![];

    loop {
        if pc > program.len() - 2 {
            return (output, rega, regb, regc);
        }
        let instr = program[pc];
        let operand = program[pc + 1];
        let combo = || match operand {
            0..=3 => operand,
            4 => rega,
            5 => regb,
            6 => regc,
            7 => panic!("operand 7 should not appear"),
            _ => panic!("invalid operand"),
        };

        match instr {
            0 => {
                // adv division
                rega = rega >> combo();
                pc += 2;
            }
            1 => {
                // bxl bitwise XOR
                regb = regb.bitxor(operand);
                pc += 2;
            }
            2 => {
                // bst
                regb = combo() % 8;
                pc += 2;
            }
            3 => {
                // jnz
                if rega == 0 {
                    pc += 2;
                } else {
                    pc = operand as usize;
                }
            }
            4 => {
                // bxc
                regb = regb.bitxor(regc);
                pc += 2;
            }
            5 => {
                // out
                output.push(combo() % 8);
                pc += 2;
            }
            6 => {
                // bdv division
                regb = rega >> combo();
                pc += 2;
            }
            7 => {
                // cdv division
                regc = rega >> combo();
                pc += 2;
            }
            _ => panic!("unknown opcode {}", instr),
        }
    }
}

fn part2() -> Option<u64> {
    let mut branches = vec![(PROGRAM.len() - 1, 0)];
    let mut solutions = vec![];

    while let Some((target_n, rega)) = branches.pop() {
        let target = PROGRAM[target_n];
        for i in 0..8 {
            let new_rega = (rega << 3).bitxor(i);
            if run(&PROGRAM[..14], new_rega, 0, 0).0[0] == target {
                if target_n == 0 {
                    solutions.push(new_rega);
                } else {
                    branches.push((target_n - 1, new_rega));
                }
            }
        }
    }

    min(solutions)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (part1, _, _, _) = run(&PROGRAM, REG_A, REG_B, REG_C);

    let part1_out = part1.into_iter().map(|i| i.to_string()).join(",");
    println!("Part 1: {}", part1_out);
    assert_eq!(part1_out, "2,1,0,4,6,2,4,2,0");

    let part2_rega = part2().unwrap();
    println!("Part 2: {}", part2_rega);
    assert_eq!(part2_rega, 109685330781408);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let (_, _, _, regc) = run(&[2, 6], 0, 0, 9);
        assert_eq!(regc, 9);
    }

    #[test]
    fn ex2() {
        let (output, _, _, _) = run(&[5, 0, 5, 1, 5, 4], 10, 0, 0);
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn ex3() {
        let (output, rega, _, _) = run(&[0, 1, 5, 4, 3, 0], 2024, 0, 0);
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(rega, 0);
    }

    #[test]
    fn ex4() {
        let (_, _, regb, _) = run(&[1, 7], 0, 29, 0);
        assert_eq!(regb, 26);
    }

    #[test]
    fn ex5() {
        let (_, _, regb, _) = run(&[4, 0], 0, 2024, 43690);
        assert_eq!(regb, 44354);
    }

    #[test]
    fn part1_sample() {
        let (output, _, _, _) = run(&[0, 1, 5, 4, 3, 0], 729, 0, 0);
        assert_eq!(output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn part2_sample() {
        let prog = vec![0, 3, 5, 4, 3, 0];
        let (output, _, _, _) = run(&prog, 117440, 0, 0);
        assert_eq!(prog, output);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(PROGRAM, run(PROGRAM, part2().unwrap(), 0, 0).0);
    }
}
