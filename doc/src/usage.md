<!-- toc -->

# usage
[top](#toc)


## some design decisions

- we use ` ```mermaid ` tag, because we want the diagram to be visible outside of mdbook and the preprocessors, eg github, vscode, ...
so there is not a new tag, the animated diagrams will be inside block, starting with ` ```mermaid ` and ending with ` ``` ` .

- in mermaid, the informations between the `---` marks are a yaml document. If this document has the key `animate` or `animate-yml-file`, it will be handled by the mdbook-mermaid-animate preprocessor, if not it will be ignored.


## informations inline    

```

---

animate:
    delay: ...
    tags: ...
    variants: ...

---

body of the mermaid diagram definition

```


## informations in a separate file

```

---

animate-yml-file: some yml file

---

body of the mermaid diagram definition

```



# the yml document





