//
// Created by Do Vien on 30/12/2022 at 10:23.
//
// Copyright © 2022 CPea2506. All rights reserved.
//
import Foundation

// See: https://github.com/apple/swift-algorithms/blob/3864606/Sources/Algorithms/SortedPrefix.swift
extension Array {
    ///
    /// Returns the k nearest elements of this collection using
    /// the given predicate as the comparison between elements.
    /// - Parameter count: The k number of elements.
    /// - Parameter direction: A predicate that returns true if its
    /// first argument should be ordered before its second argument;
    /// otherwise, false.
    ///
    func kNearest(_ count: Int, by direction: (Element, Element) throws -> Bool) rethrows -> Self {
        guard count > 0 else {
            return []
        }

        guard count < self.count else {
            return self
        }

        // get first count elements in sorted order
        var result = try prefix(count).sorted(by: direction)

        for num in dropFirst(count) {
            if let last = result.last, try direction(last, num) {
                continue
            }

            let insertionIndex = try result.partition { try direction(num, $0) }

            result.removeLast()
            result.insert(num, at: insertionIndex)
        }

        return result
    }

    @inlinable
    func partition(where belongsToSecondPartition: (Element) throws -> Bool) rethrows -> Index {
        var count = self.count
        var low = startIndex

        while count > 0 {
            let half = count / 2
            let mid = index(low, offsetBy: half)

            if try belongsToSecondPartition(self[mid]) {
                count = half
            } else {
                low = index(after: mid)
                count -= half + 1
            }
        }

        return low
    }
}
