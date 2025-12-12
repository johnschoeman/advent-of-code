import { expect, test } from "vitest"
import { Effect } from "effect"
import {
  surroundingCoords,
  reduce,
  isSymbol,
  toSchematic,
  program,
} from "./part1"
import { InputProvider } from "../inputProvider"

// Part 1

test("isSymbol", () => {
  const input1 = "%"
  const input2 = "4"
  const input3 = "."

  const expected1 = true
  const expected2 = false
  const expected3 = false

  const result1 = isSymbol(input1)
  const result2 = isSymbol(input2)
  const result3 = isSymbol(input3)

  expect(result1).toEqual(expected1)
  expect(result2).toEqual(expected2)
  expect(result3).toEqual(expected3)
})

test("reduce", () => {
  const rawSchematic = ["01.", "..*"]

  const schematic = toSchematic(rawSchematic)

  const reducer = (
    acc: number[][],
    el: string,
    rowIdx: number,
    colIdx: number,
  ) => {
    if (el === ".") {
      return [...acc, [rowIdx, colIdx]]
    } else {
      return acc
    }
  }

  const expected = [
    [0, 2],
    [1, 0],
    [1, 1],
  ]

  const result = reduce([], reducer)(schematic)

  expect(result).toEqual(expected)
})

test("surroundingCoords", () => {
  const partNumber = { value: "12", rowIdx: 1, colIdx: 1, length: 12 }

  const expected = [
    [0, 0],
    [0, 1],
    [0, 2],
    [0, 3],
    [1, 0],
    [1, 1],
    [1, 2],
    [1, 3],
    [2, 0],
    [2, 1],
    [2, 2],
    [2, 3],
  ]

  const result = surroundingCoords(partNumber)

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

  const expected = 4361
  expect(Effect.runSync(runnable)).toBe(expected)
})
