//
// Created by Logan Keenan on 3/14/21.
//

import Foundation

public class AppContext: Codable {
    var local_files_path: String
    var assets_path: String
    var database_path: String
    var views_path: String

    init(database_path: String,
         local_files_path: String,
         assets_path: String,
         views_path: String) {
        self.database_path = database_path
        self.local_files_path = local_files_path
        self.assets_path = assets_path
        self.views_path = views_path
    }
    func asJson() -> String {
        let appRequestData = try! JSONEncoder().encode(self)
        return String(data: appRequestData, encoding: .utf8)!
    }
}