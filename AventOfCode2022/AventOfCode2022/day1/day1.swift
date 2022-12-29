//
// Created by Do Vien on 24/12/2022 at 05:11.
//
// Copyright © 2022 CPea2506. All rights reserved.
//

// MARK: - Day1

struct Day1 {
    private var calories: [UInt] = []
    var day: UInt8 = 1
}

// MARK: Avent

extension Day1: Avent {
    // MARK: Lifecycle

    init(data: String) {
        var totalElveCalories: UInt = 0

        for calory in data.components(separatedBy: .newlines) {
            if calory.isEmpty {
                calories.append(totalElveCalories)
                totalElveCalories = 0
                continue
            }

            // SAFETY: calory is never a float
            totalElveCalories += UInt(calory)!
        }
    }

    // MARK: Internal

    func part1() -> UInt {
        // SAFETY: calories cannot be empty
        calories.max()!
    }

    func part2() -> UInt {
        calories.kNearest(3, by: >).reduce(0, +)
    }
}
