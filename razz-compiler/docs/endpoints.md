# List of Endpoints user can access in this programming language 

## Methods 

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
    top: Vec3, 
    bottom: Vec3
}
```

### POST 
- `/sphere` 

### PUT 

### PATCH 


### DELETE 
Coming soon...
