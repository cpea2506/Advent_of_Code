//
// Created by Do Vien on 29/12/2022 at 17:15.
//
// Copyright © 2022 CPea2506. All rights reserved.
//

import Foundation

extension String {
    func lines(omittingEmptySubsequences: Bool = true) -> [SubSequence] {
        split(omittingEmptySubsequences: omittingEmptySubsequences, whereSeparator: \.isNewline)
    }
}

extension Double {
    func rounded(to decimal: Int) -> Self {
        let power = pow(10, Double(decimal))

        return Darwin.round(self * power) / power
    }
}

/// Get the time to compute solution
func getTime<T>(for solution: () -> T) -> (T, Double) {
    let attosecondPerMs = 0.000000000000001
    let clock = ContinuousClock()
    var result: T?
    let time = clock.measure {
        result = solution()
    }

    let timeElapsedInMs = Double(time.components.attoseconds) * attosecondPerMs

    // SAFETY: result always has value when clock done measuring
    return (result!, timeElapsedInMs.rounded(to: 3))
}

/// Read main file corressponding to day
func readFile(_ file: String, byDay day: UInt8) throws -> String {
    let url = URL(fileURLWithPath: #file)
        .deletingLastPathComponent()
        .appending(component: "day\(day)/\(file)")

    return try String(contentsOf: url)
}
