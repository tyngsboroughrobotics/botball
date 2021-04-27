# botball

`botball` is the Tyngsborough High School Robotics Team's library for controlling KIPR Wombat controllers in Botball competitions. It features an easy-to-use DSL for controlling motors, servos and sensors, and organizes your strategy into steps.

`botball` is written in the [Wipple programming language](https://wipple.gramer.dev).

## Installation

Add to your `project.wpl`'s `dependencies`:

```wipple
botball : git "https://github.com/tyngsboroughrobotics/botball"
```

## Example

```wipple
use botball

arm-claw : servo {
    port : 0
    on : 1200
    off : 1900
}

backside-claw : servo {
    port : 3
    on : 1400
    off : 700
}

...

step "reset everything" {
    turn arm-claw on
    turn backside-claw on
    ...
}

step "face white cylinder" {
    turn wheels left 25 deg
    ...
}

...
```

## Documentation

Work in progress!
