import { pipe, ReadonlyArray, Number as NumberE, Effect, String } from "effect"
import { InputProvider } from "../inputProvider"

// Part 1
//
// The newly - improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover.On each line, the calibration value can be found by combining the first digit and the last digit(in that order) to form a single two - digit number.
//
// For example:
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
//
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
//
// Consider your entire calibration document. What is the sum of all of the calibration values?

const calculateTotal = (lines: string[]) => {
  return pipe(lines, ReadonlyArray.map(trebuchet), NumberE.sumAll)
}

export const trebuchet = (src: string): number => {
  return pipe(
    src,
    String.replaceAll(/\D/g, ""),
    digits => {
      const left = String.takeLeft(1)(digits)
      const right = String.takeRight(1)(digits)
      return String.concat(left, right)
    },
    Number,
  )
}

export const program = InputProvider.pipe(
  Effect.flatMap(inputProvider => {
    const path = "src/day1/input.txt"
    return inputProvider.get(path)
  }),
  Effect.map(calculateTotal),
)
