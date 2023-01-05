//
// Created by Do Vien on 04/01/2023 at 01:30.
// Copyright © 2023 CPea2506. All rights reserved.
//

// MARK: - Outcome

typealias RPSTuple = (opponent: Shape, unknown: String)

extension UInt {
    //  See https://math.stackexchange.com/questions/2678895/function-which-creates-the-sequence-1-2-3-1-2-3
    /// Create a cycling sequence from 1 to specified length
    ///
    /// Example of cycling sequence: 1, 2, 3, 1, 2, 3...
    func cycling(length: Self) -> Self {
        (self + length - 1) % length + 1
    }
}

// MARK: - Outcome

enum Outcome: UInt {
    case win = 6
    case lost = 0
    case draw = 3

    // MARK: Lifecycle

    /// Creates a new instance with specified **String** value
    init(fromValue value: String) {
        switch value {
        case "X":
            self = .lost
        case "Y":
            self = .draw
        default:
            self = .win
        }
    }

    // MARK: Internal

    /// Return the shape that lead to outcome from other shape
    func getShape(against shape: Shape) -> Shape {
        switch self {
        case .lost:
            return Shape(rawValue: (shape.rawValue - 1).cycling(length: 3))!
        case .win:
            return Shape(rawValue: (shape.rawValue + 1).cycling(length: 3))!
        case .draw:
            return shape
        }
    }
}

// MARK: - Shape

enum Shape: UInt, Comparable {
    case rock = 1
    case paper = 2
    case scissors = 3

    // MARK: Lifecycle

    /// Creates a new instance with specified **String** value
    init(fromValue value: String) {
        switch value {
        case "A",
             "X":
            self = .rock
        case "B",
             "Y":
            self = .paper
        default:
            self = .scissors
        }
    }

    // MARK: Public

    public static func < (lhs: Self, rhs: Self) -> Bool {
        switch (lhs, rhs) {
        case (.rock, .scissors):
            return false
        case (.scissors, .rock):
            return true
        default:
            return lhs.rawValue < rhs.rawValue
        }
    }

    // MARK: Internal

    func compare(to shape: Shape) -> Outcome {
        switch self {
        case shape:
            return .draw
        case ...shape:
            return .win
        default:
            return .lost
        }
    }
}
