import ArgumentParser
import Foundation

protocol Avent {
    static var day: UInt8 { get }
    init(data: String)
    func part1() -> UInt
    func part2() -> UInt
}

@available(macOS 13, *)
struct Solution<Event: Avent> {
    var event: Event?
    var time: Int64?
    var day: UInt8?

    init(data: String) {
        (event, time) = getTime { Event(data: data) }
        day = Event.day
    }

    @available(macOS 13, *)
    func getResult() {
        let (time1, part1) = getTime(for: event!.part1)
        let (time2, part2) = getTime(for: event!.part2)

        print("""
        Solution for day \(day!)
        Collect data in \(time!)
        part 1: \(part1) in \(time1)
        part 2: \(part2) in \(time2)
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

struct AOC2022: ParsableCommand {
    public static let configuration = CommandConfiguration(abstract: "Avent of Code 2022")

    @Argument(help: "AOC Day")
    private var day: UInt8

    @Flag(name: .shortAndLong, help: "Uses example file provided by AOC")
    private var example: Bool = false

    @Flag(name: .shortAndLong, help: "Gets all solutions for all AOC days")
    private var all: Bool = false

    @available(macOS 13, *)
    func run() throws {
        let mainFile = example ? "example" : "input"
        let filePath = "./day\(day)/\(mainFile).txt"
        let url = URL(filePath: filePath)
        let data = try! String(contentsOf: url)

        var solution = Solution<Day1>(data: data)

        switch day {
        case 1:
            solution = Solution<Day1>(data: data)
        default:
            print("not yet implemented")
        }

        solution.getResult()
    }
}

@main
public enum AventOfCode2022 {
    public static func main() {
        AOC2022.main()
    }
}
