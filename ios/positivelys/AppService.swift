//
// Created by Logan Keenan on 11/24/20.
//

import Foundation

public class AppService {

    public static var hostName: String = "https://positivelys.com";

    func appContext() -> String {
        let database_path: String = NSSearchPathForDirectoriesInDomains(.documentDirectory, .userDomainMask, true)[0] + "/database.sqlite"
        let local_files_path: String = NSSearchPathForDirectoriesInDomains(.documentDirectory, .userDomainMask, true)[0];

        let appContext = "{\"database_path\":\"" + database_path + "\", \"assets_path\": \"" + Bundle.main.bundlePath   + "\", \"local_files_path\":\"" + local_files_path + "\"}"
        return appContext
    }

    public func make_request(appRequest: AppRequest) -> AppResponse {
        let appRequestData = try! JSONEncoder().encode(appRequest)
        let appRequestAsJson = String(data: appRequestData, encoding: .utf8)

        let result = make_app_request(appRequestAsJson, appContext())
        let app_response_as_json = String(cString: result!)

        make_app_request_free(UnsafeMutablePointer(mutating: result))

        let response: AppResponse? = try? JSONDecoder().decode(AppResponse.self, from: app_response_as_json.data(using: .utf8)!)

        return response!;
    }

}