# installation

## cargo
you need a working installation of `cargo`, the package manager for rust, follow instructions :

[https://doc.rust-lang.org/cargo/getting-started/installation.html](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## mdbook

you need mdbook 

```
cargo install mdbook
mdbook --version
```

this was tested with version `v0.5.2`





## mdbook-mermaid preprocessor

to let mdbook process the mermaid diagrams, you need the preprocessor for mdbook mermaid

```
cargo install mdbook-mermaid
mdbook-mermaid --version
# install the .js ressources
mdbook-mermaid install <path to your doc sources>
```

this was tested with version `0.17.0`

## mermaid.js version

`mdbook-mermaid` install will install `mermaid.js`, for `0.17.0` it will install `mermaid.js` version `11.6.0`, which does not support some mermaid features.

to upgrade, add : 
```sh
curl https://unpkg.com/mermaid@11.12.2/dist/mermaid.min.js --insecure -o <path to your doc sources>/mermaid.min.js
```



## mdbook-mermaid-animate
( this project )

to let mdbook process the mermaid diagrams in order to animate them, you need the preprocessor for mdbook mermaid animate
```
cargo install mdbook-mermaid-animate
mdbook-mermaid-animate install <path to your doc sources>
```

## write your documentation

follow mdbook guidelines to write the `.md` files of your documentation
Look at [this directory](https://github.com/laurentcarrie/animated-mermaid/tree/main/doc/src) as a working example



