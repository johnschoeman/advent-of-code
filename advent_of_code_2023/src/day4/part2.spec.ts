import { expect, test } from "vitest"
import { Effect } from "effect"
import { program, parseCard, countMatching } from "./part2"
import { InputProvider } from "../inputProvider"

// Part 2

test("parseCard", () => {
  const input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

  const expected = [
    ["41", "48", "83", "86", "17"],
    ["83", "86", "6", "31", "17", "9", "48", "53"],
  ]

  const result = parseCard(input)

  expect(result).toEqual(expected)
})

test("countMatching", () => {
  const input1 = parseCard("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
  const input2 = parseCard("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")
  const input3 = parseCard("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")
  const input4 = parseCard("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")
  const input5 = parseCard("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
  const input6 = parseCard("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")

  const expected1 = 4
  const expected2 = 2
  const expected3 = 2
  const expected4 = 1
  const expected5 = 0
  const expected6 = 0

  const result1 = countMatching(input1)
  const result2 = countMatching(input2)
  const result3 = countMatching(input3)
  const result4 = countMatching(input4)
  const result5 = countMatching(input5)
  const result6 = countMatching(input6)

  expect(result1).toEqual(expected1)
  expect(result2).toEqual(expected2)
  expect(result3).toEqual(expected3)
  expect(result4).toEqual(expected4)
  expect(result5).toEqual(expected5)
  expect(result6).toEqual(expected6)
})

// Matches = [4,2,2,1,0,0]
//
// [1,1,1,1,1,1]
// [1,2,2,2,2,1]
// [1,2,4,4,2,1]
// [1,2,4,8,6,1]
// [1,2,4,8,14,1]
// [1,2,4,8,14,1]

test("program", async () => {
  const runnable = Effect.provideService(
    program,
    InputProvider,
    InputProvider.of({
      get: () =>
        Effect.succeed([
          // "Card 1: 83 | 83 86  6 31 17  9 48 53",
          // "Card 2: 1 | 61 30 68 82 17 32 24 19",

          // "Card 1: 83 86 | 83 86  6 31 17  9 48 53",
          // "Card 2: 61 | 61 30 68 82 17 32 24 19",
          // "Card 3: 44 | 69 82 63 72 16 21 14  1",

          "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
          "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
          "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
          "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
          "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
          "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]),
    }),
  )

  const expected = 30
  expect(Effect.runSync(runnable)).toBe(expected)
})
