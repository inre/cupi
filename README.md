# CuPi - Cuprum Pi

Cuprum Pi is a GPIO access library written on Rust for the Raspberry Pi.

## Overview

*Supported boards*

* Raspberry Pi A, B
* Raspberry Pi A+, B+
* Raspberry Pi 2

*Features*

| Mode     | Pin r/w | Pull Up/Dw |  Trigger   | Pin alt |
| -------- | :-----: | :--------: | :--------: | :-----: |
| Direct   |    +    |     +      |    -       |  soon   |
| Sys mode |    +    |     -      |   +(epoll) |    -    |

*Pinout*

[image](http://pi4j.com/images/j8header-2b-large.png)
