use std::time::Instant;
use intcoder::{IntCoder, ExitCode};

static PROGRAM: [i64; 2050] = [109,2050,21101,966,0,1,21102,13,1,0,1106,0,1378,21101,0,20,0,1105,1,1337,21101,0,27,0,1105,1,1279,1208,1,65,748,1005,748,73,1208,1,79,748,1005,748,110,1208,1,78,748,1005,748,132,1208,1,87,748,1005,748,169,1208,1,82,748,1005,748,239,21102,1041,1,1,21102,1,73,0,1105,1,1421,21102,1,78,1,21102,1041,1,2,21102,1,88,0,1106,0,1301,21102,1,68,1,21101,0,1041,2,21102,1,103,0,1106,0,1301,1102,1,1,750,1106,0,298,21102,1,82,1,21102,1041,1,2,21102,1,125,0,1106,0,1301,1102,2,1,750,1106,0,298,21102,1,79,1,21101,0,1041,2,21101,0,147,0,1105,1,1301,21102,84,1,1,21102,1,1041,2,21101,162,0,0,1106,0,1301,1101,3,0,750,1105,1,298,21102,65,1,1,21102,1041,1,2,21102,184,1,0,1106,0,1301,21102,76,1,1,21102,1041,1,2,21102,199,1,0,1105,1,1301,21101,75,0,1,21102,1,1041,2,21101,0,214,0,1105,1,1301,21101,221,0,0,1105,1,1337,21101,10,0,1,21101,1041,0,2,21101,0,236,0,1105,1,1301,1106,0,553,21101,0,85,1,21101,0,1041,2,21101,0,254,0,1106,0,1301,21101,0,78,1,21102,1,1041,2,21102,269,1,0,1105,1,1301,21102,1,276,0,1105,1,1337,21101,0,10,1,21101,1041,0,2,21102,1,291,0,1106,0,1301,1102,1,1,755,1105,1,553,21102,1,32,1,21102,1,1041,2,21102,313,1,0,1105,1,1301,21102,320,1,0,1105,1,1337,21101,0,327,0,1106,0,1279,2101,0,1,749,21102,65,1,2,21102,1,73,3,21101,346,0,0,1106,0,1889,1206,1,367,1007,749,69,748,1005,748,360,1102,1,1,756,1001,749,-64,751,1105,1,406,1008,749,74,748,1006,748,381,1101,-1,0,751,1105,1,406,1008,749,84,748,1006,748,395,1102,1,-2,751,1105,1,406,21102,1100,1,1,21102,406,1,0,1106,0,1421,21101,32,0,1,21101,0,1100,2,21101,421,0,0,1106,0,1301,21102,1,428,0,1105,1,1337,21102,435,1,0,1105,1,1279,2102,1,1,749,1008,749,74,748,1006,748,453,1102,1,-1,752,1106,0,478,1008,749,84,748,1006,748,467,1101,0,-2,752,1106,0,478,21102,1168,1,1,21102,478,1,0,1106,0,1421,21102,485,1,0,1105,1,1337,21102,10,1,1,21102,1,1168,2,21101,0,500,0,1105,1,1301,1007,920,15,748,1005,748,518,21101,0,1209,1,21102,1,518,0,1106,0,1421,1002,920,3,529,1001,529,921,529,101,0,750,0,1001,529,1,537,101,0,751,0,1001,537,1,545,101,0,752,0,1001,920,1,920,1105,1,13,1005,755,577,1006,756,570,21101,1100,0,1,21102,1,570,0,1105,1,1421,21102,1,987,1,1106,0,581,21102,1001,1,1,21102,1,588,0,1105,1,1378,1102,1,758,593,1002,0,1,753,1006,753,654,21002,753,1,1,21102,1,610,0,1105,1,667,21101,0,0,1,21101,621,0,0,1105,1,1463,1205,1,647,21101,1015,0,1,21102,1,635,0,1105,1,1378,21101,0,1,1,21102,646,1,0,1106,0,1463,99,1001,593,1,593,1105,1,592,1006,755,664,1102,1,0,755,1105,1,647,4,754,99,109,2,1102,1,726,757,21202,-1,1,1,21101,0,9,2,21102,1,697,3,21101,692,0,0,1106,0,1913,109,-2,2106,0,0,109,2,1002,757,1,706,1202,-1,1,0,1001,757,1,757,109,-2,2105,1,0,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,255,63,127,191,95,159,223,0,62,233,226,140,100,155,42,220,103,186,234,122,200,126,188,156,58,76,231,228,70,181,163,203,114,118,117,49,92,213,85,252,60,198,183,139,254,177,87,229,184,98,168,141,185,77,238,153,251,244,247,239,86,214,178,249,142,253,99,53,215,158,230,227,175,115,205,120,123,236,125,162,179,212,68,116,54,71,78,121,59,221,124,102,243,51,119,47,154,137,46,196,218,170,235,241,84,197,113,187,166,172,39,93,216,202,232,246,138,136,79,109,242,199,173,207,171,190,206,94,143,204,250,245,157,69,248,167,189,35,219,222,61,57,34,56,55,108,101,110,201,217,38,106,111,182,107,43,237,174,169,152,50,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,20,73,110,112,117,116,32,105,110,115,116,114,117,99,116,105,111,110,115,58,10,13,10,87,97,108,107,105,110,103,46,46,46,10,10,13,10,82,117,110,110,105,110,103,46,46,46,10,10,25,10,68,105,100,110,39,116,32,109,97,107,101,32,105,116,32,97,99,114,111,115,115,58,10,10,58,73,110,118,97,108,105,100,32,111,112,101,114,97,116,105,111,110,59,32,101,120,112,101,99,116,101,100,32,115,111,109,101,116,104,105,110,103,32,108,105,107,101,32,65,78,68,44,32,79,82,44,32,111,114,32,78,79,84,67,73,110,118,97,108,105,100,32,102,105,114,115,116,32,97,114,103,117,109,101,110,116,59,32,101,120,112,101,99,116,101,100,32,115,111,109,101,116,104,105,110,103,32,108,105,107,101,32,65,44,32,66,44,32,67,44,32,68,44,32,74,44,32,111,114,32,84,40,73,110,118,97,108,105,100,32,115,101,99,111,110,100,32,97,114,103,117,109,101,110,116,59,32,101,120,112,101,99,116,101,100,32,74,32,111,114,32,84,52,79,117,116,32,111,102,32,109,101,109,111,114,121,59,32,97,116,32,109,111,115,116,32,49,53,32,105,110,115,116,114,117,99,116,105,111,110,115,32,99,97,110,32,98,101,32,115,116,111,114,101,100,0,109,1,1005,1262,1270,3,1262,20102,1,1262,0,109,-1,2105,1,0,109,1,21102,1,1288,0,1105,1,1263,21002,1262,1,0,1102,1,0,1262,109,-1,2106,0,0,109,5,21101,1310,0,0,1106,0,1279,21202,1,1,-2,22208,-2,-4,-1,1205,-1,1332,22102,1,-3,1,21101,0,1332,0,1106,0,1421,109,-5,2106,0,0,109,2,21102,1346,1,0,1106,0,1263,21208,1,32,-1,1205,-1,1363,21208,1,9,-1,1205,-1,1363,1105,1,1373,21101,0,1370,0,1105,1,1279,1106,0,1339,109,-2,2105,1,0,109,5,1202,-4,1,1386,20102,1,0,-2,22101,1,-4,-4,21102,1,0,-3,22208,-3,-2,-1,1205,-1,1416,2201,-4,-3,1408,4,0,21201,-3,1,-3,1105,1,1396,109,-5,2105,1,0,109,2,104,10,21202,-1,1,1,21101,1436,0,0,1106,0,1378,104,10,99,109,-2,2105,1,0,109,3,20002,593,753,-1,22202,-1,-2,-1,201,-1,754,754,109,-3,2105,1,0,109,10,21102,1,5,-5,21102,1,1,-4,21102,0,1,-3,1206,-9,1555,21102,3,1,-6,21102,1,5,-7,22208,-7,-5,-8,1206,-8,1507,22208,-6,-4,-8,1206,-8,1507,104,64,1106,0,1529,1205,-6,1527,1201,-7,716,1515,21002,0,-11,-8,21201,-8,46,-8,204,-8,1105,1,1529,104,46,21201,-7,1,-7,21207,-7,22,-8,1205,-8,1488,104,10,21201,-6,-1,-6,21207,-6,0,-8,1206,-8,1484,104,10,21207,-4,1,-8,1206,-8,1569,21101,0,0,-9,1105,1,1689,21208,-5,21,-8,1206,-8,1583,21101,0,1,-9,1105,1,1689,1201,-5,716,1589,20101,0,0,-2,21208,-4,1,-1,22202,-2,-1,-1,1205,-2,1613,22101,0,-5,1,21101,1613,0,0,1105,1,1444,1206,-1,1634,22102,1,-5,1,21101,0,1627,0,1106,0,1694,1206,1,1634,21101,2,0,-3,22107,1,-4,-8,22201,-1,-8,-8,1206,-8,1649,21201,-5,1,-5,1206,-3,1663,21201,-3,-1,-3,21201,-4,1,-4,1105,1,1667,21201,-4,-1,-4,21208,-4,0,-1,1201,-5,716,1676,22002,0,-1,-1,1206,-1,1686,21101,0,1,-4,1106,0,1477,109,-10,2105,1,0,109,11,21101,0,0,-6,21101,0,0,-8,21102,0,1,-7,20208,-6,920,-9,1205,-9,1880,21202,-6,3,-9,1201,-9,921,1725,20101,0,0,-5,1001,1725,1,1732,21002,0,1,-4,21201,-4,0,1,21101,1,0,2,21102,9,1,3,21101,1754,0,0,1106,0,1889,1206,1,1772,2201,-10,-4,1767,1001,1767,716,1767,20101,0,0,-3,1106,0,1790,21208,-4,-1,-9,1206,-9,1786,22101,0,-8,-3,1106,0,1790,21202,-7,1,-3,1001,1732,1,1796,20101,0,0,-2,21208,-2,-1,-9,1206,-9,1812,21201,-8,0,-1,1106,0,1816,22102,1,-7,-1,21208,-5,1,-9,1205,-9,1837,21208,-5,2,-9,1205,-9,1844,21208,-3,0,-1,1105,1,1855,22202,-3,-1,-1,1105,1,1855,22201,-3,-1,-1,22107,0,-1,-1,1105,1,1855,21208,-2,-1,-9,1206,-9,1869,22102,1,-1,-8,1105,1,1873,21201,-1,0,-7,21201,-6,1,-6,1105,1,1708,21201,-8,0,-10,109,-11,2105,1,0,109,7,22207,-6,-5,-3,22207,-4,-6,-2,22201,-3,-2,-1,21208,-1,0,-6,109,-7,2106,0,0,0,109,5,1201,-2,0,1912,21207,-4,0,-1,1206,-1,1930,21101,0,0,-4,22102,1,-4,1,21201,-3,0,2,21101,1,0,3,21101,0,1949,0,1105,1,1954,109,-5,2106,0,0,109,6,21207,-4,1,-1,1206,-1,1977,22207,-5,-3,-1,1206,-1,1977,21201,-5,0,-5,1106,0,2045,21202,-5,1,1,21201,-4,-1,2,21202,-3,2,3,21101,1996,0,0,1106,0,1954,21202,1,1,-5,21101,0,1,-2,22207,-5,-3,-1,1206,-1,2015,21102,1,0,-2,22202,-3,-2,-3,22107,0,-4,-1,1206,-1,2037,21201,-2,0,1,21102,1,2037,0,106,0,1912,21202,-3,-1,-3,22201,-5,-3,-5,109,-6,2106,0,0];

