//
// Created by Do Vien on 24/12/2022 at 05:11.
//
// Copyright © 2022 CPea2506. All rights reserved.
//

// MARK: - Day1

struct Day1 {
    var calories: [UInt] = []
    var day: UInt8 = 1
}

// MARK: Avent

extension Day1: Avent {
    // MARK: Lifecycle

    init(data: String) {
        let components = data.components(separatedBy: .newlines)
        var sum: UInt = 0

        for calory in components {
            if calory.isEmpty {
                calories.append(sum)
                sum = 0
                continue
            }

            sum += UInt(calory)!
        }
    }

    // MARK: Internal

    func part1() -> UInt {
        calories.max()!
    }

    func part2() -> UInt {
        0
    }
}
