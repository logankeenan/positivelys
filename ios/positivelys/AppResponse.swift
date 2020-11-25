//
// Created by Logan Keenan on 10/31/20.
//

import Foundation

public class AppResponse: Codable {
    var body: String?
    var headers: Dictionary<String, String>?
    var status_code: Int?

    enum CodingKeys: String, CodingKey {
        case body, headers
        case status_code
    }
}