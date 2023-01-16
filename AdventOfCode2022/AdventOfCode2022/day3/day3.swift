//
// Created by Do Vien on 06/01/2023 at 11:41.
//
// Copyright © 2023 CPea2506. All rights reserved.
//

// MARK: - Day3

struct Day3 {
    private var rucksacks: [Substring] = []
    var day: UInt8 = 3
}

// MARK: Advent

extension Day3: Advent {
    typealias Result = UInt16

    init(data: String) {
        rucksacks = data.lines()
    }

    func part1() -> Result {
        rucksacks
            .map {
                let halfLength = $0.count / 2
                let itemOccurrence = $0.prefix(halfLength).elementOccurrence
                let commonItem = $0.suffix(halfLength).first { itemOccurrence[$0] != nil }

                return Result(commonItem!.priority)
            }
            .reduce(0, +)
    }

    func part2() -> Result {
        var commonItems = Set<Character>()

        return rucksacks
            .enumerated()
            .reduce(0) {
                if $1.offset % 3 == 0 {
                    commonItems = $1.element.charSet

                    return $0
                }

                commonItems = commonItems.intersection($1.element.charSet)

                if commonItems.count == 1, let commonItem = commonItems.first {
                    return $0 + Result(commonItem.priority)
                }

                return $0
            }
    }
}
