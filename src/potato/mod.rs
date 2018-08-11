/*
 * The Potato Chip is the 'primary' kind of core in the fantasy system.
 * It has no registers, 64KB of scratch memory as a 13 bit address space arranged in 64 bit cells, a 64 bit instruction size, and 0-4 addresses per instruction.
 */


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
  Nop      = 0x00, { panic!("Nop"); };
  // Arithmetic operators: <Op><Int/...><Unsigned/Signed><bits><Modular/Saturating><packing or simd formula if any?>
  AddIU64M = 0x01, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("AddIU64M"); };
  SubIU64M = 0x02, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("SubIU64M"); };
  MulIU64M = 0x03, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("MulIU64M"); };
  DivIU64M = 0x04, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("DivIU64M"); };
  AddIU64S = 0x05, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("AddIU64S"); };
  SubIU64S = 0x06, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("SubIU64S"); };
  MulIU64S = 0x07, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("MulIU64S"); };
  DivIU64S = 0x08, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("DivIU64S"); };
  AddIS64M = 0x09, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("AddIS64M"); };
  SubIS64M = 0x0A, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("SubIS64M"); };
  MulIS64M = 0x0B, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("MulIS64M"); };
  DivIS64M = 0x0C, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("DivIS64M"); };
  AddIS64S = 0x0D, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("AddIS64S"); };
  SubIS64S = 0x0E, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("SubIS64S"); };
  MulIS64S = 0x0F, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("MulIS64S"); };
  DivIS64S = 0x10, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|a13|Cell { panic!("DivIS64S"); };
  // Shifts: Sh<L/R><A/L><bits>
  ShLA64   = 0x11, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell { panic!("Shift") };
  ShRA64   = 0x12, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell { panic!("Shift") };
  ShLL64   = 0x13, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell { panic!("Shift") };
  ShRL64   = 0x14, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell { panic!("Shift") };
  // Rotates: Ro<L/R><bits>
  RoL64    = 0x15, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell { panic!("Rotate") };
  RoR64    = 0x16, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell { panic!("Rotate") };
  // Bit Ops
  // Rank <Leading/Trailing> <Immediate?>
  RankL    = 0x17, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell { panic!("Rank") };
  RankT    = 0x18, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell { panic!("Rank") };
  RankLi   = 0x19, r1|v13|v13,  r2|a13|Cell, r3|a13|Cell { panic!("Rank") };
  RankTi   = 0x1A, r1|v13|v13,  r2|a13|Cell, r3|a13|Cell { panic!("Rank") };
  // Select <Leading/Trailing> <Immediate?>
  SelectL    = 0x1B, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell { panic!("Select") };
  SelectT    = 0x1C, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell { panic!("Select") };
  SelectLi   = 0x1D, r1|v13|v13,  r2|a13|Cell, r3|a13|Cell { panic!("Select") };
  SelectTi   = 0x1E, r1|v13|v13,  r2|a13|Cell, r3|a13|Cell { panic!("Select") };


  // Swap - not sure this is really needed, but leave it in for now
  Swap       = 0x99, r1|a13|Cell, r2|a13|Cell { panic!("Swap.") };
}
