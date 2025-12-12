import { expect, test } from "vitest"
import { Effect } from "effect"
import {
  surroundingCoords,
  reduce,
  isSymbol,
  toSchematic,
  program,
} from "./part2"
import { InputProvider } from "../inputProvider"

// Part 2

test("surroundingCoords", () => {
  const gear = { _tag: "Gear", rowIdx: 1, colIdx: 1 }

  const expected = [
    [0, 0],
    [0, 1],
    [0, 2],
    [1, 0],
    [1, 1],
    [1, 2],
    [2, 0],
    [2, 1],
    [2, 2],
  ]

  const result = surroundingCoords(gear)

  expect(result).toEqual(expected)
})

test("program", async () => {
  const runnable = Effect.provideService(
    program,
    InputProvider,
    InputProvider.of({
      get: () =>
        Effect.succeed([
          "467..114..",
          "...*......",
          "..35..633.",
          "......#...",
          "617*......",
          ".....+.58.",
          "..592.....",
          "......755.",
          "...$.*....",
          ".664.598..",
        ]),
    }),
  )

  const expected = 467835

  expect(Effect.runSync(runnable)).toBe(expected)
})
