# Theremotion

[![Github](https://img.shields.io/badge/github-plule%2Ftheremotion-8da0cb?style=flat-square)](https://github.com/plule/leaprs)

👐 Theremotion is a virtual instrument inspired by the Theremin.

Just like a Theremin, the position of your hand in the air controls the pitch
and volume of the sound. However, instead of being controlled with a pure analog
signal, the Theremotion is controled through digital hand tracking. This enables
additional dimensions to control the sound. With specific hand motions, you can
control a low pass filter, a guitar sound, make a chord. You can even make the
instrument snap to a scale, making it easier to control than a Theremin.

The Theremotion cannot replace a Theremin but it is intended to be a twist
around the idea of the Theremin that is more fun and maybe easier to learn. It
is however not as sensitive as a Theremin, and usual Theremin finger techniques
don't apply to the Theremotion.

<iframe width="560" height="315" src="https://www.youtube.com/embed/GGALeKm_uzc" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

⚠️ The motions have changed since this video, don't rely on it to learn how to
play.

## Features

![Screenshot](doc/capture.png)

- 👈 Move up left hand to control the volume.
- 👉 Move your right hand horizontally closer or further away from a virtual antenna to control the pitch.
- 👋 Move your left hand on the horizontal plane to control the filter.
- 👋 Move up your right hand to play a chord.
- 👌 Pinch with your right hand to snap to a scale.
- 🎸 Pinch with your left hand, and rotate it to play guitar.
- 👊 Close your right fist and rotate it to play a drone.

A left handed mode is also included.

## Installation

In order to play with Theremotion you need a [Leap Motion
Controller](https://www.ultraleap.com/product/leap-motion-controller/) and a
compatible Windows PC.

First of all, install the Gemini version of [Ultraleap's tracking
software](https://developer.leapmotion.com/tracking-software-download).

Lastly, download and install Theremotion from the link at the top of the page.
For alternative downloads, refer to the [latest release
page](https://github.com/plule/theremotion/releases/latest/).

## Tools

[![Leap Motion Controller](doc/ultraleap.png)](https://www.ultraleap.com/product/leap-motion-controller/) [![Faust](doc/faust.png)]((https://faust.grame.fr/)) [![Rust](doc/rust.png)](https://www.rust-lang.org)

The hand tracking is provided by Ultraleap's [Leap Motion Controller](https://www.ultraleap.com/product/leap-motion-controller/).

The sound generation is created with Grame's [Faust](https://faust.grame.fr/).

The program is built with [Rust](https://www.rust-lang.org).

