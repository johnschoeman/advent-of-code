import {
  pipe,
  Option,
  ReadonlyArray,
  Number as NumberE,
  Effect,
  Match,
  String,
} from "effect"
import { InputProvider } from "../inputProvider"

const path = "src/day3/input.txt"

// --- Day 3: Gear Ratios ---
//
// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
//
// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
//
// "Aaah!"
//
// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
//
// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
//
// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
//
// Here is an example engine schematic:
//
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
//
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
//
// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
//

type Coord = [number, number]
const coord = (rowIdx: number, colIdx: number): Coord => [rowIdx, colIdx]

export type Schematic = ReadonlyArray<ReadonlyArray<string>>

export const toSchematic = (input: string[]): Schematic => {
  return pipe(input, ReadonlyArray.map(String.split("")))
}

const get =
  (matrix: Schematic) =>
  ([rowIdx, colIdx]: Coord): Option.Option<string> => {
    return pipe(
      matrix,
      ReadonlyArray.get(rowIdx),
      Option.flatMap(ReadonlyArray.get(colIdx)),
    )
  }

export const reduce =
  <T>(
    initialAcc: T,
    reducer: (acc: T, el: string, rowIdx: number, colIdx: number) => T,
  ) =>
  (schematic: Schematic): T => {
    return pipe(
      schematic,
      ReadonlyArray.reduce(initialAcc, (acc, row, rowIdx) => {
        return pipe(
          row,
          ReadonlyArray.reduce(acc, (innerAcc, col, colIdx) => {
            return reducer(innerAcc, col, rowIdx, colIdx)
          }),
        )
      }),
    )
  }

type PartNumber = {
  rowIdx: number
  colIdx: number
  length: number
  value: string
}

const partNumber = (
  rowIdx: number,
  colIdx: number,
  length: number,
  value: string,
) => {
  return {
    rowIdx,
    colIdx,
    length,
    value,
  }
}

type Digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
const allDigits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]

const isDigit = (str: string): str is Digit => {
  return allDigits.includes(str)
}

type NumberChr = {
  _tag: "Number"
  digit: Digit
  rowIdx: number
  colIdx: number
}

type SymbolChr = {
  _tag: "Symbol"
  rowIdx: number
  colIdx: number
}

type DotChr = {
  _tag: "Dot"
  rowIdx: number
  colIdx: number
}

type Chr = NumberChr | SymbolChr | DotChr

const parseChr = (rowIdx: number, colIdx: number, chr: string): Chr => {
  if (isDigit(chr)) {
    return { _tag: "Number", digit: chr, rowIdx, colIdx }
  }
  if (isSymbol(chr)) {
    return { _tag: "Symbol", rowIdx, colIdx }
  }
  return { _tag: "Dot", rowIdx, colIdx }
}

export const isSymbol = (chr: string): boolean => {
  const regEx = /[\d\.]/g
  return pipe(chr, String.match(regEx), Option.isNone)
}

const appendPartNumber =
  (numberChr: NumberChr) =>
  (partNumber: PartNumber): PartNumber => {
    const { rowIdx, colIdx, length, value } = partNumber
    const { digit } = numberChr

    return {
      rowIdx,
      colIdx,
      length: length + 1,
      value: value + digit,
    }
  }

type FoundPartNumbers = PartNumber[]
type InprogressPartNumber = Option.Option<PartNumber>
type PartNumberAcc = [FoundPartNumbers, InprogressPartNumber]
export const partNumbers = (
  schematicRow: readonly string[],
  rowIdx: number,
): PartNumber[] => {
  return pipe(
    schematicRow,
    ReadonlyArray.reduce<PartNumberAcc, string>(
      [[], Option.none()],
      (acc, el, colIdx): PartNumberAcc => {
        const [foundNumbers, buffer] = acc

        const chr = parseChr(rowIdx, colIdx, el)

        return pipe(
          Match.type<Chr>(),
          Match.tag("Number", (chr): PartNumberAcc => {
            const nextNumber = pipe(
              buffer,
              Option.map(appendPartNumber(chr)),
              Option.getOrElse(() => partNumber(rowIdx, colIdx, 1, chr.digit)),
            )
            return [foundNumbers, Option.some(nextNumber)]
          }),
          Match.orElse((): PartNumberAcc => {
            return pipe(
              buffer,
              Option.map<PartNumber, PartNumberAcc>(partNumber => {
                return [[...foundNumbers, partNumber], Option.none()]
              }),
              Option.getOrElse<PartNumberAcc>(() => [
                foundNumbers,
                Option.none(),
              ]),
            )
          }),
        )(chr)
      },
    ),
    ([partNumbers, maybeLastPartNumber]) => {
      if (Option.isSome(maybeLastPartNumber)) {
        return [...partNumbers, maybeLastPartNumber.value]
      } else {
        return partNumbers
      }
    },
  )
}

export const surroundingCoords = (partNumber: PartNumber): Coord[] => {
  const { rowIdx, colIdx, value } = partNumber
  const length = String.length(value)
  const rowIds = ReadonlyArray.range(rowIdx - 1, rowIdx + 1)
  const colIds = ReadonlyArray.range(colIdx - 1, colIdx + length)
  return pipe(
    rowIds,
    ReadonlyArray.flatMap(rowId => {
      return pipe(
        colIds,
        ReadonlyArray.map(colId => coord(rowId, colId)),
      )
    }),
  )
}

const hasAdjacentSymbol =
  (schematic: Schematic) =>
  (partNumber: PartNumber): boolean => {
    return pipe(
      partNumber,
      surroundingCoords,
      ReadonlyArray.map(get(schematic)),
      ReadonlyArray.some(maybeEl => {
        return pipe(
          maybeEl,
          Option.map(isSymbol),
          Option.getOrElse(() => false),
        )
      }),
    )
  }

export const sumPartNumbers = (lines: string[]): number => {
  const schematic = toSchematic(lines)

  return pipe(
    schematic,
    ReadonlyArray.flatMap(partNumbers),
    ReadonlyArray.filter(hasAdjacentSymbol(schematic)),
    ReadonlyArray.map(({ value }) => Number(value)),
    NumberE.sumAll,
  )
}

export const program = InputProvider.pipe(
  Effect.flatMap(inputProvider => {
    return inputProvider.get(path)
  }),
  Effect.map(sumPartNumbers),
)
