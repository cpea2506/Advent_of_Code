//
// Created by Do Vien on 29/12/2022 at 17:15.
//
// Copyright © 2022 CPea2506. All rights reserved.
//

import Foundation

extension Double {
    func rounded(to decimal: Int) -> Self {
        let power = pow(10, Double(decimal))

        return Darwin.round(self * power) / power
    }
}

/// Get the time to compute solution
func getTime<T>(for solution: () -> T) -> (T, Double) {
    let clock = ContinuousClock()
    var result: T?
    let time = clock.measure {
        result = solution()
    }

    let timeElapsedInMs = Double(time.components.attoseconds) * 0.000000000000001

    // SAFETY: result always has value when clock done measuring
    return (result!, timeElapsedInMs.rounded(to: 3))
}

/// Read main file corressponding to day
func readFileInDay(_ day: UInt8, for mainFile: String) throws -> String {
    // temporary hack to get true file path to this project
    let url = URL(fileURLWithPath: #filePath)
        .deletingLastPathComponent()
        .appending(component: "day\(day)/\(mainFile)")

    return try String(contentsOf: url)
}
