import {
  pipe,
  Option,
  ReadonlyArray,
  Number as NumberE,
  Effect,
  Match,
  SortedSet,
  String,
  Order,
} from "effect"
import { InputProvider } from "../inputProvider"

const path = "src/day3/input.txt"

// --- Part Two ---
//
// The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.
//
// You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.
//
// Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.
//
// The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.
//
// This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.
//
// Consider the same engine schematic again:
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
// In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

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
export const partNumbersInLine = (
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

export const surroundingCoords = (gear: Gear): Coord[] => {
  const { rowIdx, colIdx } = gear
  const rowIds = ReadonlyArray.range(rowIdx - 1, rowIdx + 1)
  const colIds = ReadonlyArray.range(colIdx - 1, colIdx + 1)

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

type Gear = {
  _tag: "Gear"
  rowIdx: number
  colIdx: number
}

const gear = (rowIdx: number, colIdx: number): Gear => {
  return {
    _tag: "Gear",
    rowIdx,
    colIdx,
  }
}

const isGear = (str: string): boolean => {
  return str === "*"
}

const findGears = (schematic: Schematic): Gear[] => {
  return pipe(
    schematic,
    reduce<Gear[]>([], (acc, el, rowIdx, colIdx) => {
      if (isGear(el)) {
        return [...acc, gear(rowIdx, colIdx)]
      } else {
        return acc
      }
    }),
  )
}

const findPartNumbers = (schematic: Schematic): PartNumber[] => {
  return pipe(schematic, ReadonlyArray.flatMap(partNumbersInLine))
}

const partNumberOrder: Order.Order<PartNumber> = Order.make((a, b) => {
  if (a.rowIdx < b.rowIdx) {
    return -1
  }
  if (a.rowIdx > b.rowIdx) {
    return 1
  }
  if (a.colIdx < b.colIdx) {
    return -1
  }
  if (a.colIdx > b.colIdx) {
    return 1
  }
  return 0
})

const partHasCoord =
  (coord: Coord) =>
  (partNumber: PartNumber): boolean => {
    const [rowIdx, colIdx] = coord

    return (
      rowIdx === partNumber.rowIdx &&
      partNumber.colIdx <= colIdx &&
      partNumber.colIdx + partNumber.length - 1 >= colIdx
    )
  }

const surroundingPartNumbers =
  (partNumbers: PartNumber[]) =>
  (gear: Gear): PartNumber[] => {
    return pipe(
      gear,
      surroundingCoords,
      ReadonlyArray.reduce(SortedSet.empty(partNumberOrder), (acc, coord) => {
        return pipe(
          partNumbers,
          ReadonlyArray.findFirst(partHasCoord(coord)),
          Option.match({
            onSome: partNumber => SortedSet.add(partNumber)(acc),
            onNone: () => acc,
          }),
        )
      }),
      ReadonlyArray.fromIterable,
    )
  }

const isValidGear = (partNumbers: PartNumber[]): boolean => {
  return partNumbers.length === 2
}

export const sumGearRatios = (lines: string[]): number => {
  const schematic = toSchematic(lines)

  const gears = findGears(schematic)
  const partNumbers = findPartNumbers(schematic)

  return pipe(
    gears,
    ReadonlyArray.map(surroundingPartNumbers(partNumbers)),
    ReadonlyArray.filter(isValidGear),
    ReadonlyArray.map(ReadonlyArray.map(({ value }) => Number(value))),
    ReadonlyArray.map(NumberE.multiplyAll),
    NumberE.sumAll,
  )
}

export const program = InputProvider.pipe(
  Effect.flatMap(inputProvider => {
    return inputProvider.get(path)
  }),
  Effect.map(sumGearRatios),
)
