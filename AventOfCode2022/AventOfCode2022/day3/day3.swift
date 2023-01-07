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

// MARK: Avent

extension Day3: Avent {
    init(data: String) {
        rucksacks = data.split(whereSeparator: \.isNewline)
    }

    func part1() -> UInt {
        rucksacks
            .map {
                let halfLength = $0.count / 2
                let itemOccurrence = $0.prefix(halfLength).elementOccurrence
                let commonItem = $0.suffix(halfLength).first { itemOccurrence[$0] != nil }

                return UInt(commonItem!.priority)
            }
            .reduce(0, +)
    }

    func part2() -> UInt {
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
                    return $0 + UInt(commonItem.priority)
                }

                return $0
            }
    }
}
