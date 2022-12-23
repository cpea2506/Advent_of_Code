//
// Created by Do Vien on 24/12/2022 at 05:11.
//
// Copyright © 2022 CPea2506. All rights reserved.
//

import ArgumentParser
import Foundation

// MARK: - Avent

protocol Avent {
    static var day: UInt8 { get }
    init(data: String)
    func part1() -> UInt
    func part2() -> UInt
}

// MARK: - Solution

struct Solution<Event: Avent> {
    // MARK: Lifecycle

    init(data: String) {
        (event, time) = getTime { Event(data: data) }
        day = Event.day
    }

    // MARK: Internal

    var event: Event?
    var time: Int64?
    var day: UInt8?

    func getResult() {
        let (time1, part1) = getTime(for: event!.part1)
        let (time2, part2) = getTime(for: event!.part2)

        print("""
        Solution for day \(day!)
        Collect data in \u{001B}[0;36m\(time!)
        part 1: \(part1) in \u{001B}[0;36m\(time1)
        part 2: \(part2) in \u{001B}[0;36m\(time2)
        """)
    }

    /// Get the time to compute solution
    func getTime<T>(for solution: () -> T) -> (T, Int64) {
        let clock = ContinuousClock()
        var result: T?
        let time = clock.measure {
            result = solution()
        }

        return (result!, time.components.seconds)
    }
}

// MARK: - AOC2022

struct AOC2022: ParsableCommand {
    // MARK: Public

    public static let configuration = CommandConfiguration(abstract: "Avent of Code 2022")

    // MARK: Internal

    func run() throws {
        let mainFile = example ? "example" : "input"
        let filePath = "day\(day)/\(mainFile).txt"
        do {
            let data = try String(contentsOfFile: filePath)
            var solution = Solution<Day1>(data: data)

            switch day {
            case 1:
                solution = Solution<Day1>(data: data)
            default:
                print("not yet implemented")
            }

            solution.getResult()
        } catch {
            print("Error loading file: 🌶️ \(error)")
        }
    }

    // MARK: Private

    @Argument(help: "AOC Day")
    private var day: UInt8

    @Flag(name: .shortAndLong, help: "Uses example file provided by AOC")
    private var example: Bool = false

    @Flag(name: .shortAndLong, help: "Gets all solutions for all AOC days")
    private var all: Bool = false
}

// MARK: - AventOfCode2022

AOC2022.main()
