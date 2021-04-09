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
