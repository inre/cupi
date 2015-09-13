# CuPi - Cuprum Pi

Cuprum Pi is a GPIO access library written on Rust for the Raspberry Pi.

[![Build Status](https://travis-ci.org/cuprumpi/cupi.svg?branch=master)](https://travis-ci.org/cuprumpi/cupi)
[![crates.io](http://meritbadge.herokuapp.com/cupi)](https://crates.io/crates/cupi)

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

*API Documentation*

[doc](http://cuprumpi.github.io/cupi/cupi/index.html)
