# glore

## glore at log

**HowTo:**

1- Use `glore::GLORE` at the root of your project

2- Add a log target with `glore::init($target)` `$target` is anything that impl `Write`

3- log!

*example of usage:*

```rust
use glore::{init, log, GLORE};

let f = std::fs::OpenOptions::new()
	.append(true)
	.open("log.txt")
	.unwrap();
let stdout = std::io::stdout();

init(stdout);
log!("hello ====");
log!("world");

init(f);
log!("hello ====");
std::thread::spawn(|| {
	log!("world");
})
.join();
```

