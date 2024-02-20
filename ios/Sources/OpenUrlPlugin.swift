import SwiftRs
import Tauri
import UIKit
import WebKit


class PingArgs: Decodable {
  let value: String?
}

class OpenUrlArgs: Decodable {
  let url: String
}

class OpenUrlPlugin: Plugin {
  @objc public func openUrl(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(OpenUrlArgs.self)
    if let url = URL(string: args.url) {
      UIApplication.shared.open(url)
      invoke.resolve(nil)
    } else {
      invoke.reject("Invalid URL")
    }
}

@_cdecl("init_plugin_openurl")
func initPlugin() -> Plugin {
  return OpenUrlPlugin()
}
