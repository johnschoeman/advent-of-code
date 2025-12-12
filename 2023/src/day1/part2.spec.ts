import { expect, test } from "vitest"
import { Effect } from "effect"
import { trebuchet, leftNumber, rightNumber, program } from "./part2"
import { InputProvider } from "../inputProvider"

test("leftNumber", () => {
  const q1 = "two1nine"
  const q2 = "eightwothree"
  const q3 = "abcone2threexyz"
  const q4 = "xtwone3four"
  const q5 = "4nineeightseven2"
  const q6 = "zoneight234"
  const q7 = "7pqrstsixteen"

  const a1 = "2"
  const a2 = "8"
  const a3 = "1"
  const a4 = "2"
  const a5 = "4"
  const a6 = "1"
  const a7 = "7"

  expect(leftNumber(q1)).toBe(a1)
  expect(leftNumber(q2)).toBe(a2)
  expect(leftNumber(q3)).toBe(a3)
  expect(leftNumber(q4)).toBe(a4)
  expect(leftNumber(q5)).toBe(a5)
  expect(leftNumber(q6)).toBe(a6)
  expect(leftNumber(q7)).toBe(a7)
})

test("rightNumber", () => {
  const q1 = "two1nine"
  const q2 = "eightwothree"
  const q3 = "abcone2threexyz"
  const q4 = "xtwone3four"
  const q5 = "4nineeightseven2"
  const q6 = "zoneight234"
  const q7 = "7pqrstsixteen"

  const a1 = "9"
  const a2 = "3"
  const a3 = "3"
  const a4 = "4"
  const a5 = "2"
  const a6 = "4"
  const a7 = "6"

  expect(rightNumber(q1)).toBe(a1)
  expect(rightNumber(q2)).toBe(a2)
  expect(rightNumber(q3)).toBe(a3)
  expect(rightNumber(q4)).toBe(a4)
  expect(rightNumber(q5)).toBe(a5)
  expect(rightNumber(q6)).toBe(a6)
  expect(rightNumber(q7)).toBe(a7)
})

test("trebuchet", () => {
  const q1 = "two1nine"
  const q2 = "eightwothree"
  const q3 = "abcone2threexyz"
  const q4 = "xtwone3four"
  const q5 = "4nineeightseven2"
  const q6 = "zoneight234"
  const q7 = "7pqrstsixteen"

  const a1 = 29
  const a2 = 83
  const a3 = 13
  const a4 = 24
  const a5 = 42
  const a6 = 14
  const a7 = 76

  expect(trebuchet(q1)).toBe(a1)
  expect(trebuchet(q2)).toBe(a2)
  expect(trebuchet(q3)).toBe(a3)
  expect(trebuchet(q4)).toBe(a4)
  expect(trebuchet(q5)).toBe(a5)
  expect(trebuchet(q6)).toBe(a6)
  expect(trebuchet(q7)).toBe(a7)
})

test("program", async () => {
  const runnable = Effect.provideService(
    program,
    InputProvider,
    InputProvider.of({
      get: () =>
        Effect.succeed([
          "two1nine",
          "eightwothree",
          "abcone2threexyz",
          "xtwone3four",
          "4nineeightseven2",
          "zoneight234",
          "7pqrstsixteen",
        ]),
    }),
  )

  const expected = 281
  expect(Effect.runSync(runnable)).toBe(expected)
})
