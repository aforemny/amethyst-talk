---
theme: gaia
_class: lead
---

# **An Introduction to the Amethyst Game Engine**

12.11.2019, Hannover

Alexander Foremny ([@aforemny](https://github.com/aforemny))

---

# Agenda

1. Why Rust, why Amethyst?
1. Amethyst concepts
1. Write pong
1. More examples
1. My game
1. Why not to use Amethyst?

Questions are always welcome!

---

# Amethyst – a data-driven game engine written in Rust

Why is Rust suited for games?

What are the features of Amethyst?

---

# Why is Rust suited for games?

- Statically typed means **fewer crashes**
- Statically typed means **easier to refactor**
- Free abstractions mean **high performance**
- Ownership means **no garbage-collector**
- Ownership means **high thread-safety**
- C FFI and LLVM backend mean **cross-plattform support**

---

# What are Amethysts features?

- Data-driven Entity Component System (ECS) with high performance
- Lean yet extensible foundation for both 2D and 3D content
- Free and open with permissive usage
- Current successes: Rendering, User Interface, Network, Audio, Asset and Prefab support

---

# Let's dive in!

We are going to build Pong from ground-up.

---

# Installation

```sh
$ rustc --version
rustc 1.38.0 (625451e37 2019-09-23)
```

```sh
$ cargo init
$ cargo install amethyst_tools
```

```sh
$ amethyst new pong
```

---

# Concepts

- **E**ntity
- **C**omponent
- **S**ystem
- Resource
- World
- System
- Bundle

---

# Entity Component System

An *entity* is just a unique identifier.

A *component* is any type of data that may be attached to an entity.

A *system* acts on components.

---

# Resource

Any type that stores data that is not a component.

---

# World

A collection of resources.

---

# Programming pong

- Set up the camera
- Draw the paddles
- Make the paddles move
- Draw the ball
- Make the ball move
- Make the ball bounce
- Winning rounds

---

# Setting up the camera

- We chose a 2D camera
- Our camera is centered on our *arena*
- Our arena is 100x100 units

---

# Draw the paddles

- We define the Paddle component
- We create two paddle entities
- We apply textures to them

---

# Make the paddles move

- We define our key bindings
- We make a system that moves the paddles

---

# Draw the ball

- We define the Ball component
- We create a ball
- We make a system that moves the ball

---

# Make the ball bounce

- We make a system that makes the ball bounce

---

# Outlook

- Winning rounds
- Resetting the game
- Keeping score
- Playing sounds
- Adding music

---

# More Amethyst examples

- https://github.com/amethyst/amethyst/tree/master/examples

---

# A sneak-peek at my game


---

# Why NOT to use Amethyst?

- Infancy of Rust and Amethyst
  - No AAA rendering (yet?)
  - No rich UI yet
  - No support for mobile or web yet
  - No editor (yet?)
  - No excellent documentation yet

---

# Questions

---

# Thank you!

- E-Mail: [aforemny@posteo.de](mailto:aforemny@posteo.de)
- [@aforemny](https://github.com/aforemny) on GitHub and Rust Slack

### Resources

- [Amethyst website](https://amethyst.rs/)
- [Amethyst documentation](https://amethyst.rs/doc)
- [Amethyst on GitHub](https://github.com/amethyst/amethyst)
