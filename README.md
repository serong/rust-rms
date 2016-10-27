# Rust-RMS

Not a generic application, but just a simple CLI app for some school tasks where average voltage and voltage rms are calculated.

## Usage

For the following signals: 

[triangle-signal](trig.png)

```
rms -p "t,0,-3,25 t,-3,-4,25 t,-4,0,25 s,0,25"
```

Triangles are treated as special trapezoids.

```
p -> periodic
t -> triangle, trapezoid
s -> square
```

Calculations are done according to lecture slide that can be found [here](http://www.martinjott.ee/ttu/iss0050/m66tmine2013-6.pdf)