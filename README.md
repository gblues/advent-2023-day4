## Advent of Code 2023: Day 4

Puzzle URL: https://adventofcode.com/2023/day/4

### Building

1. `mkdir -p src/input`
2. Save your puzzle input as `src/input/4.txt`
3. build the project

I have the solutions for each part of the puzzle as separate commits.

### Design

Bear in mind that, while I have experience with a number of different languages,
I just started learning Rust via trying to solve these `Advent of Code` puzzles.

#### Part 1

The puzzle requires us to count the number of gameplay numbers are in the
provided winning set of numbers, and assign a score that doubles with each match,
starting at 1.

Put in more mathematical terms, we need to calculate the intersection of two sets
and then raise 2 to the (intersection size - 1) power.

My solution works as follows:

1. define a regular expression pattern to capture the ticket number, winning number
   set, and card number sets into capture groups.
2. Put the puzzle input through the pattern and for each match:
   1. parse each set string into an actual Rust set object
   2. calculate the intersection, get the size, and raise it to (size-1)
   3. add that to the total
3. Print out the total

#### Part 2
The puzzle text describes that we're not supposed to get points after all, it's actually
a number of copies of subsequent tickets. The main thing is that as we're resolving this
ticket explosion, we're always moving towards the end of the table--once the first ticket
is processed, we'll never see ticket 1 again,

I adapted the Step 1 algorithm as follows:

1. The parsing loop now simply resolves to a list of match counts.
2. Create a second list of integers that start as 1. This list will keep track of
   how many copies we have of that ticket.
3. iterate over the list of match counts, and increment the counts for cards
   i+1..i+count once for each copy of the card we have.
4. now calculate a sum of the card counts list and we're done.