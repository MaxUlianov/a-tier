## item title

lines above, lines below (lines above = square size -  text lines div 2; 
lines below = square size - lines above - text lines = either lines above or +1)

for printing in block:
we prioritize less space on left, top – in case of odd numbers, +1 goes right, bottom

## config
### file config format

pass config file
`-- conf file_path`

```
[config]
length=??
height=??

[tiers]
tier_name=color
```

length, height = if not present, set as None and use default
color = (check which colors console supports)
