# code-runner
personal code runner in Zig

## How to use

- If you're using your terminal 
  1. Include (...) in your path
  2. If you want to change default settings, use code-runner settings 
  2. If you want to run a project inside a directory, use code-runner runproject path/of/dir
  3. If you want to run a single file, use code-runner path/of/file
  4. If you want to run tests, use (...)

- If you're using VS Code extension:
  ...
  
- If you're using code runner's shell:
  1. Use code-runner startproject name/of/dir to start code-runner's virtual environment on that shell
  2. use 'run' to run project, 'bench' to benchmark it, 'debug' to debug it or 'test' to test it
  3. you can use 'run/test/bench/debug name/of/file' to run/test/... a particular file in that project
  4. (...)


Rust:

If cargo.toml exists:
  cargo run/test/bench
Else:
  rustc file && ./file


Python:
Automatically deduces what kind of project it's running (django, data science, ...)

If ...
  python main.py
Else:
  python file


C/C++:

Using cmake 

Not using cmake 


C#:


Zig:


Go:


Java:


Kotlin:


JS:
J
