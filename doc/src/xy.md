


---

# chart

```mermaid

---

title: blah
animate:
  delay: 500
  tags: [ scenario ]
  variants:
    - name: v2000
      value: |
        title "Sales Revenue year 2000"
        line [5000, 6000, 7500, 8200, 9500, 10500, 11000, 10200, 9200, 8500, 7000, 6000]
        bar [5000, 6000, 7500, 8200, 9500, 10500, 11000, 10200, 9200, 8500, 7000, 6000]
    - name: v2020
      value: |
        title "Sales Revenue year 2020"
        line [7300, 6300, 7500, 8200, 9500, 10500, 9000, 9200, 9200, 8500, 7300, 6300]
        bar [7300, 6300, 7500, 8200, 9500, 10500, 9000, 9200, 9200, 8500, 7300, 6300]



  frames:
    - title: v2000
      toggles: [ scenario@v2000 ]
    - title: v2020
      toggles: [ scenario@v2020 ]

---

xychart
    x-axis [jan, feb, mar, apr, may, jun, jul, aug, sep, oct, nov, dec]
    y-axis "Revenue (in $)" 4000 --> 11000
    %% bar [5000, 6000, 7500, 8200, 9500, 10500, 11000, 10200, 9200, 8500, 7000, 6000]
    %% mermaid-animate-tag scenario


``` 