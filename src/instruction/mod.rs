macro_rules! instructions {
  (enum $enum_raw:ident; enum $enum_processed:ident; fn $fn_raw:ident; fn $fn_processed:ident; cell $cell:ty, $cell_var:ident; $(let $anvar:ident = $anexpr:expr);+ ; @match $extract:expr; @instructions $($ins:ident = $encoding:expr, $($id:ident|$size:ty|$processed_type:ty),*);+;) => (
      #[derive(Eq, PartialEq, Debug)]
      enum $enum_raw {$(
        $ins( $( $size ),* )
      ),+}
      #[derive(Eq, PartialEq, Debug)]
      enum $enum_processed {$(
        $ins( $( $processed_type ),* )
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
  enum InstructionsRaw;
  enum InstructionsProcessed;
  fn matcher_raw;
  fn matcher_processed;
  cell Cell, n;
  let extract = (n & I0);
  let r1      = (n & I1) >> 12;
  let r2      = (n & I2) >> 25;
  let r3      = (n & I3) >> 38;
  let r4      = (n & I4) >> 51;
  @match extract;
  @instructions
  Name = 0x01, r1|a13|Cell, r2|a13|Cell, r3|a13|Cell, r4|d13|Cell;
  Mame = 0x02, r1|a13|Cell, r2|a13|Cell;
  Fame = 0x03, r1|v13|v13, r2|a13|Cell;
}

#[test]
fn test_instruction_parsing() {
  let m: u64 = 0b0000000000011_0000000000000_0000000000000_0000000000000_000000000001;
  let n: u64 = 0b0000000000000_0000000000000_0000000000000_1000000000000_000000000010;
  assert_eq!(matcher_raw(m), InstructionsRaw::Name(0, 0, 0, 3));
  assert_eq!(matcher_raw(n), InstructionsRaw::Mame(0b1000000000000, 0));
}