static PART_ONE: [&str; 6] = [
  "OR A J",  // J = A
  "AND B J", // J = A & B
  "AND C J", // J = A & B & C
  "NOT J J", // J = !(A & B & C)
  "AND D J", // J = !(A & B & C) & D
  "WALK\n",
];

static PART_TWO: [&str; 9] = [
  "OR A J",  // J = A
  "AND B J", // J = A & B
  "AND C J", // J = A & B & C
  "NOT J J", // J = !(A & B & C)
  "AND D J", // J = !(A & B & C) & D
  "OR E T",  // T = E
  "OR H T",  // T = E | H
  "AND T J", // J = !(A & B & C) & D & (E | H)
  "RUN\n",
];

fn run_droid(insts: &[&str]) -> i64 {
  let mut cpu = IntCoder::new(&PROGRAM);
  cpu.push_str(&insts.join("\n"));
  let mut answer = 0;
  loop {
    match cpu.execute() {
      ExitCode::Output(o) => answer = o,
      ExitCode::Halted => return answer,
      ExitCode::AwaitInput => unreachable!(),
    }
  }
}

fn main() {
  let now = Instant::now();
  println!("Part one: {}", run_droid(&PART_ONE));
  println!("Part two: {}", run_droid(&PART_TWO));
  println!("Time: {}ms", now.elapsed().as_millis());
}
