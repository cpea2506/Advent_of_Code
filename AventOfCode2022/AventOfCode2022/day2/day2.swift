//
// Created by Do Vien on 24/12/2022 at 20:57.
//
// Copyright © 2022 CPea2506. All rights reserved.
//

// MARK: - Day2

struct Day2 {
    var calories: [UInt] = []
}

// MARK: Avent

extension Day2: Avent {
    // MARK: Lifecycle

    var day: UInt8 { 1 }

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
        guard let max = calories.max() else {
            return 0
        }

        return max
    }

    func part2() -> UInt {
        0
    }
}
