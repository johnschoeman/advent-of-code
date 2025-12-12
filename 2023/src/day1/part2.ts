import {
  pipe,
  Option,
  ReadonlyArray,
  Number as NumberE,
  Effect,
  String,
} from "effect"
import { InputProvider } from "../inputProvider"

// Part 2
//
// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
//
// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
//
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
//
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
//
// What is the sum of all of the calibration values?

const calculateTotal = (lines: string[]) => {
  return pipe(lines, ReadonlyArray.map(trebuchet), NumberE.sumAll)
}

export const trebuchet = (src: string): number => {
  return Number(leftNumber(src) + rightNumber(src))
}

type ResultAcc = [Option.Option<string>, string]
type ConcatFn = (a: string, b: string) => string
type ReduceFn = typeof ReadonlyArray.reduce

export const findNumber =
  (concatFn: ConcatFn, reduceFn: ReduceFn) =>
  (src: string): string => {
    return pipe(
      src,
      reduceFn<ResultAcc, string>([Option.none(), ""], (acc, chr) => {
        const [prevResult, prevString] = acc
        if (Option.isSome(prevResult)) {
          return acc
        }

        const nextString = concatFn(prevString, chr)

        return pipe(
          nextString,
          replaceNumber,
          String.replaceAll(/\D/g, ""),
          checkResult,
          nextResult => [nextResult, nextString],
        )
      }),
      ([resultOption, _acc]) => {
        return Option.getOrElse(() => "")(resultOption)
      },
    )
  }

const replaceNumber = (src: string): string => {
  return pipe(
    src,
    String.replace("one", "1"),
    String.replace("two", "2"),
    String.replace("three", "3"),
    String.replace("four", "4"),
    String.replace("five", "5"),
    String.replace("six", "6"),
    String.replace("seven", "7"),
    String.replace("eight", "8"),
    String.replace("nine", "9"),
  )
}

const checkResult = (maybeDigit: string): Option.Option<string> => {
  if (String.isNonEmpty(maybeDigit)) {
    return Option.some(maybeDigit)
  } else {
    return Option.none()
  }
}

export const leftNumber = findNumber((a, b) => a + b, ReadonlyArray.reduce)

export const rightNumber = findNumber(
  (a, b) => b + a,
  ReadonlyArray.reduceRight,
)

export const program = InputProvider.pipe(
  Effect.flatMap(inputProvider => {
    const path = "src/day1/input.txt"
    return inputProvider.get(path)
  }),
  Effect.map(calculateTotal),
)
