import Cocoa

func log(_ data: Data) {
    if let message = NSString(data: data, encoding: String.Encoding.utf8.rawValue) {
        NSLog("%@", message)
    }
}

let task = Process()
let stdOut = Pipe()
let stdErr = Pipe()
let bundle = Bundle.main
let path = bundle.resourcePath!
let rustBinName = bundle.infoDictionary?["RustBinaryName"] as! String

task.launchPath = bundle.path(forResource: rustBinName, ofType: nil)
task.environment = [
    "RUST_BACKTRACE": "1",
    "DYLD_FALLBACK_LIBRARY_PATH": "\(path)/lib",
]

if !FileManager.default.fileExists(atPath: "/usr/local/opt/gdk-pixbuf/") {
    let pixbufFileName = "\(path)/lib/pixbux-loaders.cache"

    let contents = try! String(contentsOfFile: pixbufFileName)
    let lines = contents.split(separator: "\n", omittingEmptySubsequences: false).map({ (line) -> Substring in
        if line.starts(with: "#") {
            return Substring("")
        }
        if !line.contains(".so\"") {
            return line
        }
        if let last = line.split(separator: "/").last {
            return Substring("\"\(path)/lib/\(last)")
        }
        return line
    })

    let result = lines.joined(separator: "\n")
    try result.write(toFile: pixbufFileName, atomically: true, encoding: .utf8)

    task.environment?["GDK_PIXBUF_MODULE_FILE"] = pixbufFileName
}

stdOut.fileHandleForReading.readabilityHandler = { log($0.availableData) }
stdErr.fileHandleForReading.readabilityHandler = { log($0.availableData) }

task.standardOutput = stdOut
task.standardError = stdErr

task.terminationHandler = { task in
    (task.standardOutput as AnyObject?)?.fileHandleForReading.readabilityHandler = nil
    (task.standardError as AnyObject?)?.fileHandleForReading.readabilityHandler = nil
}

task.launch()
task.waitUntilExit()
