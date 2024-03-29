use std::time::Instant;

const PROGRAM: [i32; 678] = [3,225,1,225,6,6,1100,1,238,225,104,0,1102,7,85,225,1102,67,12,225,102,36,65,224,1001,224,-3096,224,4,224,1002,223,8,223,101,4,224,224,1,224,223,223,1001,17,31,224,1001,224,-98,224,4,224,1002,223,8,223,101,5,224,224,1,223,224,223,1101,86,19,225,1101,5,27,225,1102,18,37,225,2,125,74,224,1001,224,-1406,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1102,13,47,225,1,99,14,224,1001,224,-98,224,4,224,102,8,223,223,1001,224,2,224,1,224,223,223,1101,38,88,225,1102,91,36,224,101,-3276,224,224,4,224,1002,223,8,223,101,3,224,224,1,224,223,223,1101,59,76,224,1001,224,-135,224,4,224,102,8,223,223,1001,224,6,224,1,223,224,223,101,90,195,224,1001,224,-112,224,4,224,102,8,223,223,1001,224,7,224,1,224,223,223,1102,22,28,225,1002,69,47,224,1001,224,-235,224,4,224,1002,223,8,223,101,5,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,107,226,226,224,102,2,223,223,1006,224,329,1001,223,1,223,1107,677,226,224,1002,223,2,223,1005,224,344,101,1,223,223,108,677,226,224,102,2,223,223,1006,224,359,101,1,223,223,7,677,226,224,102,2,223,223,1005,224,374,101,1,223,223,1008,677,226,224,1002,223,2,223,1006,224,389,1001,223,1,223,7,226,677,224,102,2,223,223,1005,224,404,101,1,223,223,1007,226,226,224,102,2,223,223,1006,224,419,101,1,223,223,7,226,226,224,102,2,223,223,1005,224,434,1001,223,1,223,8,226,226,224,1002,223,2,223,1006,224,449,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,464,101,1,223,223,1007,226,677,224,1002,223,2,223,1006,224,479,101,1,223,223,108,226,226,224,102,2,223,223,1005,224,494,1001,223,1,223,1108,677,677,224,102,2,223,223,1005,224,509,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,524,101,1,223,223,1108,677,226,224,1002,223,2,223,1005,224,539,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,554,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,569,1001,223,1,223,8,677,226,224,102,2,223,223,1006,224,584,101,1,223,223,107,677,677,224,102,2,223,223,1006,224,599,101,1,223,223,8,226,677,224,102,2,223,223,1006,224,614,101,1,223,223,1107,226,677,224,102,2,223,223,1006,224,629,101,1,223,223,108,677,677,224,1002,223,2,223,1005,224,644,1001,223,1,223,1107,226,226,224,102,2,223,223,1005,224,659,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,674,101,1,223,223,4,223,99,226];

fn parse_inst(code: i32) -> (i32, bool, bool) {
  let opcode = code % 100;
  let imm1 = (code / 100)  % 10 == 1;
  let imm2 = (code / 1000) % 10 == 1;
  (opcode, imm1, imm2)
}

fn fetch(program: &[i32], pos: usize, imm: bool) -> i32 {
  let index = if imm { pos } else { program[pos] as usize };
  program[index]
}

fn run(input: i32) {
  let mut program = PROGRAM;
  let mut pc = 0;
  loop {
    let (opcode, imm1, imm2) = parse_inst(program[pc]);
    match opcode {
      1 => { // add
        let a = fetch(&program, pc+1, imm1);
        let b = fetch(&program, pc+2, imm2);
        let c = fetch(&program, pc+3, true) as usize;
        program[c] = a + b;
        pc += 4;
      },
      2 => { // mul
        let a = fetch(&program, pc+1, imm1);
        let b = fetch(&program, pc+2, imm2);
        let c = fetch(&program, pc+3, true) as usize;
        program[c] = a * b;
        pc += 4;
      },
      3 => { // input
        let a = fetch(&program, pc+1, true) as usize;
        program[a] = input;
        pc += 2;
      },
      4 => { // output
        println!("{}", fetch(&program, pc+1, imm1));
        pc += 2;
      },
      5 => { // jnz
        let a = fetch(&program, pc+1, imm1);
        let b = fetch(&program, pc+2, imm2);
        pc = if a != 0 { b as usize } else { pc + 3 };
      },
      6 => { // jz
        let a = fetch(&program, pc+1, imm1);
        let b = fetch(&program, pc+2, imm2);
        pc = if a == 0 { b as usize } else { pc + 3 };
      },
      7 => { // slt
        let a = fetch(&program, pc+1, imm1);
        let b = fetch(&program, pc+2, imm2);
        let c = fetch(&program, pc+3, true) as usize;
        program[c] = (a < b) as i32;
        pc += 4;
      },
      8 => { // seq
        let a = fetch(&program, pc+1, imm1);
        let b = fetch(&program, pc+2, imm2);
        let c = fetch(&program, pc+3, true) as usize;
        program[c] = (a == b) as i32;
        pc += 4;
      },
      _ => break
    }
  }
}

fn main() {
  let now = Instant::now();
  println!("Part one:");
  run(1);
  print!("Part two: ");
  run(5);
  println!("Time: {}ms", now.elapsed().as_millis());
}
