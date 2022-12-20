import ArgumentParser

struct Solution: ParsableCommand {
    public static let configuration = CommandConfiguration(abstract: "Avent of Code 2022")

    @Argument(help: "AOC Day")
    private var day: Int8

    @Argument(help: "Uses example file provided by AOC")
    private var example: Bool

    @Argument(help: "Gets all solutions for all AOC days")
    private var all: Bool

    func run() throws {}

    /// Get the time to compute solution
    @available(macOS 13, *)
    func getTime<T>(for solution: () -> T) -> (T?, Int64) {
        let clock = ContinuousClock()
        var result: T?
        let time = clock.measure {
            result = solution()
        }

        return (result, time.components.seconds)
    }
}

@main
public enum AventOfCode2022 {
    public static func main() {
        Solution.main()
    }
}
