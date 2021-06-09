//
// Created by Logan Keenan on 11/24/20.
//

import Foundation

public class AppRequest: Codable {
    var uri: String = ""
    var body: String?
    var headers: Dictionary<String, String>?
    var method: String

    init(uri: String, method: String) {
        self.uri = uri
        self.method = method.uppercased()
    }
}