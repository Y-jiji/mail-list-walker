# Installation

First install rustup. [The rust-lang official website will be helpful](https://rust-lang.github.io/rustup/installation/index.html). 

In this directory, run: 

```
cargo install --path .
```

# Usage

```
email-list [config.json] [table.xlsx] [sheet-name]
```

An example of configuration json file can be found in `test-file/MAIL.json.template`. 

Variable names in curly bracket will be replaced by related columns in table.xlsx. 

For example, if the xlsx table looks like
```
----------------------------
   | name  | student number 
----------------------------
 1 | james | 40000
----------------------------
```

Then in the template file, after walking to row 1, `{name}` will be replaced by `james` and `{student number}` will be replaced by `40000`. 

We don't have escape mechanisms for this, just like println!() in rust. 
