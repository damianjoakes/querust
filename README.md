# ApponoDB
<sup><i>appono - put/apply/add to</i></sup>

## Installation
```sh
cargo add appono-db
```

## Purpose
To create a fast, Rust-focused library for creating and maintaining various databases. 
Additionally, these databases should be capable of intercommunication between others by encoding information to 
MessagePack. 

## Why ApponoDB?
The options for a mature database driver for use in Rust are limited. Many of these crates tend to be underdeveloped, 
require external code, or in general do not feel like they were developed with Rust programmers in mind.

ApponoDB aims to address these issues by being a library that allows for programmers to create performant relational
and as well as non-relational databases. This library aims to have a low-level, high performance core that is abstracted
by a high-level interface for easy management.

## Relational vs. Non-Relational
ApponoDB offers both relational and non-relational database management by offering different APIs for setting up either
simple document-based databases, or complex relational databases using linked tables, foreign keys, and junction tables.