//
// Created by Do Vien on 24/12/2022 at 20:57.
//
// Copyright © 2022 CPea2506. All rights reserved.
//

// MARK: - Day2

struct Day2 {
    private var scores: [RPSTuple] = []
    var day: UInt8 = 2
}

// MARK: Avent

extension Day2: Avent {
    // MARK: Lifecycle

    init(data: String) {
        scores = data
            .lines()
            .map { round in
                let components = round.components(separatedBy: " ")

                return (opponent: Shape(fromValue: components[0]), unknown: components[1])
            }
    }

    // MARK: Internal

    func part1() -> UInt {
        scores.reduce(0) {
            let you = Shape(fromValue: $1.unknown)
            let outcome = $1.opponent.compare(to: you)

            return $0 + you.rawValue + outcome.rawValue
        }
    }

    func part2() -> UInt {
        scores.reduce(0) {
            let outcome = Outcome(fromValue: $1.unknown)
            let you = outcome.getShape(against: $1.opponent)

            return $0 + you.rawValue + outcome.rawValue
        }
    }
}
