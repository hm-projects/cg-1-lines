# Computational Geometry | Assignment 1: Lines

- Nicolas Bissig
- Antonino Grasso

## Quick Overview

Implementation: Rust

Quick summary: We test with the $ccw$ if two lines intersect, and for the colinear edge case, we use a simple overlap check

Results and runtime:
|Dataset|Amount of intersections|Runtime|
|---|---|---|
|s_1000_1.dat|11|~ 1 ms|
|s_10000_1.dat|733|~ 135 ms|
|s_100000_1.dat|77138|~ 15 s|

## Algorithm

main loop:

intersect:

ccw's:

overlap_for_collinear:

## Uniqueness of our solution

## Challenges

## Why is our solution correct?

## Bounding Box Test Early Return

(does not work as of now, because the overlap_for_colinear is strictly designed for colinear points)

