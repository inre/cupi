# CuPi - Cuprum Pi

Cuprum Pi is a GPIO access library written on Rust for the Raspberry Pi.

[![Build Status](https://travis-ci.org/inre/cupi.svg?branch=master)](https://travis-ci.org/inre/cupi)
[![crates.io](http://meritbadge.herokuapp.com/cupi)](https://crates.io/crates/cupi)

## Overview

*Supported boards*

* Raspberry Pi A, B
* Raspberry Pi A+, B+
* Raspberry Pi 2, 3, Zero

*Features*

| Mode     | Pin r/w | Pull Up/Dw |  Trigger   | Pin alt |
| -------- | :-----: | :--------: | :--------: | :-----: |
| Direct   |    +    |     +      |    -       |  soon   |
| Sys mode |    +    |     -      |   +(epoll) |    -    |

*Pinout*

[image](http://pi4j.com/images/j8header-2b-large.png)

*API Documentation*

[doc](http://inre.github.io/cupi/cupi/index.html)

*Cross Compiling for Raspberry Pi*

[instructions](https://github.com/Ogeon/rust-on-raspberry-pi)
