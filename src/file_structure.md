# Generic package structure

```
Package/
 |- Cargo.lock
 |- Cargo.toml 
 |- src/ 
 |   |- lib.rs  <- Library crate
 |   |- main.rs  <- Binary crate
 |   |- some_mod.rs
 |   |- another_mod/
 |   |   |- mod.rs
 |   |   +- some_sub_mod.rs
 |   +- bin/  <- Binary crate
 |       |- another-executable.rs
 |       +- yet-another-executable.rs
 |           |- main.rs
 |           +- some_mod.rs
 |- benches/
 |   |- some-bench.rs
 |   +- another-bench/
 |       |- main.rs
 |       +- some_mod.rs
 |- examples/
 |   |- some-example.rs
 |   +- another-example/
 |       |- main.rs
 |       +- some_mod.rs
 +- tests/
     |- some-integration-test.rs
	 +- another-test/
         |- main.rs
		 +- some_mod.rs
 ```

 [also see](https://aloso.github.io/2021/03/28/module-system.html#the-module-tree)