//
// Created by Do Vien on 06/01/2023 at 11:41.
//
// Copyright © 2023 CPea2506. All rights reserved.
//

typealias Compartments = (firstHalf: Substring, secondHalf: Substring)

extension Character {
    var priority: UInt8 {
        switch self {
        case "a"..."z":
            return asciiValue! - 96 // priority 1...26
        case "A"..."Z":
            return asciiValue! - 38 // priority 27..52
        default:
            fatalError("Unreachable!")
        }
    }
}

extension Substring {
    /// Return number of element's occurrences in the current substring
    var elementOccurrence: [Element: UInt] {
        reduce(into: [:]) {
            $0[$1, default: 0] += 1
        }
    }

    /// Return set of elements in the current substring
    var charSet: Set<Element> {
        reduce(into: Set()) { $0.insert($1) }
    }
}
