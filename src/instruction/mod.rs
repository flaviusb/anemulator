macro_rules! instructions {
  (struct $enum:ident; fn $fn:ident; @instructions $($ins:ident = $encoding:expr, $($matchers:tt),*);+) => ()
}


instructions! {
  struct Instructions;
  fn match;
  @instructions
  name = 0x01, r1, r2, r3, r4;
  mame = 0x02, r2, r3
}
