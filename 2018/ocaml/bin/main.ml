open Core
open Lib.DayOne
open Lib.Utils

let file = "../files/day1-numbers.txt"

let () =
  let lines = readFile file in
  let set = SI.empty in
  let sum: int = frequencyUntilRepeat set lines in 

  printf "Resulting frequency: %s\n" (Pervasives.string_of_int sum)