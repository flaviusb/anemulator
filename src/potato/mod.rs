
type a13 = usize;
type d13 = usize;
type v13 = u16;
type Cell = u64;

const I0: u64 = 0b0000000000000_0000000000000_0000000000000_0000000000000_111111111111;
const I1: u64 = 0b0000000000000_0000000000000_0000000000000_1111111111111_000000000000;
const I2: u64 = 0b0000000000000_0000000000000_1111111111111_0000000000000_000000000000;
const I3: u64 = 0b0000000000000_1111111111111_0000000000000_0000000000000_000000000000;
const I4: u64 = 0b1111111111111_0000000000000_0000000000000_0000000000000_000000000000;



instructions! {
  enum InstructionsDecode;
  enum InstructionsFetchRegisters;
  fn decode;
  fn fetch_μregisters;
  cell Cell, n;
  let extract = (n & I0);
  let r1      = (n & I1) >> 12;
  let r2      = (n & I2) >> 25;
  let r3      = (n & I3) >> 38;
  let r4      = (n & I4) >> 51;
  @match extract;
  @instructions
  Name = 0x00, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|d13|Cell { panic!("Name"); };
  Mame = 0x02, r1|a13|Cell, r2|a13|Cell { panic!("Mame"); };
  Fame = 0x03, r1|v13|v13, r2|a13|Cell { panic!("Fame"); };
}
