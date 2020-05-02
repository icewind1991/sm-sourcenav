# sm-sourcenav

Sourcemod extension for parsing .nav files 

## Usage

Get the z-height at an x/y point

```sourcepawn
#include <sourcenav>

new SourceNav:sourceNav = new SourceNav("path/to/nav.nav");
float z = sourceNav.query(x, y, z_guess); // z_guess is used when there are multiple possible heights
```
 
