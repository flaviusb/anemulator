macro_rules! instructions {
  (enum $enum:ident; fn $fn:ident; cell $cell:ty, $cell_var:ident; extract $extract:expr; @instructions $($ins:ident = $encoding:expr, $($size:ty),*);+) => (
      enum $enum {$(
        $ins( $( $size ),* )
      ),+}
    )
}

type a13 = u32;
type d13 = u32;
type Cell = u64;

instructions! {
  enum Instructions;
  fn match;
  cell Cell, n;
  extract (n || 0b111111111111);
  @instructions
  Name = 0x01, a13, a13, a13, d13;
  Mame = 0x02, a13, a13
}
