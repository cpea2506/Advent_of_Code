//
// Created by Do Vien on 09/01/2023 at 00:26.
//
// Copyright © 2023 CPea2506. All rights reserved.
//

extension CountableClosedRange {
    init(start: Bound, end: Bound) {
        self.init(uncheckedBounds: (start, end))
    }
}

typealias Assignment = CountableClosedRange<UInt>

// MARK: - AssignmentPair

struct AssignmentPair {
    // MARK: Lifecycle

    init(_ firstElve: Assignment, _ secondElve: Assignment) {
        self.firstElve = firstElve
        self.secondElve = secondElve
    }

    // MARK: Internal

    let firstElve: Assignment
    let secondElve: Assignment

    // check if one elve's assignment fully contains the other
    var fullyContained: Bool {
        firstElve.lowerBound <= secondElve.lowerBound &&
            firstElve.upperBound >= secondElve.upperBound ||
            firstElve.lowerBound >= secondElve.lowerBound &&
            firstElve.upperBound <= secondElve.upperBound
    }

    var pairsOverlapped: Bool {
        firstElve.overlaps(secondElve)
    }
}
