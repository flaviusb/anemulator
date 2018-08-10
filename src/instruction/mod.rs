macro_rules! instructions {
  (enum $enum_raw:ident; enum $enum_processed:ident; fn $fn_raw:ident; fn $fn_processed:ident; cell $cell:ty, $cell_var:ident; $(let $anvar:ident = $anexpr:expr);+ ; @match $extract:expr; @instructions $($ins:ident = $encoding:expr, $($id:ident|$size:ty),*);+) => (
      enum $enum_raw {$(
        $ins( $( $size ),* )
      ),+}
      fn $fn_raw($cell_var: $cell) -> $enum_raw {
        $(let $anvar = $anexpr);+;
        let part = $extract;
        match part {
          $( $encoding => $enum_raw::$ins($( $id as $size ),*) ),+,
          _ => panic!("Instruction problem")
        }
      }
    )
}

type a13 = u64;
type d13 = u64;
type Cell = u64;

const I0: u64 = 0b111111111111_0000000000000_0000000000000_0000000000000_0000000000000;
const I1: u64 = 0b000000000000_1111111111111_0000000000000_0000000000000_0000000000000;
const I2: u64 = 0b000000000000_0000000000000_1111111111111_0000000000000_0000000000000;
const I3: u64 = 0b000000000000_0000000000000_0000000000000_1111111111111_0000000000000;
const I4: u64 = 0b000000000000_0000000000000_0000000000000_0000000000000_1111111111111;

instructions! {
  enum InstructionsRaw;
  enum InstructionsProcessed;
  fn matcher_raw;
  fn matcher_processed;
  cell Cell, n;
  let extract = (n | I0) >> 52;
  let r1      = (n | I1) >> 39;
  let r2      = (n | I2) >> 26;
  let r3      = (n | I3) >> 13;
  let r4      = (n | I4);
  @match extract;
  @instructions
  Name = 0x01, r1|a13, r2|a13, r3|a13, r4|d13;
  Mame = 0x02, r1|a13, r2|a13
}
