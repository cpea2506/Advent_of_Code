//
// Created by Do Vien on 09/01/2023 at 00:26.
//
// Copyright © 2023 CPea2506. All rights reserved.
//

// MARK: - Day4

struct Day4 {
    private var assignmentPairs: [AssignmentPair] = []
    var day: UInt8 = 4
}

// MARK: Avent

extension Day4: Avent {
    typealias Result = UInt

    init(data: String) {
        assignmentPairs = data.lines().map {
            let assignments = $0.split(separator: ",").map {
                let range = $0.split(separator: "-").compactMap { Result($0) }

                return Assignment(start: range[0], end: range[1])
            }

            return AssignmentPair(assignments[0], assignments[1])
        }
    }

    // MARK: Internal

    func part1() -> Result {
        UInt(assignmentPairs.filter { $0.fullyContained }.count)
    }

    func part2() -> Result {
        UInt(assignmentPairs.filter { $0.pairsOverlapped }.count)
    }
}
