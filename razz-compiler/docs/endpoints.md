# Endpoints
A list of endpoints user can access in Razz Programming Language

## Methods 
REST-like methods built-in the language

### GET 
Returns the scene object at its current state. 
Default values are set by the renderer on startup. 
Modifications are not reflected until a PATCH or PUT is issued.

- `/camera`: Retrieve the camera object from the scene, the object interface is 
```
Camera {
    lookfrom: Point3; 
    lookat: Point3; 
    vfov: float; 
    vup: Vec3; 
    focus_dist: float; 
    defocus_angle: float; 
}
```
- `/image`: Retrieve the image target to be rendered, the interface 
```
Image {
    width: int; 
    height: int; 
}
```
- `/background`: Retrieve the background interface with the structure 
```
Background {
    top: Vec3; 
    bottom: Vec3;
}
```

- `/output`: Retrieve current output setup 
```
Output {
    type: OutputType; 
    file: string?; 
}
```

### POST 
- `/sphere`: Create a sphere, the argument must be 
```
Sphere {
    coord: Vec3; 
    radius: float; 
    material: Material;
}
```

### PUT 
- `/camera`: Change the entire camera object, given the interface 
```
Camera {
    lookfrom: Point3; 
    lookat: Point3; 
    vfov: float; 
    vup: Vec3; 
    focus_dist: float; 
    defocus_angle: float; 
}
```

- `/background`: Change the entire background color with the interface
```
Background {
    top: Vec3; 
    bottom: Vec3;
}
```

- `/image`: Change the entire image structure 
```
Image {
    width: int; 
    height: int; 
}
```

- `/output`: Change the output format 
```
Output {
    type: OutputType; 
    file: string?; 
}
```

### PATCH 
- `/camera`: Update the camera object, given the interface 
```
Camera {
    lookfrom: Point3?; 
    lookat: Point3?; 
    vfov: float?; 
    vup: Vec3?; 
    focus_dist: float?; 
    defocus_angle: float?; 
}
```

- `/background`: Update the background color with the interface
```
Background {
    top: Vec3?; 
    bottom: Vec3?;
}
```

- `/image`: Update the image structure 
```
Image {
    width: int?; 
    height: int?; 
}
```

- `/output`: Update the output format 
```
Output {
    type: OutputType?; 
    file: string?; 
}
```

### DELETE 
Coming soon...
