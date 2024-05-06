# What is this
I was very disappointed when I realized that this doesn't work:
```cmatrix | lolcat```

Thus I recreated cmatrix with custom color trails.

# How to build it
No special build options are used for building this. Just use cargo

# How to use it
Run the program in a terminal that supports ANSI 265 colors.

If you want to you can pass it the path to a file containing a different color scheme.
This file must be a series of 8-bit numbers separated by new line characters.
These numbers will be interpreted as ANSI 256 colors.

```trans_colors.txt``` is such a file containing a set of colors inspired by the trans pride flag.

Any more arguments passed to the program will be ignored.
