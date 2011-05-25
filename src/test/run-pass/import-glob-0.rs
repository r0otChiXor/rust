import module_of_many_things::*;
import dug::too::greedily::and::too::deep::*;

mod module_of_many_things {
  export f1;
  export f2;
  export f4;
  fn f1() {
    log "f1";
  }
  fn f2() {
    log "f2";
  }
  fn f3() {
    log "f3";
  }
  fn f4() {
    log "f4";
  }
}

mod dug {
  mod too {
    mod greedily {
      mod and {
        mod too {
          mod deep {
            fn nameless_fear() {
              log "Boo!";
            }
            fn also_redstone() {
              log "Whatever.";
            }
          }
        }
      }
    }
  }
}


fn main() {
  f1();
  f2();
  f4();
  nameless_fear();
  also_redstone();
}