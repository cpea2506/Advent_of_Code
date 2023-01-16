//
// Created by Do Vien on 24/12/2022 at 05:11.
//
// Copyright © 2022 CPea2506. All rights reserved.
//

// MARK: - Day1

struct Day1 {
    private var calories: [Result] = []
    var day: UInt8 = 1
}

// MARK: Advent

extension Day1: Advent {
    // MARK: Lifecycle

    typealias Result = UInt32

    init(data: String) {
        var totalElveCalories: Result = 0

        // we need the line terminator so we only need one condition
        for calory in data.lines(omittingEmptySubsequences: false) {
            if calory.isEmpty {
                calories.append(totalElveCalories)
                totalElveCalories = 0
                continue
            }

            // SAFETY: calory always has value and is a number
            totalElveCalories += Result(calory)!
        }
    }

    // MARK: Internal

    func part1() -> Result {
        // SAFETY: calories cannot be empty
        calories.max()!
    }

    func part2() -> Result {
        calories.kNearest(3, by: >).reduce(0, +)
    }
}
