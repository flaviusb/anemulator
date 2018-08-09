macro_rules! instructions {
  (enum $enum:ident; fn $fn:ident; @instructions $($ins:ident = $encoding:expr, $($size:ty),*);+) => (
      enum $enum {$(
        $ins( $( $size ),* )
      ),+}
    )
}

type a13 = u32;
type d13 = u32;

instructions! {
  enum Instructions;
  fn match;
  @instructions
  Name = 0x01, a13, a13, a13, d13;
  Mame = 0x02, a13, a13
}
